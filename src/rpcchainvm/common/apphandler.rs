use std::io::Result;

use crate::ids;
use chrono::{DateTime, Utc};

/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/engine/common#AppHandler
#[tonic::async_trait]
pub trait AppHandler {
    async fn app_request(
        &self,
        node_id: &ids::node::Id,
        request_id: u32,
        deadline: DateTime<Utc>,
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
