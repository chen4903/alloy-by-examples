#![allow(missing_docs)]

use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    node_bindings::Anvil,
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().block_time(1).try_spawn()?;

    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let wallet = EthereumWallet::from(signer);

    let rpc_url = anvil.endpoint_url();
    let provider =
        ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(rpc_url);

    let tx = TransactionRequest::default()
        .with_to(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"))
        .with_value(U256::from(100));

    let tx_hash = provider.send_transaction(tx).await?.watch().await?;

    println!("Sent transaction: {tx_hash}");

    Ok(())
}
