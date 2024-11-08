#![allow(missing_docs)]

use alloy::signers::{aws::AwsSigner, Signer};
use aws_config::BehaviorVersion;
use eyre::Result;

// AWS KMS签名器是一种使用AWS KMS（密钥管理服务）中的密钥执行加密签名操作的组件。
// AWS KMS是一个云端的密钥管理服务，允许用户创建、管理和控制加密密钥。这些密钥通常用于保护应用中的敏感数据，也可以用于签名操作。

#[tokio::main]
async fn main() -> Result<()> {
    let Ok(key_id) = std::env::var("AWS_KEY_ID") else {
        return Ok(());
    };

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_kms::Client::new(&config);
    let signer = AwsSigner::new(client, key_id, Some(1)).await?;

    let message = "Hello, world!";
    let signature = signer.sign_message(message.as_bytes()).await?;

    assert_eq!(signature.recover_address_from_msg(message)?, signer.address());

    Ok(())
}