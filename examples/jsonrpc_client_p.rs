use std::env::args;

use avalanche_types::jsonrpc::client::p;
use tokio::runtime::Runtime;

/// cargo run --example jsonrpc_client_p -- [HTTP RPC ENDPOINT] P-custom1qwmslrrqdv4slxvynhy9csq069l0u8mqwjzmcd
/// cargo run --example jsonrpc_client_p -- http://54.180.73.56:9650 P-custom1qwmslrrqdv4slxvynhy9csq069l0u8mqwjzmcd
///
/// ```
/// # or run this
/// avalanche-cli-rust get-utxos \
/// --http-rpc-endpoint [HTTP RPC ENDPOINT] \
/// --p-chain-address P-custom1qwmslrrqdv4slxvynhy9csq069l0u8mqwjzmcd
/// ```
///
fn main() {
    // ref. https://github.com/env-logger-rs/env_logger/issues/47
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let rt = Runtime::new().unwrap();

    let url = args().nth(1).expect("no url given");
    let paddr = args().nth(2).expect("no p-chain address given");

    println!("{}", url);
    println!("{}", paddr);
    let resp = rt
        .block_on(p::get_balance(&url, &paddr))
        .expect("failed to get balance");
    log::info!("get_balance response: {:?}", resp);

    let resp = rt
        .block_on(p::get_utxos(&url, &paddr))
        .expect("failed to get UTXOs");
    log::info!("get_utxos response: {:?}", resp);

    let resp = rt
        .block_on(p::get_height(&url))
        .expect("failed to get height");
    log::info!("get_height response: {:?}", resp);

    let resp = rt
        .block_on(p::get_primary_network_validators(&url))
        .expect("failed to get current validators");
    log::info!("get_current_validators response: {:?}", resp);
}
