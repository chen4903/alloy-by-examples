#![allow(missing_docs)]

use alloy::{
    network::TransactionBuilder,
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = ProviderBuilder::new().on_anvil();

    let accounts = provider.get_accounts().await?;
    let alice = accounts[0];
    let bob = accounts[1];

    let tx = TransactionRequest::default()
        .with_to(bob)
        .with_nonce(0)
        .with_value(U256::from(100))
        .with_gas_price(20_000_000_000)
        .with_gas_limit(21_000);

    let pending_tx = provider.send_transaction(tx).await?;

    println!("Pending transaction... {}", pending_tx.tx_hash());

    let receipt = pending_tx.get_receipt().await?;

    println!(
        "Transaction included in block {}",
        receipt.block_number.expect("Failed to get block number")
    );

    assert_eq!(receipt.from, alice);
    assert_eq!(receipt.to, Some(bob));

    Ok(())
}
