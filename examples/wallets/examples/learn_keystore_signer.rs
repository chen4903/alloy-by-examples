#![allow(missing_docs)]

use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::LocalSigner,
};
use eyre::{Ok, Result};
use std::{env, path::PathBuf};

#[tokio::main]
async fn main() -> Result<()> {
    let password = "test";

    let keystore_file_path =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("examples/keystore/alice.json");
    let signer = LocalSigner::decrypt_keystore(keystore_file_path, password)?;
    let wallet = EthereumWallet::from(signer);

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_anvil_with_config(|anvil| anvil.block_time(1));

    let vitalik = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let tx = TransactionRequest::default().with_to(vitalik).with_value(U256::from(100));
    let tx_hash = provider.send_transaction(tx).await?.watch().await?;

    println!("Sent TX: {tx_hash}");

    Ok(())
}
