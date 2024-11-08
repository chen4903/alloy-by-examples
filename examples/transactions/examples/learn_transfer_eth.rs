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
    let provider = ProviderBuilder::new().with_recommended_fillers().on_anvil_with_wallet();

    let accounts = provider.get_accounts().await?;
    let alice = accounts[0];
    let bob = accounts[1];

    let tx =
        TransactionRequest::default().with_from(alice).with_to(bob).with_value(U256::from(100));

    let tx_hash = provider.send_transaction(tx).await?.watch().await?;

    println!("Sent transaction: {tx_hash}");

    Ok(())
}