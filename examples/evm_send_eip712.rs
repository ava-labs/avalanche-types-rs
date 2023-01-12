use std::{convert::TryFrom, env::args, io};

use avalanche_types::key;
use ethers::contract::{Eip712, EthAbiType};
use ethers_core::types::{transaction::eip712::Eip712, Address, U256};
use ethers_providers::{Http, Provider};

/// Generate the EIP712 hash with "domainSeparator".
/// ref. <https://eips.ethereum.org/EIPS/eip-712>
/// ref. <https://eips.ethereum.org/EIPS/eip-2612>
#[derive(Eip712, EthAbiType, Clone)]
#[eip712(
    name = "Uniswap V2",
    version = "1",
    chain_id = 1,
    verifying_contract = "0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc"
)]
struct ForwardRequest {
    from: Address,
    to: Address,
    value: U256,
    gas: U256,
    nonce: U256,
    data: Vec<u8>,
    valid_until_time: U256,
}

/// cargo run --example evm_send_eip712 --features="jsonrpc_client evm" -- [HTTP RPC ENDPOINT]
/// cargo run --example evm_send_eip712 --features="jsonrpc_client evm" -- http://localhost:9876/rpc
#[tokio::main]
async fn main() -> io::Result<()> {
    // ref. https://github.com/env-logger-rs/env_logger/issues/47
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let forwarder = ForwardRequest {
        from: Address::random(),
        to: Address::random(),
        value: 100.into(),
        gas: 0.into(),
        nonce: 0.into(),
        data: vec![0, 1, 2],
        valid_until_time: U256::MAX,
    };
    let forwarder_hash = forwarder.encode_eip712().unwrap();
    println!("hash {:?}", forwarder_hash);

    let url = args().nth(1).expect("no url given");
    log::info!("running against {url}");

    let _provider =
        Provider::<Http>::try_from(url.clone()).expect("could not instantiate HTTP Provider");
    log::info!("created provider for {url}");

    let k = key::secp256k1::TEST_KEYS[0].clone();
    let key_info = k.to_info(1).unwrap();
    log::info!("created hot key:\n\n{}\n", key_info);
    let _signer: ethers_signers::LocalWallet = k.signing_key().into();

    Ok(())
}
