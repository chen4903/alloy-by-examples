#![allow(missing_docs)]

use alloy::primitives::{eip191_hash_message, keccak256};
use eyre::Result;

fn main() -> Result<()>{
    let hash = keccak256(b"hello world");
    assert_eq!(
        hash.to_string(),
        "0x47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad"
    );
    assert_eq!(hash.len(), 32);

    let eip191_hash = eip191_hash_message(b"hello_world");
    // the same as: keccak256(b"\x19Ethereum Signed Message:\n11hello_world")
    assert_eq!(
        eip191_hash.to_string(),
        "0xd52de6e039c023a7c77752126e4d9d99e2a7dacea3d19e97e9c2ebcb3ecf1c00"
    );
    assert_eq!(eip191_hash.len(), 32);

    Ok(())
}