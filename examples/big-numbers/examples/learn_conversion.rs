#![allow(missing_docs)]

use alloy::primitives::{utils::format_units, U256};
use eyre::Result;

fn main() -> Result<()> {
    let num = U256::from(42_u8);

    let a: u128 = num.to::<u128>();
    assert_eq!(a, 42);

    let b = num.to::<u64>();
    assert_eq!(b, 42);

    let c = num.to_string();
    assert_eq!(c, "42");

    let d = format_units(num, 4)?;
    assert_eq!(d, "0.0042");

    let e = match format_units(num, 4) {
        Ok(val) => val,
        Err(e) => return Err(e.into()), // 将 `UnitsError` 转换为 `eyre::Report`
        // 在 Rust 中，eyre::Report 是一种通用的错误类型，用于包装和处理各种不同的错误类型，因此非常适合用于需要兼容多种错误的场景
    };
    assert_eq!(e, "0.0042");

    // let eth = U256::from_str_radix("1395633240123456000", 10).unwrap();
    let f_1 = U256::from_str_radix("1395633240123456000", 10)?;
    let f_2 = format_units(f_1, "eth")?;
    assert_eq!(f_2, "1.395633240123456000");

    Ok(())
}
