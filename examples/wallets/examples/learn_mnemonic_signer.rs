#![allow(missing_docs)]

use alloy::signers::local::{coins_bip39::English, MnemonicBuilder};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let phrase = "work man father plunge mystery proud hollow address reunion sauce theory bonus";
    let index = 0u32;
    let password = "TREZOR123";

    let wallet = MnemonicBuilder::<English>::default()
    .phrase(phrase)
    .index(index)?
    .password(password)
    .build()?;

    println!("Wallet: {}", wallet.address());

    let wallet = MnemonicBuilder::<English>::default()
    .word_count(24)
    .derivation_path("m/44'/60'/0'/2/1")?
    .build_random()?;

    println!("Random wallet: {}", wallet.address());

    Ok(())
}
