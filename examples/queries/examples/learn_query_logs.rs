#![allow(missing_docs)]

use alloy::{
    primitives::address,
    providers::{Provider, ProviderBuilder},
    rpc::types::Filter,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let latest_block = provider.get_block_number().await?;

    let filter = Filter::new().from_block(latest_block);

    let logs = provider.get_logs(&filter).await?;
    for log in logs {
        println!("[AAAAAAAAAA] {log:?}");
    }

    let filter = Filter::new().event("Transfer(address,address,uint256)").from_block(latest_block);

    let logs = provider.get_logs(&filter).await?;

    for log in logs {
        println!("[BBBBBBBBBB] Transfer event: {log:?}");
    }

    let weth_token_address = address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
    let filter = Filter::new().address(weth_token_address).from_block(latest_block);

    let logs = provider.get_logs(&filter).await?;

    for log in logs {
        println!("[CCCCCCCCCC] WETH token logs: {log:?}");
    }

    Ok(())
}
