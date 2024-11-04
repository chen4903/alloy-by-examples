#![allow(missing_docs)]

use alloy::{
    eips::BlockId, primitives::address, providers::{Provider, ProviderBuilder}
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a provider.
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // Get the bytecode of the Uniswap V3 USDC-ETH pool on Ethereum mainnet.
    let pool_address = address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640");
    let bytecode = provider.get_code_at(pool_address).block_id(BlockId::from(21113028)).await?;

    println!("Bytecode: {bytecode:?}");

    Ok(())
}
