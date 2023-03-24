use std::io::Result;

use crate::{
    ids::Id,
    subnet::rpc::{consensus::snowman, snow::engine::common::vm::CommonVm},
};

/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/engine/snowman/block#ChainVm>
#[tonic::async_trait]
pub trait ChainVm: CommonVm + Getter + Parser {
    type Block: snowman::Block;

    /// Attempt to create a new block from ChainVm data
    /// Returns either a block or an error
    async fn build_block(&self) -> Result<<Self as ChainVm>::Block>;

    /// Issues a transaction to the chain
    async fn issue_tx(&self) -> Result<<Self as ChainVm>::Block>;

    /// Notify the Vm of the currently preferred block.
    async fn set_preference(&self, id: Id) -> Result<()>;

    /// Returns the ID of the last accepted block.
    /// If no blocks have been accepted, this should return the genesis block
    async fn last_accepted(&self) -> Result<Id>;
}

/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/engine/snowman/block#Getter>
#[tonic::async_trait]
pub trait Getter {
    type Block: snowman::Block;

    async fn get_block(&self, id: Id) -> Result<<Self as Getter>::Block>;
}

/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/engine/snowman/block#Parser>
#[tonic::async_trait]
pub trait Parser {
    type Block: snowman::Block;

    async fn parse_block(&self, bytes: &[u8]) -> Result<<Self as Parser>::Block>;
}
