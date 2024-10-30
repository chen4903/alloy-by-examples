//! Example of comparison and equivalence of `U256` instances.

use alloy::primitives::U256;

/// `U256` implements traits in `std::cmp`, that means `U256` instances
/// can be easily compared using standard Rust operators.
fn main() {
    // a == b
    // 256：表示该 Uint 类型的位宽，即这是一个 256 位的无符号整数，类似于在 Solidity 中常见的 uint256。
    // 4：表示该 Uint 类型内部用于存储的字数组长度。由于每个字是 64 位（8 字节），4 个字（64 位 × 4 = 256 位）正好满足存储 256 位整数的需求。
    let a: alloy::primitives::Uint<256, 4> = U256::from(100_u32);
    let b = U256::from(100_u32);
    assert!(a == b);

    // a < b
    let a = U256::from(1_u32);
    let b = U256::from(100_u32);
    assert!(a < b);

    // a <= b
    let a = U256::from(100_u32);
    let b = U256::from(100_u32);
    assert!(a <= b);

    // a > b
    let a = U256::from(100_u32);
    let b = U256::from(1_u32);
    assert!(a > b);

    // a >= b
    let a = U256::from(100_u32);
    let b = U256::from(100_u32);
    assert!(a >= b);

    // a == 0
    let a = U256::ZERO;
    assert!(a.is_zero());
}
