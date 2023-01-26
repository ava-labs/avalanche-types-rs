#![allow(deprecated)]

use std::{convert::TryFrom, env::args, io, str::FromStr};

use avalanche_types::{
    evm::{abi, eip712::gsn::Tx},
    key,
};
use ethers_core::{
    abi::{Function, StateMutability},
    types::{H160, U256},
};
use ethers_providers::{Http, Middleware, Provider};

/// cargo run --example evm_contract_counter_relay_eip712 --features="jsonrpc_client evm" -- [HTTP RPC ENDPOINT]
/// cargo run --example evm_contract_counter_relay_eip712 --features="jsonrpc_client evm" -- http://localhost:9876/rpc
///
/// cast send --gas-price 700000000000 --priority-gas-price 10000000000 --private-key=56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027 --rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc 0x5DB9A7629912EBF95876228C24A848de0bfB43A9 "increment()"
/// cast call --rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc 0x5DB9A7629912EBF95876228C24A848de0bfB43A9 "getNumber()" | sed -r '/^\s*$/d' | tail -1
#[tokio::main]
async fn main() -> io::Result<()> {
    // ref. https://github.com/env-logger-rs/env_logger/issues/47
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let k = key::secp256k1::private_key::Key::from_hex(
        "1af42b797a6bfbd3cf7554bed261e876db69190f5eb1b806acbd72046ee957c3",
    )
    .unwrap();
    let key_info = k.to_info(1).unwrap();
    log::info!("created hot key:\n\n{}\n", key_info);
    let signer: ethers_signers::LocalWallet = k.to_ethers_core_signing_key().into();

    // parsed function of "increment()"
    let func = Function {
        name: "increment".to_string(),
        inputs: vec![],
        outputs: Vec::new(),
        constant: None,
        state_mutability: StateMutability::NonPayable,
    };
    let arg_tokens = vec![];
    let calldata = abi::encode_calldata(func, &arg_tokens).unwrap();
    log::info!("calldata: 0x{}", hex::encode(calldata.clone()));

    let relay_tx_request = Tx::new()
        //
        // make sure this matches with "registerDomainSeparator" call
        .domain_name("my name")
        .domain_version("1")
        //
        // local network
        .domain_chain_id(U256::from(43112))
        //
        // trusted forwarder contract address
        .domain_verifying_contract(
            H160::from_str("0x52C84043CD9c865236f11d9Fc9F56aa003c1f922".trim_start_matches("0x"))
                .unwrap(),
        )
        .from(key_info.h160_address.clone())
        //
        // contract address that this gasless transaction will interact with
        .to(
            H160::from_str("0x5DB9A7629912EBF95876228C24A848de0bfB43A9".trim_start_matches("0x"))
                .unwrap(),
        )
        //
        // contract call needs no value
        .value(U256::zero())
        //
        // assume this is the first transaction
        .nonce(U256::from(1))
        //
        // calldata for contract calls
        .data(calldata)
        //
        //
        .valid_until_time(U256::MAX)
        //
        //
        .type_name("my name")
        //
        //
        .type_suffix_data("my suffix")
        //
        //
        .sign_to_request(signer)
        .await
        .unwrap();

    log::info!("relay_tx_request: {:?}", relay_tx_request);

    let signed_bytes: ethers_core::types::Bytes =
        serde_json::to_vec(&relay_tx_request).unwrap().into();

    let url = args().nth(1).expect("no url given");
    log::info!("running against {url}");

    let provider =
        Provider::<Http>::try_from(url.clone()).expect("could not instantiate HTTP Provider");
    log::info!("created provider for {url}");

    let pending = provider.send_raw_transaction(signed_bytes).await.unwrap();
    log::info!("pending tx hash {}", pending.tx_hash());

    Ok(())
}
