#![allow(missing_docs)]

use alloy::{
    consensus::{SidecarBuilder, SimpleCoder},
    eips::eip4844::DATA_GAS_PER_BLOB,
    network::{TransactionBuilder, TransactionBuilder4844},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let provider =
        ProviderBuilder::new().on_anvil_with_config(|anvil| anvil.args(["--hardfork", "cancun"]));

    let accounts = provider.get_accounts().await?;
    let alice = accounts[0];
    let bob = accounts[1];

    let sidecar: SidecarBuilder<SimpleCoder> = SidecarBuilder::from_slice(b"Blobs are fun!");
    let sidecar = sidecar.build()?;

    let gas_price = provider.get_gas_price().await?;
    let eip1559_est = provider.estimate_eip1559_fees(None).await?;
    let tx = TransactionRequest::default()
        .with_to(bob)
        .with_nonce(0)
        .with_max_fee_per_blob_gas(gas_price)
        .with_max_fee_per_gas(eip1559_est.max_fee_per_gas)
        .with_max_priority_fee_per_gas(eip1559_est.max_priority_fee_per_gas)
        .with_blob_sidecar(sidecar);

    let pending_tx = provider.send_transaction(tx).await?;

    println!("Pending transaction... {}", pending_tx.tx_hash());

    let receipt = pending_tx.get_receipt().await?;

    println!(
        "Transaction included in block {}",
        receipt.block_number.expect("Failed to get block number")
    );

    assert_eq!(receipt.from, alice);
    assert_eq!(receipt.to, Some(bob));
    assert_eq!(
        receipt.blob_gas_used.expect("Expected to be EIP-4844 transaction"),
        DATA_GAS_PER_BLOB as u128
    );

    println!("DATA_GAS_PER_BLOB: {}", DATA_GAS_PER_BLOB);

    Ok(())
}
