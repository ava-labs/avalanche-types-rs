use std::{env::args, io};

use avalanche_types::jsonrpc::client::info as jsonrpc_client_info;

/// cargo run --all-features --example jsonrpc_client_info -- [HTTP RPC ENDPOINT]
/// cargo run --all-features --example jsonrpc_client_info -- http://localhost:9650
/// cargo run --all-features --example jsonrpc_client_info -- http://44.230.236.23:9650
#[tokio::main]
async fn main() -> io::Result<()> {
    // ref. <https://github.com/env-logger-rs/env_logger/issues/47>
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let url = args().nth(1).expect("no url given");

    println!();
    let resp = jsonrpc_client_info::get_network_name(&url).await.unwrap();
    log::info!(
        "get_network_name response: {}",
        serde_json::to_string_pretty(&resp).unwrap()
    );

    println!();
    let resp = jsonrpc_client_info::get_network_id(&url).await.unwrap();
    log::info!(
        "get_network_id response: {}",
        serde_json::to_string_pretty(&resp).unwrap()
    );

    println!();
    let resp = jsonrpc_client_info::get_blockchain_id(&url, "X")
        .await
        .unwrap();
    log::info!(
        "get_blockchain_id for X response: {}",
        serde_json::to_string_pretty(&resp).unwrap()
    );
    log::info!(
        "blockchain_id for X: {}",
        resp.result.unwrap().blockchain_id
    );

    println!();
    let resp = jsonrpc_client_info::get_blockchain_id(&url, "P")
        .await
        .unwrap();
    log::info!("get_blockchain_id for P response: {:?}", resp);
    log::info!(
        "blockchain_id for P: {}",
        resp.result.unwrap().blockchain_id
    );

    println!();
    let resp = jsonrpc_client_info::get_blockchain_id(&url, "C")
        .await
        .unwrap();
    log::info!(
        "get_blockchain_id for C response: {}",
        serde_json::to_string_pretty(&resp).unwrap()
    );
    log::info!(
        "blockchain_id for C: {}",
        resp.result.unwrap().blockchain_id
    );

    println!();
    let resp = jsonrpc_client_info::get_node_id(&url).await.unwrap();
    log::info!(
        "get_node_id response: {}",
        serde_json::to_string_pretty(&resp).unwrap()
    );
    assert_eq!(
        resp.result
            .unwrap()
            .node_pop
            .unwrap()
            .pubkey
            .unwrap()
            .to_compressed_bytes()
            .len(),
        48
    );

    println!();
    let resp = jsonrpc_client_info::get_node_version(&url).await.unwrap();
    log::info!(
        "get_node_version response: {}",
        serde_json::to_string_pretty(&resp).unwrap()
    );

    println!();
    let resp = jsonrpc_client_info::get_vms(&url).await.unwrap();
    log::info!(
        "get_vms response: {}",
        serde_json::to_string_pretty(&resp).unwrap()
    );

    println!();
    let resp = jsonrpc_client_info::is_bootstrapped(&url).await.unwrap();
    log::info!(
        "get_bootstrapped response: {}",
        serde_json::to_string_pretty(&resp).unwrap()
    );

    println!();
    let resp = jsonrpc_client_info::get_tx_fee(&url).await.unwrap();
    log::info!(
        "get_tx_fee response: {}",
        serde_json::to_string_pretty(&resp).unwrap()
    );

    Ok(())
}
