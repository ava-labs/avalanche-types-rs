#[cfg(any(test, feature = "proto"))]
use std::io::{self, Error, ErrorKind};

use avalanche_proto::{
    grpcutil::default_server,
    http::http_server::{Http, HttpServer},
    rpcdb::database_server::{Database, DatabaseServer},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;

pub async fn serve_test_database<D: Database + 'static>(
    database: D,
    listener: TcpListener,
) -> io::Result<()>
where
    D: Database,
{
    default_server()
        .add_service(DatabaseServer::new(database))
        .serve_with_incoming(TcpListenerStream::new(listener))
        .await
        .map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed to serve test database service: {}", e),
            )
        })
}

pub async fn serve_test_http_server<H: Http + 'static>(
    http: H,
    listener: TcpListener,
) -> std::io::Result<()>
where
    H: avalanche_proto::http::http_server::Http,
{
    avalanche_proto::grpcutil::default_server()
        .add_service(HttpServer::new(http))
        .serve_with_incoming(TcpListenerStream::new(listener))
        .await
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("failed to serve test http service: {}", e),
            )
        })
}

pub fn generate_http_request(
    method_name: &str,
    address: &str,
    param: &[&str],
) -> http::Request<Vec<u8>> {
    let mut json_params = Vec::with_capacity(param.len());

    for i in 0..param.len() {
        json_params.push(serde_json::Value::from(param[i]))
    }

    let m = jsonrpc_core::MethodCall {
        jsonrpc: Some(jsonrpc_core::Version::V2),
        method: String::from(method_name),
        params: jsonrpc_core::Params::Array(json_params),
        id: jsonrpc_core::Id::Num(1),
    };

    let body = serde_json::to_vec(&m).unwrap();

    http::Request::builder()
        .method("POST")
        .uri(address)
        .header("Content-type", "application/json")
        .header("Content-length", body.len().to_string().as_str())
        .body(body)
        .unwrap()
}

#[derive(Serialize, Deserialize)]
pub struct HttpBarParams {
    pub name: String,
}
