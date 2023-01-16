use std::{env::args, io, str::FromStr};

use avalanche_types::{evm::eip712::gsn::RelayTransactionRequestBuilder, key};
use ethers_core::types::{H160, U256};

/// cargo run --example evm_eip712_domain_separator_request_type_hash --features="evm" -- "my domain name" "1" 1234567 0x17aB05351fC94a1a67Bf3f56DdbB941aE6c63E25
fn main() -> io::Result<()> {
    // ref. https://github.com/env-logger-rs/env_logger/issues/47
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    macro_rules! ab {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    let domain_name = args().nth(1).expect("no domain_name given");
    log::info!("domain_name: {domain_name}");

    let domain_version = args().nth(2).expect("no domain_version given");
    log::info!("domain_version: {domain_version}");

    let domain_chain_id = args().nth(3).expect("no domain_chain_id given");
    let domain_chain_id = U256::from_str(&domain_chain_id).unwrap();
    log::info!("domain_chain_id: {domain_chain_id}");

    let domain_verifying_contract = args().nth(4).expect("no domain_verifying_contract given");
    let domain_verifying_contract =
        H160::from_str(&domain_verifying_contract.trim_start_matches("0x")).unwrap();
    log::info!("domain_verifying_contract: {domain_verifying_contract}");

    let k = key::secp256k1::TEST_KEYS[0].clone();
    let key_info = k.to_info(1).unwrap();
    log::info!("created hot key:\n\n{}\n", key_info);
    let signer: ethers_signers::LocalWallet = k.signing_key().into();

    let relay_tx_request = ab!(RelayTransactionRequestBuilder::new()
        .domain_name(domain_name)
        .domain_version(domain_version)
        .domain_chain_id(domain_chain_id)
        .domain_verifying_contract(domain_verifying_contract)
        .from(H160::random())
        .to(H160::random())
        .value(U256::zero())
        .nonce(U256::from(1))
        .data(vec![1, 2, 3])
        .valid_until_time(U256::MAX)
        .build_and_sign(signer))
    .unwrap();

    let domain_separator = relay_tx_request.domain_separator();
    log::info!("domain_separator: 0x{:x}", domain_separator);

    let request_type_hash = relay_tx_request.request_type_hash();
    log::info!("request_type_hash: 0x{:x}", request_type_hash);

    Ok(())
}
