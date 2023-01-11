use std::{env::args, str::FromStr};

use avalanche_types::jsonrpc::client::evm;
use tokio::runtime::Runtime;

/// cargo run --example jsonrpc_client_evm -- [HTTP RPC ENDPOINT] 0x613040a239BDfCF110969fecB41c6f92EA3515C0
/// cargo run --example jsonrpc_client_evm -- http://localhost:9650 0x613040a239BDfCF110969fecB41c6f92EA3515C0
/// cargo run --example jsonrpc_client_evm -- http://54.180.73.56:9650 0x613040a239BDfCF110969fecB41c6f92EA3515C0
fn main() {
    // ref. https://github.com/env-logger-rs/env_logger/issues/47
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let rt = Runtime::new().unwrap();

    let http_rpc = args().nth(1).expect("no http_rpc given");
    let caddr = args().nth(2).expect("no C-chain address given");

    let chain_id = rt
        .block_on(evm::chain_id(format!("{http_rpc}/ext/bc/C/rpc").as_str()))
        .expect("failed to get chain_id");
    log::info!("chain_id: {:?}", chain_id);

    let balance = rt
        .block_on(evm::get_balance(
            format!("{http_rpc}/ext/bc/C/rpc").as_str(),
            primitive_types::H160::from_str(caddr.trim_start_matches("0x")).unwrap(),
        ))
        .expect("failed to get balance");
    log::info!("balance: {:?}", balance);
}
