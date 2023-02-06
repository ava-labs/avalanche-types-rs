use std::{
    collections::HashMap,
    io::{self, Error, ErrorKind},
};

use crate::jsonrpc::{self, platformvm};

/// e.g., "platform.issueTx" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators>
pub async fn issue_tx(http_rpc: &str, tx: &str) -> io::Result<platformvm::IssueTxResponse> {
    let joined = http_manager::join_uri(http_rpc, "/ext/P")?;
    log::debug!("issuing a transaction via {:?}", joined.as_str());

    let mut data = platformvm::IssueTxRequest::default();
    data.method = String::from("platform.issueTx");

    let params = platformvm::IssueTxParams {
        tx: prefix_manager::prepend_0x(tx),
        encoding: String::from("hex"), // don't use "cb58"
    };
    data.params = Some(params);

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, "/ext/P", &d).await?;

    serde_json::from_slice(&rb)
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed platform.issueTx '{}'", e)))
}

/// e.g., "platform.getTx" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/apis/avalanchego/apis/p-chain/#platformgettx>
pub async fn get_tx(http_rpc: &str, tx_id: &str) -> io::Result<platformvm::GetTxResponse> {
    let joined = http_manager::join_uri(http_rpc, "/ext/P")?;
    log::debug!("getting tx via {}", joined.as_str());

    let mut data = jsonrpc::Request::default();
    data.method = String::from("platform.getTx");

    let mut params = HashMap::new();
    params.insert(String::from("txID"), String::from(tx_id));
    params.insert(String::from("encoding"), String::from("json")); // TODO: use "hex"?
    data.params = Some(params);

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, "/ext/P", &d).await?;

    serde_json::from_slice(&rb)
        .map_err(|e| Error::new(ErrorKind::InvalidData, format!("failed to decode '{}'", e)))
}

/// e.g., "platform.getTxStatus" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/apis/avalanchego/apis/p-chain/#platformgettxstatus>
pub async fn get_tx_status(
    http_rpc: &str,
    tx_id: &str,
) -> io::Result<platformvm::GetTxStatusResponse> {
    let joined = http_manager::join_uri(http_rpc, "/ext/P")?;
    log::debug!("getting tx status via {}", joined.as_str());

    let mut data = jsonrpc::Request::default();
    data.method = String::from("platform.getTxStatus");

    let mut params = HashMap::new();
    params.insert(String::from("txID"), String::from(tx_id));
    data.params = Some(params);

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, "/ext/P", &d).await?;

    serde_json::from_slice(&rb).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed platform.getTxStatus '{}'", e),
        )
    })
}

/// e.g., "platform.getHeight" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetheight>
pub async fn get_height(http_rpc: &str) -> io::Result<platformvm::GetHeightResponse> {
    let joined = http_manager::join_uri(http_rpc, "/ext/P")?;
    log::debug!("getting height for {:?}", joined);

    let mut data = jsonrpc::Request::default();
    data.method = String::from("platform.getHeight");

    let params = HashMap::new();
    data.params = Some(params);

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, "/ext/P", &d).await?;

    serde_json::from_slice(&rb).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed platform.getHeight '{}'", e),
        )
    })
}

/// e.g., "platform.getBalance" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetbalance>
/// ref. <https://github.com/ava-labs/avalanchego/blob/45ec88151f8a0e3bca1d43fe902fd632c41cd956/vms/platformvm/service.go#L192-L194>
pub async fn get_balance(
    http_rpc: &str,
    paddr: &str,
) -> io::Result<platformvm::GetBalanceResponse> {
    let joined = http_manager::join_uri(http_rpc, "/ext/P")?;
    log::debug!("getting balances for {} via {:?}", paddr, joined);

    let mut data = jsonrpc::RequestWithParamsHashMapToArray::default();
    data.method = String::from("platform.getBalance");

    let mut params = HashMap::new();
    params.insert(String::from("addresses"), vec![paddr.to_string()]);
    data.params = Some(params);

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, "/ext/P", &d).await?;

    serde_json::from_slice(&rb).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed platform.getBalance '{}'", e),
        )
    })
}

/// e.g., "platform.getUTXOs" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetutxos>
pub async fn get_utxos(http_rpc: &str, paddr: &str) -> io::Result<platformvm::GetUtxosResponse> {
    let joined = http_manager::join_uri(http_rpc, "/ext/P")?;
    log::debug!("getting UTXOs for {} via {:?}", paddr, joined);

    let mut data = platformvm::GetUtxosRequest::default();
    data.method = String::from("platform.getUTXOs");

    let params = platformvm::GetUtxosParams {
        addresses: vec![paddr.to_string()],
        limit: 100,
        encoding: String::from("hex"), // don't use "cb58"
    };
    data.params = Some(params);

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, "/ext/P", &d).await?;

    serde_json::from_slice(&rb).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed platform.getUTXOs '{}'", e),
        )
    })
}

/// e.g., "platform.getCurrentValidators" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators>
/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/platformvm#ClientPermissionlessValidator>
pub async fn get_primary_network_validators(
    http_rpc: &str,
) -> io::Result<platformvm::GetCurrentValidatorsResponse> {
    let joined = http_manager::join_uri(http_rpc, "/ext/P")?;
    log::debug!("getting primary network validators via {}", joined.as_str());

    let mut data = jsonrpc::Request::default();
    data.method = String::from("platform.getCurrentValidators");

    let params = HashMap::new();
    data.params = Some(params);

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, "/ext/P", &d).await?;

    serde_json::from_slice(&rb).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed platform.getCurrentValidators '{}'", e),
        )
    })
}

/// e.g., "platform.getCurrentValidators" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators>
/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/platformvm#ClientPermissionlessValidator>
pub async fn get_subnet_validators(
    http_rpc: &str,
    subnet_id: &str,
) -> io::Result<platformvm::GetCurrentValidatorsResponse> {
    let joined = http_manager::join_uri(http_rpc, "/ext/P")?;
    log::debug!(
        "getting subnet {} validators via {}",
        subnet_id,
        joined.as_str()
    );

    let mut data = jsonrpc::Request::default();
    data.method = String::from("platform.getCurrentValidators");

    let mut params = HashMap::new();
    params.insert(String::from("subnetID"), subnet_id.to_string());
    data.params = Some(params);

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, "/ext/P", &d).await?;

    serde_json::from_slice(&rb).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed platform.getCurrentValidators '{}'", e),
        )
    })
}
