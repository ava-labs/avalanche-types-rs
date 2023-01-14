use std::{convert::TryFrom, env::args, io};

use avalanche_types::{
    evm::eip712::gsn::{RelayTransactionRequest, RelayTransactionRequestBuilder},
    key,
};
use ethers_core::types::{H160, U256};
use ethers_providers::{Http, Middleware, Provider};

/// cargo run --example evm_send_eip712_relay_request --features="jsonrpc_client evm" -- [HTTP RPC ENDPOINT]
/// cargo run --example evm_send_eip712_relay_request --features="jsonrpc_client evm" -- http://localhost:9876/rpc
#[tokio::main]
async fn main() -> io::Result<()> {
    // ref. https://github.com/env-logger-rs/env_logger/issues/47
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let k = key::secp256k1::TEST_KEYS[0].clone();
    let key_info = k.to_info(1).unwrap();
    log::info!("created hot key:\n\n{}\n", key_info);
    let signer: ethers_signers::LocalWallet = k.signing_key().into();

    let typed_data = RelayTransactionRequestBuilder::new()
        .domain_name("example.com")
        .domain_version("1")
        .domain_chain_id(U256::from(1))
        .domain_verifying_contract(H160::random())
        .from(H160::random())
        .to(H160::random())
        .value(U256::zero())
        .nonce(U256::from(1))
        .data(vec![1, 2, 3])
        .valid_until_time(U256::MAX)
        .build_typed_data();
    let relay_tx_request = RelayTransactionRequest::sign(signer, typed_data)
        .await
        .unwrap();
    println!("relay_tx_request: {:?}", relay_tx_request);

    let signed_bytes: ethers_core::types::Bytes =
        serde_json::to_vec(&relay_tx_request).unwrap().into();
    println!("signed_bytes: {}", signed_bytes);

    let url = args().nth(1).expect("no url given");
    log::info!("running against {url}");

    let provider =
        Provider::<Http>::try_from(url.clone()).expect("could not instantiate HTTP Provider");
    log::info!("created provider for {url}");

    let pending = provider.send_raw_transaction(signed_bytes).await.unwrap();
    log::info!("pending tx hash {}", pending.tx_hash());

    Ok(())
}
