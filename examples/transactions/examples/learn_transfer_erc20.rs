#![allow(missing_docs)]

use alloy::{
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    sol,
};
use eyre::Result;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20Example,
    "examples/artifacts/ERC20Example.json"
);

#[tokio::main]
async fn main() -> Result<()> {

    let rpc_url = "https://eth.merkle.io";
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url));

    let accounts = provider.get_accounts().await?;
    let alice = accounts[0];
    let bob = accounts[1];

    let contract = ERC20Example::deploy(provider).await?;

    let alice_before_balance = contract.balanceOf(alice).call().await?._0;
    let bob_before_balance = contract.balanceOf(bob).call().await?._0;

    let amount = U256::from(100);
    let tx_hash = contract.transfer(bob, amount).send().await?.watch().await?;

    println!("Sent transaction: {tx_hash}");

    let alice_after_balance = contract.balanceOf(alice).call().await?._0;
    let bob_after_balance = contract.balanceOf(bob).call().await?._0;

    assert_eq!(alice_before_balance - alice_after_balance, amount);
    assert_eq!(bob_after_balance - bob_before_balance, amount);

    Ok(())
}