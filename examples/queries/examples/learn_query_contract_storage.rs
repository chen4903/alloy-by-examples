#![allow(missing_docs)]

use alloy::{
    eips::BlockId, primitives::{address, U256}, providers::{Provider, ProviderBuilder}
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let pool_address = address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640");
    let storage_slot = U256::from(0);

    let storage = provider.get_storage_at(pool_address, storage_slot).block_id(BlockId::from(21113028)).await?;

    println!("Slot 0: 0x{:x}", storage);

    Ok(())
}
