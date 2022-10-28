use std::io::{self, Error, ErrorKind};

use crate::jsonrpc::{self, evm};

pub async fn block_number(
    http_rpc: &str,
    chain_id_alias: &str,
) -> io::Result<evm::BlockNumberResponse> {
    let chain_rpc_url_path = format!("/ext/bc/{}/rpc", chain_id_alias);
    log::info!("getting block number via {} {}", http_rpc, chain_id_alias,);

    let mut data = jsonrpc::Request::default();
    data.method = String::from("eth_blockNumber");

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, &chain_rpc_url_path, &d).await?;

    serde_json::from_slice(&rb)
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed eth_blockNumber '{}'", e)))
}

pub async fn chain_id(http_rpc: &str, chain_id_alias: &str) -> io::Result<evm::ChainIdResponse> {
    let chain_rpc_url_path = format!("/ext/bc/{}/rpc", chain_id_alias);
    log::info!("getting chain id via {} {}", http_rpc, chain_id_alias,);

    let mut data = jsonrpc::Request::default();
    data.method = String::from("eth_chainId");

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, &chain_rpc_url_path, &d).await?;

    serde_json::from_slice(&rb)
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed eth_chainId '{}'", e)))
}

/// Fetches the balance.
/// "chain_id_alias" is "C" for C-chain, and blockchain Id for subnet-evm.
/// e.g., "eth_getBalance" on "http://[ADDR]:9650" and "/ext/bc/C/rpc" path.
/// ref. https://docs.avax.network/build/avalanchego-apis/c-chain#eth_getassetbalance
pub async fn get_balance(
    http_rpc: &str,
    chain_id_alias: &str,
    eth_addr: &str,
) -> io::Result<evm::GetBalanceResponse> {
    let chain_rpc_url_path = format!("/ext/bc/{}/rpc", chain_id_alias);
    log::info!(
        "getting balances for {} via {} {}",
        eth_addr,
        http_rpc,
        chain_rpc_url_path
    );

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("eth_getBalance");

    let params = vec![String::from(eth_addr), "latest".to_string()];
    data.params = Some(params);

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, &chain_rpc_url_path, &d).await?;

    serde_json::from_slice(&rb)
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed eth_getBalance '{}'", e)))
}

/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_gettransactioncount
pub async fn get_latest_transaction_count(
    http_rpc: &str,
    chain_id_alias: &str,
    eth_addr: &str,
) -> io::Result<evm::GetTransactionCountResponse> {
    let chain_rpc_url_path = format!("/ext/bc/{}/rpc", chain_id_alias);
    log::info!(
        "getting transaction count for {} via {} {}",
        eth_addr,
        http_rpc,
        chain_rpc_url_path
    );

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("eth_getTransactionCount");

    let params = vec![String::from(eth_addr), "latest".to_string()];
    data.params = Some(params);

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, &chain_rpc_url_path, &d).await?;

    serde_json::from_slice(&rb).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed eth_getTransactionCount '{}'", e),
        )
    })
}

/// Get transaction receipt.
/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_gettransactionreceipt
pub async fn get_transaction_receipt(
    http_rpc: &str,
    chain_id_alias: &str,
    tx_hash: &str,
) -> io::Result<evm::GetTransactionReceiptResponse> {
    let chain_rpc_url_path = format!("/ext/bc/{}/rpc", chain_id_alias);
    log::info!(
        "getting transaction receipt for {} via {} {}",
        tx_hash,
        http_rpc,
        chain_rpc_url_path
    );

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("eth_getTransactionReceipt");

    let params = vec![String::from(tx_hash)];
    data.params = Some(params);

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, &chain_rpc_url_path, &d).await?;

    serde_json::from_slice(&rb).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed eth_getTransactionReceipt '{}'", e),
        )
    })
}

/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_signtransaction
/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendtransaction
/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendrawtransaction
pub async fn send_raw_transaction(
    http_rpc: &str,
    chain_id_alias: &str,
    tx_bytes_signed_hex: &str,
) -> io::Result<evm::SendRawTransactionResponse> {
    let chain_rpc_url_path = format!("/ext/bc/{}/rpc", chain_id_alias);
    log::info!("sending raw tx via {} {}", http_rpc, chain_id_alias,);

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("eth_sendRawTransaction");

    let params = vec![tx_bytes_signed_hex.to_string()];
    data.params = Some(params);

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, &chain_rpc_url_path, &d).await?;

    serde_json::from_slice(&rb).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed eth_sendRawTransaction '{}'", e),
        )
    })
}
