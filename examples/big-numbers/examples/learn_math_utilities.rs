#![allow(missing_docs)]

use alloy::primitives::utils::format_units;
use alloy::primitives::{utils::parse_units, U256};
use eyre::Ok;
use eyre::Result;

fn main() -> Result<()>{
    // string => U256
    let a = parse_units("1.0", "wei")?;
    let b: U256 = a.into();
    assert_eq!(b, U256::from(1));

    // U256 => string
    let one_ether = U256::from(1000000000000000000_u128);
    let c = format_units(one_ether, "ether")?;
    assert_eq!(c, "1.000000000000000000");

    Ok(())
}