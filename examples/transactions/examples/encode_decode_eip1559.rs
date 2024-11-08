//! Example showing how to encode and decode an [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559) transaction.

use alloy::{
    consensus::{SignableTransaction, TxEip1559},
    eips::eip2930::AccessList,
    primitives::{address, b256, hex, Signature, TxKind, U256},
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // EIP1559 transaction: <https://etherscan.io/tx/0x0ec0b6a2df4d87424e5f6ad2a654e27aaeb7dac20ae9e8385cc09087ad532ee0>
    let tx_hash = b256!("0ec0b6a2df4d87424e5f6ad2a654e27aaeb7dac20ae9e8385cc09087ad532ee0");

    // Signer of the transaction.
    let signer = address!("DD6B8b3dC6B7AD97db52F08a275FF4483e024CEa");

    // Construct the EIP-1559 transaction.
    let tx = TxEip1559 {
        chain_id: 1,
        nonce: 0x42,
        gas_limit: 44386,
        to: TxKind::Call( address!("6069a6c32cf691f5982febae4faf8a6f3ab2f0f6")),
        value: U256::from(0_u64),
        input: hex!("a22cb4650000000000000000000000005eee75727d804a2b13038928d36f8b188945a57a0000000000000000000000000000000000000000000000000000000000000000").into(),
        max_fee_per_gas: 0x4a817c800,
        max_priority_fee_per_gas: 0x3b9aca00,
        access_list: AccessList::default(),
    };

    // Construct the signature of the transaction.
    // Ethereum的签名由两个部分的数字（r和s）和一个标识符（通常为v或parity）组成。
    // Signature::from_scalars_and_parity使用这两个标量值和一个布尔值来生成一个签名对象
    let signature = Signature::from_scalars_and_parity(
        b256!("840cfc572845f5786e702984c2a582528cad4b49b2a10b9db1be7fca90058565"),
        b256!("25e7109ceb98168d95b09b18bbf6b685130e0562f233877d492b94eee0c5b6d1"),
        // 该布尔值指定签名的奇偶性（也称为parity），通常用于指示恢复签名时使用的特定公钥。
        // Ethereum签名的parity参数可以是true或false，代表不同的签名恢复路径（用于唯一确定一
        // 个签名和它的签名者地址）。
        // parity本质上就是v值的简化版（例如：v = 27等价于parity = false，而v = 28等价于parity = true）
        false,
    )?;

    // Convert the transaction into a signed transaction.
    let signed_tx = tx.into_signed(signature);
    assert_eq!(*signed_tx.hash(), tx_hash);

    // Recover the signer from the signed transaction to ensure it matches the expected signer.
    let recovered_signer = signed_tx.recover_signer()?;
    assert_eq!(recovered_signer, signer);

    Ok(())
}
