use std::{
    collections::HashMap,
    io::{self, Error, ErrorKind},
    time::Duration,
};

use crate::{
    jsonrpc::{self, info},
    utils,
};
use reqwest::{header::CONTENT_TYPE, ClientBuilder};

/// e.g., "info.getNetworkName".
/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetnetworkname>
pub async fn get_network_name(http_rpc: &str) -> io::Result<info::GetNetworkNameResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/info")
        } else {
            format!("{scheme}://{host}/ext/info")
        }
    } else {
        format!("http://{host}/ext/info")
    };
    log::info!("getting network name for {u}");

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.getNetworkName");
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
            format!("failed info.getNetworkName '{}'", e),
        )
    })
}

/// e.g., "info.getNetworkID".
/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetnetworkid>
pub async fn get_network_id(http_rpc: &str) -> io::Result<info::GetNetworkIdResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/info")
        } else {
            format!("{scheme}://{host}/ext/info")
        }
    } else {
        format!("http://{host}/ext/info")
    };
    log::info!("getting network Id for {u}");

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.getNetworkID");
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
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/info")
        } else {
            format!("{scheme}://{host}/ext/info")
        }
    } else {
        format!("http://{host}/ext/info")
    };
    log::info!("getting blockchain Id for {u}");

    let mut data = jsonrpc::Request::default();
    data.method = String::from("info.getBlockchainID");

    let mut params = HashMap::new();
    params.insert(String::from("alias"), String::from(chain_alias));
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
            format!("failed info.getBlockchainID '{}'", e),
        )
    })
}

/// e.g., "info.getNodeID".
/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetnodeid>
pub async fn get_node_id(http_rpc: &str) -> io::Result<info::GetNodeIdResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/info")
        } else {
            format!("{scheme}://{host}/ext/info")
        }
    } else {
        format!("http://{host}/ext/info")
    };
    log::info!("getting node Id for {u}");

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.getNodeID");
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
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed info.getNodeID '{}'", e)))
}

/// e.g., "info.getNodeVersion".
/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetnodeversion>
pub async fn get_node_version(http_rpc: &str) -> io::Result<info::GetNodeVersionResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/info")
        } else {
            format!("{scheme}://{host}/ext/info")
        }
    } else {
        format!("http://{host}/ext/info")
    };
    log::info!("getting node version for {u}");

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.getNodeVersion");
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
            format!("failed info.getNodeVersion '{}'", e),
        )
    })
}

/// e.g., "info.getVMs".
/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetvms>
pub async fn get_vms(http_rpc: &str) -> io::Result<info::GetVmsResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/info")
        } else {
            format!("{scheme}://{host}/ext/info")
        }
    } else {
        format!("http://{host}/ext/info")
    };
    log::info!("getting VMs for {u}");

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.getVMs");
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
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed info.getVMs '{}'", e)))
}

/// e.g., "info.isBootstrapped".
/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infoisbootstrapped>
pub async fn is_bootstrapped(http_rpc: &str) -> io::Result<info::IsBootstrappedResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/info")
        } else {
            format!("{scheme}://{host}/ext/info")
        }
    } else {
        format!("http://{host}/ext/info")
    };
    log::info!("getting bootstrapped for {u}");

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.isBootstrapped");
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
            format!("failed info.isBootstrapped '{}'", e),
        )
    })
}

/// e.g., "info.getTxFee".
/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogettxfee>
/// ref. "genesi/genesis_mainnet.go" requires 1 * units::AVAX for create_subnet_tx_fee/create_blockchain_tx_fee
/// ref. "genesi/genesis_fuji/local.go" requires 100 * units::MILLI_AVAX for create_subnet_tx_fee/create_blockchain_tx_fee
pub async fn get_tx_fee(http_rpc: &str) -> io::Result<info::GetTxFeeResponse> {
    log::info!("getting tx fee for {}", http_rpc);

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("info.getTxFee");
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
        .post(format!("{http_rpc}/ext/info").as_str())
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
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed info.getTxFee '{}'", e)))
}
