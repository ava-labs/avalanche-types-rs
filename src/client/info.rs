use std::{
    collections::HashMap,
    io::{self, Error, ErrorKind},
};

use crate::jsonrpc::{self, info};

/// e.g., "info.getNetworkName".
/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetnetworkname>
pub async fn get_network_name(http_rpc: &str) -> io::Result<info::GetNetworkNameResponse> {
    log::info!("getting network name for {}", http_rpc);

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.getNetworkName");

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, "ext/info", &d).await?;

    serde_json::from_slice(&rb).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed info.getNetworkName '{}'", e),
        )
    })
}

/// e.g., "info.getNetworkID".
/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetnetworkid>
pub async fn get_network_id(http_rpc: &str) -> io::Result<info::GetNetworkIdResponse> {
    log::info!("getting network ID for {}", http_rpc);

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.getNetworkID");

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, "ext/info", &d).await?;

    serde_json::from_slice(&rb).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed info.getNetworkID '{}'", e),
        )
    })
}

/// e.g., "info.getBlockchainID".
/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetblockchainid>
pub async fn get_blockchain_id(
    http_rpc: &str,
    chain_alias: &str,
) -> io::Result<info::GetBlockchainIdResponse> {
    log::info!("getting blockchain ID for {} and {}", http_rpc, chain_alias);

    let mut data = jsonrpc::Request::default();
    data.method = String::from("info.getBlockchainID");

    let mut params = HashMap::new();
    params.insert(String::from("alias"), String::from(chain_alias));
    data.params = Some(params);

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, "ext/info", &d).await?;

    serde_json::from_slice(&rb).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed info.getBlockchainID '{}'", e),
        )
    })
}

/// e.g., "info.getNodeID".
/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetnodeid>
pub async fn get_node_id(http_rpc: &str) -> io::Result<info::GetNodeIdResponse> {
    log::info!("getting node ID for {}", http_rpc);

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.getNodeID");

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, "ext/info", &d).await?;

    serde_json::from_slice(&rb)
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed info.getNodeID '{}'", e)))
}

/// e.g., "info.getNodeVersion".
/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetnodeversion>
pub async fn get_node_version(http_rpc: &str) -> io::Result<info::GetNodeVersionResponse> {
    let joined = http_manager::join_uri(http_rpc, "ext/info")?;
    log::info!("getting node version for {}", joined.as_str());

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.getNodeVersion");

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, "ext/info", &d).await?;

    serde_json::from_slice(&rb).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed info.getNodeVersion '{}'", e),
        )
    })
}

/// e.g., "info.getVMs".
/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetvms>
pub async fn get_vms(http_rpc: &str) -> io::Result<info::GetVmsResponse> {
    log::info!("getting VMs for {}", http_rpc);

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.getVMs");

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, "ext/info", &d).await?;

    serde_json::from_slice(&rb)
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed info.getVMs '{}'", e)))
}

/// e.g., "info.isBootstrapped".
/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infoisbootstrapped>
pub async fn is_bootstrapped(http_rpc: &str) -> io::Result<info::IsBootstrappedResponse> {
    log::info!("getting bootstrapped for {}", http_rpc);

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.isBootstrapped");

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, "ext/info", &d).await?;

    serde_json::from_slice(&rb).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed info.isBootstrapped '{}'", e),
        )
    })
}

/// e.g., "info.getTxFee".
/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogettxfee>
pub async fn get_tx_fee(http_rpc: &str) -> io::Result<info::GetTxFeeResponse> {
    log::info!("getting node ID for {}", http_rpc);

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.getTxFee");

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, "ext/info", &d).await?;

    serde_json::from_slice(&rb)
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed info.getTxFee '{}'", e)))
}
