#![allow(missing_docs)]

use alloy::{
    network::EthereumWallet,
    node_bindings::Anvil,
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::{Ok, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().block_time(1).try_spawn()?;

    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let waller = EthereumWallet::from(signer.clone());

    let alice = signer.address();
    assert_eq!(alice, anvil.addresses()[0]);
    let bob = anvil.addresses()[1];

    let rpc_url = anvil.endpoint_url();
    println!("rpc_url: {}", rpc_url);

    let provider = ProviderBuilder::new().with_recommended_fillers().wallet(waller).on_http(rpc_url);

    let tx = TransactionRequest::default().to(bob).value(U256::from(100));

    let pending_tx = provider.send_transaction(tx).await?;

    println!("Pending TX..., {}", pending_tx.tx_hash());
    
    let receipt = pending_tx.get_receipt().await?;

    println!(
        "Transaction included in block {}",
        receipt.block_number.expect("Failed to get block number")
    );

    assert_eq!(receipt.from, alice);
    assert_eq!(receipt.to, Some(bob));

    Ok(())
}