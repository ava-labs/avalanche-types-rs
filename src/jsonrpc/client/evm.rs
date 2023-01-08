use std::{
    io::{self, Error, ErrorKind},
    time::Duration,
};

use ethers_providers::{Http, Middleware, Provider};
use primitive_types::{H160, U256};

pub async fn chain_id(http_rpc: &str, chain_id_alias: &str) -> io::Result<U256> {
    let chain_rpc_url_path = format!("/ext/bc/{}/rpc", chain_id_alias);
    let rpc_ep = format!("{http_rpc}{chain_rpc_url_path}");
    let provider = Provider::<Http>::try_from(&rpc_ep)
        .map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed to create provider '{}'", e),
            )
        })?
        .interval(Duration::from_millis(2000u64));

    log::info!("getting chain id via {} {}", http_rpc, chain_id_alias,);
    provider
        .get_chainid()
        .await
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed get_chainid '{}'", e)))
}

/// Fetches the balance.
/// "chain_id_alias" is "C" for C-chain, and blockchain Id for subnet-evm.
/// e.g., "eth_getBalance" on "http://[ADDR]:9650" and "/ext/bc/C/rpc" path.
/// ref. <https://docs.avax.network/build/avalanchego-apis/c-chain#eth_getassetbalance>
pub async fn get_balance(http_rpc: &str, chain_id_alias: &str, eth_addr: H160) -> io::Result<U256> {
    let chain_rpc_url_path = format!("/ext/bc/{}/rpc", chain_id_alias);
    let rpc_ep = format!("{http_rpc}{chain_rpc_url_path}");
    let provider = Provider::<Http>::try_from(&rpc_ep)
        .map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed to create provider '{}'", e),
            )
        })?
        .interval(Duration::from_millis(2000u64));

    log::info!(
        "getting balances for {} via {} {}",
        eth_addr,
        http_rpc,
        chain_rpc_url_path
    );
    provider
        .get_balance(eth_addr, None)
        .await
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed get_balance '{}'", e)))
}
