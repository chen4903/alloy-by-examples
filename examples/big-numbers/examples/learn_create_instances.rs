#![allow(missing_docs)]

use alloy::primitives::utils::parse_units;
use eyre::Result;

fn main() -> Result<()>{
    let amount = "42";
    let units = 4;
    let b = parse_units(amount, units)?;
    assert_eq!(b.to_string(), "420000");

    Ok(())
}