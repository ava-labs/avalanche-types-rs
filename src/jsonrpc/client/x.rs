use std::{
    collections::HashMap,
    io::{self, Error, ErrorKind},
    time::Duration,
};

use crate::{
    jsonrpc::{self, avm},
    utils,
};
use reqwest::{header::CONTENT_TYPE, ClientBuilder};

/// e.g., "avm.issueTx" on "http://[ADDR]:9650" and "/ext/bc/X" path.
/// ref. <https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmissuetx>
pub async fn issue_tx(http_rpc: &str, tx: &str) -> io::Result<avm::IssueTxResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/bc/X")
        } else {
            format!("{scheme}://{host}/ext/bc/X")
        }
    } else {
        format!("http://{host}/ext/bc/X")
    };
    log::info!("issuing a transaction via {u}");

    let mut data = avm::IssueTxRequest::default();
    data.method = String::from("avm.issueTx");
    let params = avm::IssueTxParams {
        tx: prefix_manager::prepend_0x(tx),
        encoding: String::from("hex"), // don't use "cb58"
    };
    data.params = Some(params);
    let d = data.encode_json()?;

    let req_cli_builder = ClientBuilder::new()
        .user_agent(env!("CARGO_PKG_NAME"))
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(15))
        .connection_verbose(true)
        .build()
        .map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed ClientBuilder build {}", e),
            )
        })?;
    let resp = req_cli_builder
        .post(&u)
        .header(CONTENT_TYPE, "application/json")
        .body(d)
        .send()
        .await
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed ClientBuilder send {}", e)))?;
    let out = resp.bytes().await.map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed ClientBuilder bytes {}", e),
        )
    })?;
    let out: Vec<u8> = out.into();

    serde_json::from_slice(&out)
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed avm.issueTx '{}'", e)))
}

/// e.g., "avm.getTxStatus" on "http://[ADDR]:9650" and "/ext/bc/X" path.
/// ref. <https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgettxstatus>
pub async fn get_tx_status(http_rpc: &str, tx_id: &str) -> io::Result<avm::GetTxStatusResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/bc/X")
        } else {
            format!("{scheme}://{host}/ext/bc/X")
        }
    } else {
        format!("http://{host}/ext/bc/X")
    };
    log::info!("getting tx status via {u}");

    let mut data = jsonrpc::Request::default();
    data.method = String::from("avm.getTxStatus");
    let mut params = HashMap::new();
    params.insert(String::from("txID"), String::from(tx_id));
    data.params = Some(params);
    let d = data.encode_json()?;

    let req_cli_builder = ClientBuilder::new()
        .user_agent(env!("CARGO_PKG_NAME"))
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(15))
        .connection_verbose(true)
        .build()
        .map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed ClientBuilder build {}", e),
            )
        })?;
    let resp = req_cli_builder
        .post(&u)
        .header(CONTENT_TYPE, "application/json")
        .body(d)
        .send()
        .await
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed ClientBuilder send {}", e)))?;
    let out = resp.bytes().await.map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed ClientBuilder bytes {}", e),
        )
    })?;
    let out: Vec<u8> = out.into();

    serde_json::from_slice(&out)
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed avm.getTxStatus '{}'", e)))
}

/// e.g., "avm.getBalance" on "http://[ADDR]:9650" and "/ext/bc/X" path.
/// ref. <https://docs.avax.network/build/avalanchego-apis/x-chain#avmgetbalance>
pub async fn get_balance(http_rpc: &str, xaddr: &str) -> io::Result<avm::GetBalanceResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/bc/X")
        } else {
            format!("{scheme}://{host}/ext/bc/X")
        }
    } else {
        format!("http://{host}/ext/bc/X")
    };
    log::info!("getting balance via {u} for {xaddr}");

    let mut data = jsonrpc::Request::default();
    data.method = String::from("avm.getBalance");
    let mut params = HashMap::new();
    params.insert(String::from("assetID"), String::from("AVAX"));
    params.insert(String::from("address"), xaddr.to_string());
    data.params = Some(params);
    let d = data.encode_json()?;

    let req_cli_builder = ClientBuilder::new()
        .user_agent(env!("CARGO_PKG_NAME"))
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(15))
        .connection_verbose(true)
        .build()
        .map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed ClientBuilder build {}", e),
            )
        })?;
    let resp = req_cli_builder
        .post(&u)
        .header(CONTENT_TYPE, "application/json")
        .body(d)
        .send()
        .await
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed ClientBuilder send {}", e)))?;
    let out = resp.bytes().await.map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed ClientBuilder bytes {}", e),
        )
    })?;
    let out: Vec<u8> = out.into();

    serde_json::from_slice(&out)
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed avm.getBalance '{}'", e)))
}

/// e.g., "avm.getAssetDescription".
/// ref. <https://docs.avax.network/build/avalanchego-apis/x-chain/#avmgetassetdescription>
pub async fn get_asset_description(
    http_rpc: &str,
    asset_id: &str,
) -> io::Result<avm::GetAssetDescriptionResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/bc/X")
        } else {
            format!("{scheme}://{host}/ext/bc/X")
        }
    } else {
        format!("http://{host}/ext/bc/X")
    };
    log::info!("getting asset description via {u}");

    let mut data = jsonrpc::Request::default();
    data.method = String::from("avm.getAssetDescription");
    let mut params = HashMap::new();
    params.insert(String::from("assetID"), String::from(asset_id));
    data.params = Some(params);
    let d = data.encode_json()?;

    let req_cli_builder = ClientBuilder::new()
        .user_agent(env!("CARGO_PKG_NAME"))
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(15))
        .connection_verbose(true)
        .build()
        .map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed ClientBuilder build {}", e),
            )
        })?;
    let resp = req_cli_builder
        .post(&u)
        .header(CONTENT_TYPE, "application/json")
        .body(d)
        .send()
        .await
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed ClientBuilder send {}", e)))?;
    let out = resp.bytes().await.map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed ClientBuilder bytes {}", e),
        )
    })?;
    let out: Vec<u8> = out.into();

    serde_json::from_slice(&out).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed avm.getAssetDescription '{}'", e),
        )
    })
}

/// e.g., "avm.getUTXOs" on "http://[ADDR]:9650" and "/ext/bc/X" path.
/// TODO: support paginated calls
/// ref. <https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgetutxos>
pub async fn get_utxos(http_rpc: &str, xaddr: &str) -> io::Result<avm::GetUtxosResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/bc/X")
        } else {
            format!("{scheme}://{host}/ext/bc/X")
        }
    } else {
        format!("http://{host}/ext/bc/X")
    };
    log::info!("getting UTXOs via {u} for {xaddr}");

    let mut data = avm::GetUtxosRequest::default();
    data.method = String::from("avm.getUTXOs");
    let params = avm::GetUtxosParams {
        addresses: vec![xaddr.to_string()],
        limit: 1024,
        encoding: String::from("hex"), // don't use "cb58"
    };
    data.params = Some(params);
    let d = data.encode_json()?;

    let req_cli_builder = ClientBuilder::new()
        .user_agent(env!("CARGO_PKG_NAME"))
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(15))
        .connection_verbose(true)
        .build()
        .map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed ClientBuilder build {}", e),
            )
        })?;
    let resp = req_cli_builder
        .post(&u)
        .header(CONTENT_TYPE, "application/json")
        .body(d)
        .send()
        .await
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed ClientBuilder send {}", e)))?;
    let out = resp.bytes().await.map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed ClientBuilder bytes {}", e),
        )
    })?;
    let out: Vec<u8> = out.into();

    serde_json::from_slice(&out)
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed avm.getUTXOs '{}'", e)))
}

/// e.g., "avm.issueStopVertex" on "http://[ADDR]:9650" and "/ext/bc/X" path.
/// Issue itself is asynchronous, so the internal error is not exposed!
pub async fn issue_stop_vertex(http_rpc: &str) -> io::Result<()> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/bc/X")
        } else {
            format!("{scheme}://{host}/ext/bc/X")
        }
    } else {
        format!("http://{host}/ext/bc/X")
    };
    log::info!("issuing a stop vertex transaction via {u}");

    let mut data = avm::IssueStopVertexRequest::default();
    data.method = String::from("avm.issueStopVertex");
    let params = avm::IssueStopVertexParams {};
    data.params = Some(params);
    let d = data.encode_json()?;

    let req_cli_builder = ClientBuilder::new()
        .user_agent(env!("CARGO_PKG_NAME"))
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(15))
        .connection_verbose(true)
        .build()
        .map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed ClientBuilder build {}", e),
            )
        })?;
    let resp = req_cli_builder
        .post(&u)
        .header(CONTENT_TYPE, "application/json")
        .body(d)
        .send()
        .await
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed ClientBuilder send {}", e)))?;

    if !resp.status().is_success() {
        return Err(Error::new(
            ErrorKind::Other,
            format!("status code non-success {}", resp.status()),
        ));
    }

    Ok(())
}
