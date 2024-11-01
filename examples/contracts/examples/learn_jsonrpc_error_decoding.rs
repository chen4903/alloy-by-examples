#![allow(missing_docs)]

use alloy::{primitives::U256, rpc::json_rpc::ErrorPayload, sol};
use eyre::Result;

sol! {
    library Errors {
        error SomeCustomError(uint256 iamhere);
    }
}

fn main() -> Result<()> {
    // 类似r#xxxxxxx#的写法，可以在里面写冒号等特殊字符，然后不需要转译
    let json =  r#"{"code":3,"message":"execution reverted: ","data":"0x810f00230000000000000000000000000000000000000000000000000000000000000001"}"#;

    let payload: ErrorPayload = serde_json::from_str(json)?;
    let Errors::ErrorsErrors::SomeCustomError(value) = payload.as_decoded_error::<Errors::ErrorsErrors>(false).unwrap();

    assert_eq!(value.iamhere, U256::from(1));

    Ok(())
}