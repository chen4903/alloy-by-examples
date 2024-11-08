#![allow(missing_docs)]

use alloy::{
    network::{eip2718::Encodable2718, EthereumWallet, TransactionBuilder},
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let flashbots_url = "https://rpc.flashbots.net".parse()?;
    let provider = ProviderBuilder::new().on_http(flashbots_url);

    let signer = PrivateKeySigner::random();
    let wallet = EthereumWallet::from(signer);

    let bob = PrivateKeySigner::random().address();
    let tx = TransactionRequest::default()
        .with_to(bob)
        .with_nonce(0)
        .with_chain_id(1)
        .with_value(U256::from(100))
        .with_gas_limit(21_000)
        .with_max_priority_fee_per_gas(1_000_000_000)
        .with_max_fee_per_gas(20_000_000_000);

    let tx_envelope = tx.build(&wallet).await?;
    let tx_encoded = tx_envelope.encoded_2718();

    let pending = provider.send_raw_transaction(&tx_encoded).await?.register().await?;

    println!("Sent transaction: {}", pending.tx_hash());

    Ok(())
}
