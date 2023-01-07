use std::{env::args, io, ops::Div};

use avalanche_types::{key, wallet};
use ethers_providers::Middleware;
use primitive_types::U256;

/// cargo run --example wallet_evm_send_transaction_hot_key -- [HTTP RPC ENDPOINT] [CHAIN ALIAS] [PRIVATE KEY]
/// cargo run --example wallet_evm_send_transaction_hot_key -- http://54.180.73.56:9650 C 1000777 56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027
/// cargo run --example wallet_evm_send_transaction_hot_key -- http://54.180.73.56:9650 2BsVz9yXyJAZ2EAGiNSZNWJnfEWkN2DuCFHGECkw676S84EgM2 2000777 56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027
#[tokio::main]
async fn main() -> io::Result<()> {
    // ref. https://github.com/env-logger-rs/env_logger/issues/47
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let url = args().nth(1).expect("no url given");
    let chain_alias = args().nth(2).expect("no chain_alias given");
    let chain_id: u64 = args().nth(3).expect("no chain_id given").parse().unwrap();
    log::info!("running against {url} {chain_alias} {chain_id}");

    let private_key = args().nth(4).expect("no private_key given");

    let k1 = key::secp256k1::private_key::Key::from_hex(private_key).unwrap();
    let key_info1 = k1.to_info(1).unwrap();
    log::info!("created hot key:\n\n{}\n", key_info1);

    let k1_eth_signer: ethers_signers::LocalWallet = k1.signing_key().into();

    let k2 = key::secp256k1::private_key::Key::generate().unwrap();
    let key_info2 = k2.to_info(1).unwrap();
    log::info!("created hot key:\n\n{}\n", key_info2);

    let w = wallet::Builder::new(&k1)
        .http_rpc(url.clone())
        .build()
        .await?;
    let evm_wallet = w.evm(&k1_eth_signer, chain_alias, U256::from(chain_id))?;

    let c_bal = evm_wallet.balance().await?;
    let transfer_amount = c_bal.div(U256::from(10));

    let (max_fee_per_gas, max_priority_fee_per_gas) = evm_wallet
        .picked_middleware
        .estimate_eip1559_fees(None)
        .await
        .unwrap();
    log::info!("max_fee_per_gas: {}", max_fee_per_gas);
    log::info!("max_priority_fee_per_gas: {}", max_priority_fee_per_gas);

    // TODO: make gas configurable
    // ref. <https://www.rapidtables.com/convert/number/decimal-to-hex.html>
    let tx_id = evm_wallet
        .eip1559()
        .recipient(key_info2.h160_address)
        .value(transfer_amount)
        // .max_priority_fee_per_gas(U256::from_str_radix("3EDD410C00", 16).unwrap()) // 270000000000
        // .max_fee_per_gas(U256::from_str_radix("60DB88400", 16).unwrap()) // 26000000000
        // .gas_limit(21000)
        .check_acceptance(true)
        .submit()
        .await?;
    log::info!("evm ethers wallet SUCCESS with transaction id {}", tx_id);

    Ok(())
}
