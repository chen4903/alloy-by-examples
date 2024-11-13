#![allow(missing_docs)]

use alloy::{
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    sol,
};
use eyre::Result;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    SimpleStorage,
    "examples/artifacts/SimpleStorage.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let provider = ProviderBuilder::new().with_recommended_fillers().on_anvil();

    // Create two users, Alice and Bob.
    let accounts = provider.get_accounts().await?;
    let alice = accounts[0];
    let bob = accounts[1];

    // Deploy the `SimpleStorage` contract.
    let contract_address =
        SimpleStorage::deploy_builder(provider.clone(), "I am constructor()".to_string())
            .from(alice)
            .deploy()
            .await?;
    let contract = SimpleStorage::new(contract_address, provider.clone());

    // Build a transaction to set the values of the contract.
    // The `from` field is automatically filled to the first signer's address (Alice).
    let set_value_call = contract.setValues("hello".to_string(), "world".to_string());
    let calldata = set_value_call.calldata().to_owned();
    let tx = TransactionRequest::default().from(bob).to(contract_address).input(calldata.into());

    // access_list 是以太坊在 EIP-2930 中引入的一种优化技术，用于降低合约调用的Gas成本，特别是在访问多个合约地址或大量存储槽时。

    // access_list 的工作原理
    // 在以太坊中，每次合约调用或数据读取都会消耗Gas。access_list 提供了一种机制，提前指定当前交易所需要访问的存储槽和合约地址，
    // 使得这些位置的Gas费用可以降低。这样可以避免重复访问带来的额外Gas消耗，从而提升交易的效率。

    // Create an access list for the transaction.
    let access_list_with_gas_used = provider.create_access_list(&tx).await?;

    // Add the access list to the transaction.
    let tx_with_access_list = tx.access_list(access_list_with_gas_used.access_list);

    // Send the transaction with the access list.
    let tx_hash = provider.send_transaction(tx_with_access_list).await?.watch().await?;

    println!("Transaction hash: {tx_hash}");

    // Check the value of the contract.
    let value = contract.getValue().call().await?._0;

    assert_eq!(value, "hello");

    Ok(())
}