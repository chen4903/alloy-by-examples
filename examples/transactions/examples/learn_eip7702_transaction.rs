#![allow(missing_docs)]

use alloy::{
    eips::eip7702::Authorization,
    network::{EthereumWallet, TransactionBuilder, TransactionBuilder7702},
    node_bindings::Anvil,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::{local::PrivateKeySigner, SignerSync},
    sol,
};
use eyre::Result;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc, bytecode = "6080806040523460135760c9908160188239f35b5f80fdfe6004361015600b575f80fd5b5f3560e01c80637b3ab2d014605f57639ee1a440146027575f80fd5b34605b575f366003190112605b577f2d67bb91f17bca05af6764ab411e86f4ddf757adb89fcec59a7d21c525d417125f80a1005b5f80fd5b34605b575f366003190112605b577fbcdfe0d5b27dd186282e187525415c57ea3077c34efb39148111e4d342e7ab0e5f80a100fea2646970667358221220f6b42b522bc9fb2b4c7d7e611c7c3e995d057ecab7fd7be4179712804c886b4f64736f6c63430008190033")]
    contract Log {
        #[derive(Debug)]
        event Hello();
        event World();

        function emitHello() public {
            emit Hello();
        }

        function emitWorld() public {
            emit World();
        }
    }
);

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().arg("--hardfork").arg("prague").try_spawn()?;

    let alice: PrivateKeySigner = anvil.keys()[0].clone().into();
    let bob: PrivateKeySigner = anvil.keys()[1].clone().into();

    let rpc_url = anvil.endpoint_url();
    let wallet = EthereumWallet::from(bob.clone());
    let provider =
        ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(rpc_url);

    let contract = Log::deploy(&provider).await?;

    let authorization = Authorization {
        chain_id: anvil.chain_id(),
        address: *contract.address(),
        nonce: provider.get_transaction_count(alice.address()).await?,
    };

    let signature = alice.sign_hash_sync(&authorization.signature_hash())?;
    let signed_authorization = authorization.into_signed(signature);

    let call = contract.emitHello();
    let emit_hello_calldata = call.calldata().to_owned();

    let tx = TransactionRequest::default()
        .with_to(alice.address())
        .with_authorization_list(vec![signed_authorization])
        .with_input(emit_hello_calldata);

    let pending_tx = provider.send_transaction(tx).await?;

    println!("Pending transaction... {}", pending_tx.tx_hash());

    let receipt = pending_tx.get_receipt().await?;

    println!(
        "Transaction included in block {}",
        receipt.block_number.expect("Failed to get block number")
    );

    assert!(receipt.status());
    assert_eq!(receipt.from, bob.address());
    assert_eq!(receipt.to, Some(alice.address()));
    assert_eq!(receipt.inner.logs().len(), 1);
    assert_eq!(receipt.inner.logs()[0].address(), alice.address());

    Ok(())
}
