use std::{io::Result, time};

use crate::ids;
use avalanche_proto::{google::protobuf::Empty, vm::AppRequestMsg};
use tonic::{Request, Response};

/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/engine/common#AppHandler
#[tonic::async_trait]
pub trait AppHandler {
    async fn app_request(
        &self,
        node_id: &ids::node::Id,
        request_id: u32,
        deadline: time::Instant,
        request: &[u8],
    ) -> Result<()>;
    async fn app_request_failed(&self, node_id: &ids::node::Id, request_id: u32) -> Result<()>;
    async fn app_response(
        &self,
        node_id: &ids::node::Id,
        request_id: u32,
        response: &[u8],
    ) -> Result<()>;
    async fn app_gossip(&self, node_id: &ids::node::Id, msg: &[u8]) -> Result<()>;
}

/// gRPC Server AppHandler allows for conditional traits to be implemented for the gRPC
/// server service.
#[tonic::async_trait]
pub trait AppHandlerServer {
    async fn app_request(
        &self,
        _request: Request<AppRequestMsg>,
    ) -> std::result::Result<Response<Empty>, tonic::Status>;

    async fn app_request_failed(&self, node_id: &ids::node::Id, request_id: u32) -> Result<()>;
    async fn app_response(
        &self,
        node_id: &ids::node::Id,
        request_id: u32,
        response: &[u8],
    ) -> Result<()>;
    async fn app_gossip(&self, node_id: &ids::node::Id, msg: &[u8]) -> Result<()>;
}
