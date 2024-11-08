#![allow(missing_docs)]

use alloy::signers::{local::PrivateKeySigner, Signer};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let signer = PrivateKeySigner::random();

    let signer = signer.with_chain_id(Some(1337));

    let message = b"hello";

    let signature = signer.sign_message(message).await?;

    println!("Signature produced by {}: {:?}", signer.address(), signature);
    println!("Signature recovered address: {}", signature.recover_address_from_msg(&message[..])?);

    Ok(())
}
