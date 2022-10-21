use std::io::Result;

use crate::{choices::status::Status, ids::Id};

/// ref.https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/consensus/snowman#Block
#[tonic::async_trait]
pub trait Block: Decidable + Initializer + StatusWriter + Sync + Send {
    /// Returns the bytes of this block.
    async fn bytes(&self) -> &[u8];

    /// Returns bytes from serde.
    async fn to_bytes(&self) -> Result<Vec<u8>>;

    /// Returns the height of the block in the chain.
    async fn height(&self) -> u64;

    /// Returns the creation timestamp of the block in the chain.
    async fn timestamp(&self) -> u64;

    /// Returns the ID of this block's parent.
    async fn parent(&self) -> Id;

    /// Returns error if the block can not be verified.
    async fn verify(&mut self) -> Result<()>;
}

/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/choices#Decidable
#[tonic::async_trait]
pub trait Decidable {
    /// Returns the ID of this block's parent.
    async fn id(&self) -> Id;

    /// Returns the current status.
    async fn status(&self) -> Status;

    /// Accepts this element.
    async fn accept(&mut self) -> Result<()>;

    /// Rejects this element.
    async fn reject(&mut self) -> Result<()>;
}

#[tonic::async_trait]
pub trait Initializer {
    /// Initializes the block.
    async fn init(&mut self, bytes: &[u8], status: Status) -> Result<()>;
}

#[tonic::async_trait]
pub trait StatusWriter {
    /// Sets the blocks status.
    async fn set_status(&mut self, status: Status);
}
