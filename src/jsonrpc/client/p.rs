use std::{
    collections::HashMap,
    io::{self, Error, ErrorKind},
    time::Duration,
};

use crate::{
    ids,
    jsonrpc::{self, platformvm},
    utils,
};
use reqwest::{header::CONTENT_TYPE, ClientBuilder};

/// "platform.issueTx" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators>
pub async fn issue_tx(http_rpc: &str, tx: &str) -> io::Result<platformvm::IssueTxResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/P")
        } else {
            format!("{scheme}://{host}/ext/P")
        }
    } else {
        format!("http://{host}/ext/P")
    };
    log::info!("issuing a transaction via {u}");

    let mut data = platformvm::IssueTxRequest::default();
    data.method = String::from("platform.issueTx");
    let params = platformvm::IssueTxParams {
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
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed platform.issueTx '{}'", e)))
}

/// "platform.getTx" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/apis/avalanchego/apis/p-chain/#platformgettx>
pub async fn get_tx(http_rpc: &str, tx_id: &str) -> io::Result<platformvm::GetTxResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/P")
        } else {
            format!("{scheme}://{host}/ext/P")
        }
    } else {
        format!("http://{host}/ext/P")
    };
    log::info!("getting tx via {u}");

    let mut data = jsonrpc::Request::default();
    data.method = String::from("platform.getTx");
    let mut params = HashMap::new();
    params.insert(String::from("txID"), String::from(tx_id));
    params.insert(String::from("encoding"), String::from("json")); // TODO: use "hex"?
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
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed platform.getTx '{}'", e)))
}

/// "platform.getTxStatus" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/apis/avalanchego/apis/p-chain/#platformgettxstatus>
pub async fn get_tx_status(
    http_rpc: &str,
    tx_id: &str,
) -> io::Result<platformvm::GetTxStatusResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/P")
        } else {
            format!("{scheme}://{host}/ext/P")
        }
    } else {
        format!("http://{host}/ext/P")
    };
    log::info!("getting tx status via {u}");

    let mut data = jsonrpc::Request::default();
    data.method = String::from("platform.getTxStatus");
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

    serde_json::from_slice(&out).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed platform.getTxStatus '{}'", e),
        )
    })
}

/// "platform.getHeight" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetheight>
pub async fn get_height(http_rpc: &str) -> io::Result<platformvm::GetHeightResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/P")
        } else {
            format!("{scheme}://{host}/ext/P")
        }
    } else {
        format!("http://{host}/ext/P")
    };
    log::info!("getting height via {u}");

    let mut data = jsonrpc::Request::default();
    data.method = String::from("platform.getHeight");

    let params = HashMap::new();
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
            format!("failed platform.getHeight '{}'", e),
        )
    })
}

/// "platform.getBalance" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetbalance>
/// ref. <https://github.com/ava-labs/avalanchego/blob/45ec88151f8a0e3bca1d43fe902fd632c41cd956/vms/platformvm/service.go#L192-L194>
pub async fn get_balance(
    http_rpc: &str,
    paddr: &str,
) -> io::Result<platformvm::GetBalanceResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/P")
        } else {
            format!("{scheme}://{host}/ext/P")
        }
    } else {
        format!("http://{host}/ext/P")
    };
    log::info!("getting balance via {u} for {}", paddr);

    let mut data = jsonrpc::RequestWithParamsHashMapToArray::default();
    data.method = String::from("platform.getBalance");
    let mut params = HashMap::new();
    params.insert(String::from("addresses"), vec![paddr.to_string()]);
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
            format!("failed platform.getBalance '{}'", e),
        )
    })
}

/// "platform.getUTXOs" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetutxos>
pub async fn get_utxos(http_rpc: &str, paddr: &str) -> io::Result<platformvm::GetUtxosResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/P")
        } else {
            format!("{scheme}://{host}/ext/P")
        }
    } else {
        format!("http://{host}/ext/P")
    };
    log::info!("getting UTXOs via {u} for {}", paddr);

    let mut data = platformvm::GetUtxosRequest::default();
    data.method = String::from("platform.getUTXOs");
    let params = platformvm::GetUtxosParams {
        addresses: vec![paddr.to_string()],
        limit: 100,
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

    serde_json::from_slice(&out).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed platform.getUTXOs '{}'", e),
        )
    })
}

/// "platform.getCurrentValidators" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators>
/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/platformvm#ClientPermissionlessValidator>
pub async fn get_primary_network_validators(
    http_rpc: &str,
) -> io::Result<platformvm::GetCurrentValidatorsResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/P")
        } else {
            format!("{scheme}://{host}/ext/P")
        }
    } else {
        format!("http://{host}/ext/P")
    };
    log::info!("getting primary network validators via {u}");

    let mut data = jsonrpc::Request::default();
    data.method = String::from("platform.getCurrentValidators");
    let params = HashMap::new();
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
            format!("failed platform.getCurrentValidators '{}'", e),
        )
    })
}

/// "platform.getCurrentValidators" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators>
/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/platformvm#ClientPermissionlessValidator>
pub async fn get_subnet_validators(
    http_rpc: &str,
    subnet_id: &str,
) -> io::Result<platformvm::GetCurrentValidatorsResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/P")
        } else {
            format!("{scheme}://{host}/ext/P")
        }
    } else {
        format!("http://{host}/ext/P")
    };
    log::info!("getting subnet validators via {u} for {subnet_id}");

    let mut data = jsonrpc::Request::default();
    data.method = String::from("platform.getCurrentValidators");
    let mut params = HashMap::new();
    params.insert(String::from("subnetID"), subnet_id.to_string());
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
            format!("failed platform.getCurrentValidators '{}'", e),
        )
    })
}

/// "platform.getSubnets" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/apis/avalanchego/apis/p-chain#platformgetsubnets>
pub async fn get_subnets(
    http_rpc: &str,
    subnet_ids: Option<Vec<ids::Id>>,
) -> io::Result<platformvm::GetSubnetsResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/P")
        } else {
            format!("{scheme}://{host}/ext/P")
        }
    } else {
        format!("http://{host}/ext/P")
    };
    log::info!("getting subnets via {u}");

    let mut data = jsonrpc::RequestWithParamsHashMapToArray::default();
    data.method = String::from("platform.getSubnets");
    let mut ids = Vec::new();
    if let Some(ss) = &subnet_ids {
        for id in ss.iter() {
            ids.push(id.to_string());
        }
    }
    let mut params = HashMap::new();
    params.insert(String::from("ids"), ids);
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
            format!("failed platform.getSubnets '{}'", e),
        )
    })
}

/// "platform.getBlockchains" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/apis/avalanchego/apis/p-chain#platformgetblockchains>
pub async fn get_blockchains(http_rpc: &str) -> io::Result<platformvm::GetBlockchainsResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/P")
        } else {
            format!("{scheme}://{host}/ext/P")
        }
    } else {
        format!("http://{host}/ext/P")
    };
    log::info!("getting blockchain via {u}");

    let mut data = jsonrpc::Request::default();
    data.method = String::from("platform.getBlockchains");
    let params = HashMap::new();
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
            format!("failed platform.getBlockchains '{}'", e),
        )
    })
}

/// "platform.getBlockchainStatus" on "http://[ADDR]:9650" and "/ext/P" path.
/// ref. <https://docs.avax.network/apis/avalanchego/apis/p-chain#platformgetblockchainstatus>
pub async fn get_blockchain_status(
    http_rpc: &str,
    blockchain_id: ids::Id,
) -> io::Result<platformvm::GetBlockchainStatusResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc)?;
    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/P")
        } else {
            format!("{scheme}://{host}/ext/P")
        }
    } else {
        format!("http://{host}/ext/P")
    };
    log::info!("getting blockchain status via {u} for {blockchain_id}");

    let mut data = jsonrpc::Request::default();
    data.method = String::from("platform.getBlockchainStatus");
    let mut params = HashMap::new();
    params.insert(String::from("blockchainID"), blockchain_id.to_string());
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
            format!("failed platform.getBlockchainStatus '{}'", e),
        )
    })
}
