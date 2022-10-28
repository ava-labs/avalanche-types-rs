use std::env::args;

use avalanche_types::client::x;
use tokio::runtime::Runtime;

/// cargo run --example client_x -- [HTTP RPC ENDPOINT] X-custom152qlr6zunz7nw2kc4lfej3cn3wk46u3002k4w5
fn main() {
    // ref. https://github.com/env-logger-rs/env_logger/issues/47
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let rt = Runtime::new().unwrap();

    let url = args().nth(1).expect("no url given");
    let xaddr = args().nth(2).expect("no x-chain address given");

    let resp = rt
        .block_on(x::get_balance(&url, &xaddr))
        .expect("failed get_balance");
    log::info!("get_balance response: {:?}", resp);

    let resp = rt
        .block_on(x::get_asset_description(&url, "AVAX"))
        .expect("failed get_asset_description");
    log::info!("get_asset_description response: {:?}", resp);
}
