pub mod corruptabledb;
pub mod iterator;
pub mod manager;
pub mod memdb;
pub mod nodb;
pub mod rpcdb;

use std::io::Result;

use crate::subnet::rpc::health::Checkable;

pub const MAX_BATCH_SIZE: usize = 128 * 1000;

#[tonic::async_trait]
pub trait Closer {
    async fn close(&self) -> Result<()>;
}

#[tonic::async_trait]
pub trait Database:
    CloneBox + KeyValueReaderWriterDeleter + Closer + Checkable + iterator::Iteratee
{
}

/// Helper type which defines a thread safe boxed Database interface.
pub type BoxedDatabase = Box<dyn Database + Send + Sync + 'static>;

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
    fn clone_box(&self) -> BoxedDatabase;
}

impl<T> CloneBox for T
where
    T: 'static + Database + Clone + Send + Sync,
{
    fn clone_box(&self) -> BoxedDatabase {
        Box::new(self.clone())
    }
}

impl Clone for BoxedDatabase {
    fn clone(&self) -> BoxedDatabase {
        self.clone_box()
    }
}

#[tonic::async_trait]
pub trait VersionedDatabase {
    async fn close(&mut self) -> Result<()>;
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
