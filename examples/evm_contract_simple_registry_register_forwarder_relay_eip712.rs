#![allow(deprecated)]

use std::{convert::TryFrom, env::args, io, str::FromStr};

use avalanche_types::{
    evm::{abi, eip712::gsn::Tx},
    jsonrpc::client::evm as json_client_evm,
    key,
};
use ethers_core::{
    abi::{Function, Param, ParamType, StateMutability, Token},
    types::{H160, U256},
};
use ethers_providers::{Http, Middleware, Provider};

/// cargo run --example evm_contract_simple_registry_register_forwarder_relay_eip712 --features="jsonrpc_client evm" -- [RELAY SERVER HTTP RPC ENDPOINT] [EVM HTTP RPC ENDPOINT] [FORWARDER CONTRACT ADDRESS] [RECIPIENT CONTRACT ADDRESS]
/// cargo run --example evm_contract_simple_registry_register_forwarder_relay_eip712 --features="jsonrpc_client evm" -- http://127.0.0.1:9876/rpc http://127.0.0.1:9650/ext/bc/C/rpc 0x52C84043CD9c865236f11d9Fc9F56aa003c1f922 0x5DB9A7629912EBF95876228C24A848de0bfB43A9
#[tokio::main]
async fn main() -> io::Result<()> {
    // ref. https://github.com/env-logger-rs/env_logger/issues/47
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let relay_server_rpc_url = args().nth(1).expect("no relay server RPC URL given");
    let chain_rpc_url = args().nth(2).expect("no chain RPC URL given");

    let forwarder_contract_addr = args().nth(3).expect("no forwarder contract address given");
    let forwarder_contract_addr =
        H160::from_str(forwarder_contract_addr.trim_start_matches("0x")).unwrap();

    let recipient_contract_addr = args().nth(4).expect("no recipient contract address given");
    let recipient_contract_addr =
        H160::from_str(recipient_contract_addr.trim_start_matches("0x")).unwrap();

    let chain_id = json_client_evm::chain_id(&chain_rpc_url).await.unwrap();
    log::info!(
        "running against {chain_rpc_url}, {chain_id} for forwarder contract {forwarder_contract_addr}, recipient contract {recipient_contract_addr}"
    );

    let no_gas_key = key::secp256k1::private_key::Key::generate().unwrap();
    let no_gas_key_info = no_gas_key.to_info(1).unwrap();
    log::info!("created hot key:\n\n{}\n", no_gas_key_info);
    let no_gas_key_signer: ethers_signers::LocalWallet =
        no_gas_key.to_ethers_core_signing_key().into();

    // parsed function of "register(string name)"
    let func = Function {
        name: "register".to_string(),
        inputs: vec![Param {
            name: "name".to_string(),
            kind: ParamType::String,
            internal_type: None,
        }],
        outputs: Vec::new(),
        constant: None,
        state_mutability: StateMutability::NonPayable,
    };
    let name_to_register = random_manager::string(10);
    log::info!("registering {name_to_register}");
    let arg_tokens = vec![Token::String(name_to_register.clone())];
    let no_gas_recipient_contract_calldata = abi::encode_calldata(func, &arg_tokens).unwrap();
    log::info!(
        "no gas recipient contract calldata: 0x{}",
        hex::encode(no_gas_recipient_contract_calldata.clone())
    );

    let relay_tx_request = Tx::new()
        //
        // make sure this matches with "registerDomainSeparator" call
        .domain_name("my name")
        .domain_version("1")
        //
        // local network
        .domain_chain_id(chain_id)
        //
        // trusted forwarder contract address
        .domain_verifying_contract(forwarder_contract_addr)
        .from(no_gas_key_info.h160_address.clone())
        //
        // contract address that this gasless transaction will interact with
        .to(recipient_contract_addr)
        //
        // fails if zero (e.g., "out of gas")
        // TODO: better estimate gas based on "RelayHub"
        .gas(U256::from(30000))
        //
        // contract call needs no value
        .value(U256::zero())
        //
        // assume this is the first transaction
        .nonce(U256::from(0))
        //
        // calldata for contract calls
        .data(no_gas_recipient_contract_calldata)
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
        .sign_to_request(no_gas_key_signer)
        .await
        .unwrap();

    log::info!("relay_tx_request: {:?}", relay_tx_request);

    let signed_bytes: ethers_core::types::Bytes =
        serde_json::to_vec(&relay_tx_request).unwrap().into();

    let provider = Provider::<Http>::try_from(relay_server_rpc_url.clone())
        .expect("could not instantiate HTTP Provider");
    log::info!("created provider for {relay_server_rpc_url}");

    let pending = provider.send_raw_transaction(signed_bytes).await.unwrap();
    log::info!(
        "pending tx hash {} from 0x{:x}",
        pending.tx_hash(),
        no_gas_key_info.h160_address
    );
    log::info!("registered {name_to_register}");

    Ok(())
}
