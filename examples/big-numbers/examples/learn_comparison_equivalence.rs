#![allow(missing_docs)]
use alloy::primitives::U256;

fn main() {
    let a = U256::from(100_u32);
    let b = U256::from(200_u64);
    assert!(a < b);
}