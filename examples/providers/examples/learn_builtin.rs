#![allow(missing_docs)]

use alloy::{
    node_bindings::Anvil,
    providers::{Provider, ProviderBuilder},
};
use eyre::{Ok, Result};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().block_time(1).try_spawn()?;

    let http_rpc_url = anvil.endpoint();
    let http_provider = ProviderBuilder::new().on_builtin(&http_rpc_url).await?;

    let block_number = http_provider.get_block_number().await?;

    println!("Last block number: {block_number:?}");

    let ws_rpc_url = anvil.ws_endpoint();
    let ws_provider = ProviderBuilder::new().on_builtin(&ws_rpc_url).await?;

    let sub = ws_provider.subscribe_blocks().await?;

    let mut stream = sub.into_stream().take(3);
    
    println!("Awaiting blocks...");

    let handle = tokio::spawn(async move {
        while let Some(block) = stream.next().await {
            println!("{}", block.header.number);
        }
    });

    handle.await?;

    Ok(())
}