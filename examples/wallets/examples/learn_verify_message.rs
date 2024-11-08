#![allow(missing_docs)]

use alloy::signers::{local::PrivateKeySigner, SignerSync};
use eyre::{Ok, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let signer = PrivateKeySigner::random();

    let message = "Some data";
    let signature = signer.sign_message_sync(message.as_bytes())?;

    let recovered = signature.recover_address_from_msg(message)?;
    assert_eq!(recovered, signer.address());

    Ok(())
}
