#![allow(missing_docs)]

use alloy::{primitives::U256, providers::ProviderBuilder, sol};
use eyre::Result;

sol!(
    #[sol(rpc)]
    Counter,
    "examples/artifacts/Counter.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    let provider = ProviderBuilder::new().with_recommended_fillers().on_anvil_with_wallet();
    let contract = Counter::deploy(&provider).await?;

    println!("Deployed contract at address: {}", contract.address());

    let builder = contract.setNumber(U256::from(42));
    let tx_hash = builder.send().await?.watch().await?;
    println!("Set number to 42: {tx_hash}");

    let builder = contract.increment();
    let tx_hash = builder.send().await?.watch().await?;
    println!("Incremented number: {tx_hash}");

    let builder = contract.number();

    let number = builder.call().await?._0;
    println!("Retrieved number: {number}");

    Ok(())
}