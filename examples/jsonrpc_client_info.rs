use std::env::args;

use avalanche_types::jsonrpc::client::info;
use tokio::runtime::Runtime;

/// cargo run --example jsonrpc_client_info -- [HTTP RPC ENDPOINT]
/// cargo run --example jsonrpc_client_info -- http://54.180.73.56:9650
fn main() {
    // ref. https://github.com/env-logger-rs/env_logger/issues/47
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let url = args().nth(1).expect("no url given");
    let rt = Runtime::new().unwrap();

    let resp = rt
        .block_on(info::get_network_name(&url))
        .expect("failed get_network_name");
    log::info!("get_network_name response: {:?}", resp);

    let resp = rt
        .block_on(info::get_network_id(&url))
        .expect("failed get_network_id");
    log::info!("get_network_id response: {:?}", resp);

    let resp = rt
        .block_on(info::get_blockchain_id(&url, "X"))
        .expect("failed get_blockchain_id");
    log::info!("get_blockchain_id for X response: {:?}", resp);
    log::info!(
        "blockchain_id for X: {}",
        resp.result.unwrap().blockchain_id
    );

    let resp = rt
        .block_on(info::get_blockchain_id(&url, "P"))
        .expect("failed get_blockchain_id");
    log::info!("get_blockchain_id for P response: {:?}", resp);
    log::info!(
        "blockchain_id for P: {}",
        resp.result.unwrap().blockchain_id
    );

    let resp = rt
        .block_on(info::get_blockchain_id(&url, "C"))
        .expect("failed get_blockchain_id");
    log::info!("get_blockchain_id for C response: {:?}", resp);
    log::info!(
        "blockchain_id for C: {}",
        resp.result.unwrap().blockchain_id
    );

    let resp = rt
        .block_on(info::get_node_id(&url))
        .expect("failed get_node_id");
    log::info!("get_node_id response: {:?}", resp);

    let resp = rt
        .block_on(info::get_node_version(&url))
        .expect("failed get_node_version");
    log::info!("get_node_version response: {:?}", resp);

    let resp = rt.block_on(info::get_vms(&url)).expect("failed get_vms");
    log::info!("get_vms response: {:?}", resp);

    let resp = rt
        .block_on(info::is_bootstrapped(&url))
        .expect("failed get_bootstrapped");
    log::info!("get_bootstrapped response: {:?}", resp);

    let resp = rt
        .block_on(info::get_tx_fee(&url))
        .expect("failed get_tx_fee");
    log::info!("get_tx_fee response: {:?}", resp);
}
