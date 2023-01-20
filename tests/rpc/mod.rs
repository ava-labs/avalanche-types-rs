mod common;
mod database;
mod shutdown;

use crate::rpc::common::*;
use avalanche_types::subnet::rpc::http::{
    client::Client as HttpClient, server::Server as HttpServer,
};
use jsonrpc_core::Response as JsonResp;
use tokio::net::TcpListener;
use tonic::transport::Endpoint;

#[tokio::test]
async fn test_http_service() {
    let mut handler = jsonrpc_core::IoHandler::new();
    handler.add_method("foo", |_params: jsonrpc_core::Params| async move {
        Ok(jsonrpc_core::Value::String(format!("Hello, from foo")))
    });

    handler.add_method("bar", |params: jsonrpc_core::Params| async move {
        let params: HttpBarParams = params.parse().unwrap();

        Ok(jsonrpc_core::Value::String(format!(
            "Hello, {}, from bar",
            params.name
        )))
    });

    let http_server = HttpServer::new(handler);
    let listener = TcpListener::bind("127.0.0.1:1234").await.unwrap();

    tokio::spawn(async move {
        serve_test_http_server(http_server, listener).await.unwrap();
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let client_conn = Endpoint::from_static("http://127.0.0.1:1234")
        .connect()
        .await
        .unwrap();

    let mut client = HttpClient::new(client_conn);

    let foo_request = generate_http_request("foo", "http://127.0.0.1:1234", &[]);
    let foo_resp = client.serve_http_simple(foo_request).await;
    assert!(!foo_resp.is_err());
    let foo_resp = foo_resp.unwrap();

    assert!(foo_resp.status().is_success());

    let json_str = std::str::from_utf8(foo_resp.body());
    assert!(json_str.is_ok());
    let foo_json_resp = JsonResp::from_json(json_str.unwrap()).unwrap();

    let foo_output: jsonrpc_core::Output = match foo_json_resp {
        JsonResp::Single(val) => val,
        JsonResp::Batch(_) => panic!("Test should return single output"),
    };

    match foo_output {
        jsonrpc_core::Output::Success(_) => {}
        jsonrpc_core::Output::Failure(f) => panic!("inner resp invalid: {}", f.error),
    }

    let bar_request = generate_http_request("bar", "http://127.0.0.1:1234", &["John"]);
    let bar_resp = client.serve_http_simple(bar_request).await;
    assert!(!bar_resp.is_err());
    let bar_resp = bar_resp.unwrap();

    assert!(bar_resp.status().is_success());

    let json_str = std::str::from_utf8(bar_resp.body());
    assert!(json_str.is_ok());
    let bar_json_resp = JsonResp::from_json(json_str.unwrap()).unwrap();

    let bar_output: jsonrpc_core::Output = match bar_json_resp {
        JsonResp::Single(val) => val,
        JsonResp::Batch(_) => panic!("Test should return single output"),
    };

    match bar_output {
        jsonrpc_core::Output::Success(_) => {}
        jsonrpc_core::Output::Failure(f) => panic!("inner resp invalid: {}", f.error),
    }
}
