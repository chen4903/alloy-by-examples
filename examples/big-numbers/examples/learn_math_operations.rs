#![allow(missing_docs)]

use alloy::primitives::{utils::format_units, U256};
use eyre::Ok;
use std::ops::{Div, Mul};
use eyre::Result;

fn main() -> Result<()>{
    let a = U256::from(1);
    let b = U256::from(2);
    assert_eq!(a + b, U256::from(3));

    assert_eq!(a.pow(b), U256::from(1));

    let eth1 = U256::from(10_000000000000000000_u128);
    let eth2 = U256::from(20_000000000000000000_u128);
    let base = U256::from(10).pow(U256::from(18));
    let mul = eth1.mul(eth2).div(base);
    let s = format_units(mul, "ether")?;
    assert_eq!(s, "200.000000000000000000"); // 200

    Ok(())
}