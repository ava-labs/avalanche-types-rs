use crate::proto::pb::{
    self,
    google::protobuf::Empty,
    http::{HandleSimpleHttpRequest, HandleSimpleHttpResponse, HttpRequest},
};
use jsonrpc_core::MethodCall;
use prost::bytes::Bytes;
use tonic::{codegen::http, Status};

#[derive(Clone)]
pub struct Server {
    /// handler generated from create_handlers
    handler: jsonrpc_core::IoHandler,
}

impl Server {
    pub fn new(handler: jsonrpc_core::IoHandler) -> impl pb::http::http_server::Http {
        Server { handler }
    }
}

#[tonic::async_trait]
impl pb::http::http_server::Http for Server {
    /// handles http requests including websockets
    async fn handle(
        &self,
        _request: tonic::Request<HttpRequest>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        Err(tonic::Status::unimplemented("handle"))
    }

    /// handles http simple (non web-socket) requests
    async fn handle_simple(
        &self,
        request: tonic::Request<HandleSimpleHttpRequest>,
    ) -> Result<tonic::Response<HandleSimpleHttpResponse>, tonic::Status> {
        let request = request.into_inner();

        // TODO: this assumes JSON-RPC.
        let de_request: MethodCall = serde_json::from_slice(request.body.as_ref())
            .map_err(|e| tonic::Status::unknown(e.to_string()))?;

        let json_str = serde_json::to_string(&de_request)
            .map_err(|e| tonic::Status::unknown(format!("failed to serialize request: {}", e)))?;

        // pass HTTP body bytes from [HandleSimpleHttpRequest] to underlying IoHandler
        let response = self
            .handler
            .handle_request(&json_str)
            .await
            .ok_or_else(|| Status::internal("failed to get response from rpc handler"))?;

        Ok(tonic::Response::new(HandleSimpleHttpResponse {
            code: http::StatusCode::OK.as_u16() as i32,
            body: Bytes::from(response.into_bytes()),
            headers: vec![],
        }))
    }
}
