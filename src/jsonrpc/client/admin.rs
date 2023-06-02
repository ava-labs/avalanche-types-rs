use std::time::Duration;

use reqwest::{header::CONTENT_TYPE, ClientBuilder};

use crate::{
    errors::{Error, Result},
    jsonrpc::admin::{ChainAliasParams, ChainAliasRequest, ChainAliasResponse},
    utils,
};

/// Set an alias for a chain.
pub async fn alias_chain(
    http_rpc: &str,
    chain: String,
    alias: String,
) -> Result<ChainAliasResponse> {
    let (scheme, host, port, _, _) =
        utils::urls::extract_scheme_host_port_path_chain_alias(http_rpc).map_err(|e| {
            Error::Other {
                message: format!("failed extract_scheme_host_port_path_chain_alias '{}'", e),
                retryable: false,
            }
        })?;

    let u = if let Some(scheme) = scheme {
        if let Some(port) = port {
            format!("{scheme}://{host}:{port}/ext/admin")
        } else {
            format!("{scheme}://{host}/ext/admin")
        }
    } else {
        format!("http://{host}/ext/admin")
    };
    log::info!("getting network name for {u}");

    let data = ChainAliasRequest {
        params: Some(ChainAliasParams { chain, alias }),
        ..Default::default()
    };

    let d = data.encode_json().map_err(|e| Error::Other {
        message: format!("failed encode_json '{}'", e),
        retryable: false,
    })?;

    let req_cli_builder = ClientBuilder::new()
        .user_agent(env!("CARGO_PKG_NAME"))
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(15))
        .connection_verbose(true)
        .build()
        .map_err(|e| {
            // TODO: check retryable
            Error::Other {
                message: format!("failed reqwest::ClientBuilder.build '{}'", e),
                retryable: false,
            }
        })?;

    let resp = req_cli_builder
        .post(&u)
        .header(CONTENT_TYPE, "application/json")
        .body(d)
        .send()
        .await
        .map_err(|e|
        // TODO: check retryable
        Error::API {
            message: format!("failed reqwest::Client.send '{}'", e),
            retryable: false,
        })?;

    let out = resp.bytes().await.map_err(|e| {
        // TODO: check retryable
        Error::Other {
            message: format!("failed reqwest response bytes '{}'", e),
            retryable: false,
        }
    })?;
    let out: Vec<u8> = out.into();

    let response: ChainAliasResponse = serde_json::from_slice(&out)
        .map_err(|e| Error::Other {
            message: format!("failed serde_json::from_slice '{}'", e),
            retryable: false,
        })
        .unwrap();

    Ok(response)
}
