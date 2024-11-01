#![allow(missing_docs)]

use alloy::{primitives::address, providers::ProviderBuilder, sol};
use eyre::Result;

sol!(
    #[sol(rpc)]
    IWETH9,
    "examples/abi/IWETH9.json"
);

#[tokio::main]
async fn main() -> Result<()> {

    let rpc_url = "https://eth.merkle.io";
    let provider = ProviderBuilder::new().on_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url));

    let contract = IWETH9::new(address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"), provider);

    let total_supply = contract.totalSupply().call().await?._0;

    println!("WETH total supply is {total_supply}");

    Ok(())
}