#![allow(missing_docs)]

use alloy::{
    providers::{Provider, ProviderBuilder},
    rpc::client::RpcClient,
    transports::http::{
        reqwest::{
            header::{HeaderMap, HeaderValue, AUTHORIZATION},
            Client,
        },
        Http,
    },
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_static("deadbeef"));

    let client_with_auth = Client::builder().default_headers(headers).build()?;

    let rpc_url = "https://eth.merkle.io".parse()?;
    let http = Http::with_client(client_with_auth, rpc_url);
    let rpc_client = RpcClient::new(http, false);

    let provider = ProviderBuilder::new().on_client(rpc_client);

    // Get latest block number.
    let latest_block = provider.get_block_number().await?;

    println!("Latest block number: {latest_block}");

    Ok(())
}