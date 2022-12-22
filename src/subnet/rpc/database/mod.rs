pub mod corruptabledb;
pub mod errors;
pub mod manager;
pub mod memdb;
pub mod rpcdb;

use std::io::Result;

use num_derive::{FromPrimitive, ToPrimitive};

use crate::subnet::rpc::health::Checkable;

#[tonic::async_trait]
pub trait Closer {
    async fn close(&self) -> Result<()>;
}

#[tonic::async_trait]
pub trait Database: CloneBox + KeyValueReaderWriterDeleter + Closer + Checkable {}

/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/database#KeyValueReaderWriterDeleter>
#[tonic::async_trait]
pub trait KeyValueReaderWriterDeleter {
    async fn has(&self, key: &[u8]) -> Result<bool>;
    async fn get(&self, key: &[u8]) -> Result<Vec<u8>>;
    async fn put(&mut self, key: &[u8], value: &[u8]) -> Result<()>;
    async fn delete(&mut self, key: &[u8]) -> Result<()>;
}

pub trait CloneBox {
    /// Returns a Boxed clone of the underlying Database.
    fn clone_box(&self) -> Box<dyn Database + Send + Sync>;
}

impl<T> CloneBox for T
where
    T: 'static + Database + Clone + Send + Sync,
{
    fn clone_box(&self) -> Box<dyn Database + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Database + Send + Sync> {
    fn clone(&self) -> Box<dyn Database + Send + Sync> {
        self.clone_box()
    }
}

#[tonic::async_trait]
pub trait VersionedDatabase {
    async fn close(&mut self) -> Result<()>;
}

/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/database#ErrClosed>
#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum DatabaseError {
    None = 0,
    Closed,
    NotFound,
}

#[tokio::test]
async fn clone_box_test() {
    // create box and mutate underlying hashmap
    let mut db = memdb::Database::new();
    let resp = db.put("foo".as_bytes(), "bar".as_bytes()).await;
    assert!(!resp.is_err());

    // clone and mutate
    let mut cloned_db = db.clone();
    let resp = cloned_db.delete("foo".as_bytes()).await;
    assert!(!resp.is_err());

    // verify mutation
    let resp = cloned_db.get("foo".as_bytes()).await;
    assert!(resp.is_err());
}
