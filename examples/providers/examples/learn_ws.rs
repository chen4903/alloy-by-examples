#![allow(missing_docs)]

use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use eyre::{Ok, Result};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "wss://ethereum-rpc.publicnode.com";
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    let sub = provider.subscribe_blocks().await?;

    // Wait and take the next 4 blocks.
    let mut stream = sub.into_stream().take(4);

    println!("Awaiting blocks...");

    // Take the stream and print the block number upon receiving a new block.
    let handle = tokio::spawn(async move {
        while let Some(block) = stream.next().await {
            println!("Latest block number: {}", block.header.number);
        }
    });

    handle.await?;

    Ok(())
}
