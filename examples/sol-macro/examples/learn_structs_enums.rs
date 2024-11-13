#![allow(missing_docs)]

use alloy::{primitives::U256, sol};
use eyre::Result;

sol! {
    #[derive(Debug)]
    struct Foo {
        uint256 a;
        uint64 b;
        Bar greater;
    }

    #[allow(missing_docs)]
    #[derive(Debug)]
    enum Bar {
        A,
        B,
    }
}

fn main() -> Result<()> {
    let foo = Foo { a: U256::from(1), b: 2_u64, greater: Bar::A };

    println!("{foo:?}");

    Ok(())
}
