use std::io::Result;

use crate::{
    ids::Id,
    subnet::rpc::{consensus::snowman, snow::engine::common::vm::Vm},
};

/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/engine/snowman/block#ChainVm>
#[tonic::async_trait]
pub trait ChainVm: Vm + Getter + Parser {
    /// Attempt to create a new block from ChainVm data
    /// Returns either a block or an error
    async fn build_block(&self) -> Result<Box<dyn snowman::Block + Send + Sync>>;

    /// Issues a transaction to the chain
    async fn issue_tx(&self) -> Result<Box<dyn snowman::Block + Send + Sync>>;

    /// Notify the Vm of the currently preferred block.
    async fn set_preference(&self, id: Id) -> Result<()>;

    /// Returns the ID of the last accepted block.
    /// If no blocks have been accepted, this should return the genesis block
    async fn last_accepted(&self) -> Result<Id>;
}

/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/engine/snowman/block#Getter>
#[tonic::async_trait]
pub trait Getter {
    async fn get_block(&self, id: Id) -> Result<Box<dyn snowman::Block + Send + Sync>>;
}

/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/engine/snowman/block#Parser>
#[tonic::async_trait]
pub trait Parser {
    async fn parse_block(&self, bytes: &[u8]) -> Result<Box<dyn snowman::Block + Send + Sync>>;
}
