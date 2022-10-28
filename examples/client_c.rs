use std::env::args;

use avalanche_types::client::evm;
use tokio::runtime::Runtime;

/// cargo run --example client_c -- [HTTP RPC ENDPOINT] 0x613040a239BDfCF110969fecB41c6f92EA3515C0
fn main() {
    // ref. https://github.com/env-logger-rs/env_logger/issues/47
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let rt = Runtime::new().unwrap();

    let url = args().nth(1).expect("no url given");
    let caddr = args().nth(2).expect("no C-chain address given");

    let resp = rt
        .block_on(evm::get_balance(&url, "C", &caddr))
        .expect("failed to get balance");
    log::info!("response: {:?}", resp);
}
