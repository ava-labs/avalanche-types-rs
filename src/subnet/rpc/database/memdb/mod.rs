//! Implements an in-memory database useful for testing.
//!
//!```rust
//! use avalanche_types::subnet::rpc::database::memdb::Database;
//!
//! let mut db = Database::new();
//! let resp = db.put("foo".as_bytes(), "bar".as_bytes()).await;
//! let resp = db.has("foo".as_bytes()).await;
//! assert_eq!(resp.unwrap(), true);
//! ```

use std::{
    collections::HashMap,
    io,
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
};

use super::errors;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Database {
    inner: Arc<RwLock<HashMap<Vec<u8>, Vec<u8>>>>,
    closed: Arc<AtomicBool>,
}

impl Database {
    pub fn new() -> Box<dyn crate::subnet::rpc::database::Database + Send + Sync> {
        let state: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        Box::new(Database {
            inner: Arc::new(RwLock::new(state)),
            closed: Arc::new(AtomicBool::new(false)),
        })
    }
}

/// Database is an ephemeral key-value store that implements the Database interface.
/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/database/memdb#Database>
impl crate::subnet::rpc::database::Database for Database {}

#[tonic::async_trait]
impl crate::subnet::rpc::database::KeyValueReaderWriterDeleter for Database {
    /// Attempts to return if the database has a key with the provided value.
    async fn has(&self, key: &[u8]) -> io::Result<bool> {
        let db = self.inner.read().await;
        match db.get(key) {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    /// Attempts to return the value that was mapped to the key that was provided.
    async fn get(&self, key: &[u8]) -> io::Result<Vec<u8>> {
        if self.closed.load(Ordering::Relaxed) {
            return Err(errors::database_closed());
        }

        let db = self.inner.read().await;
        match db.get(key) {
            Some(key) => Ok(key.to_vec()),
            None => Err(errors::not_found()),
        }
    }

    /// Attempts to set the value this key maps to.
    async fn put(&mut self, key: &[u8], value: &[u8]) -> io::Result<()> {
        if self.closed.load(Ordering::Relaxed) {
            return Err(errors::database_closed());
        }

        let mut db = self.inner.write().await;
        db.insert(key.to_vec(), value.to_vec());
        Ok(())
    }

    /// Attempts to remove any mapping from the key.
    async fn delete(&mut self, key: &[u8]) -> io::Result<()> {
        if self.closed.load(Ordering::Relaxed) {
            return Err(errors::database_closed());
        }

        let mut db = self.inner.write().await;
        db.remove(key);
        Ok(())
    }
}

#[tonic::async_trait]
impl crate::subnet::rpc::database::Closer for Database {
    /// Attempts to close the database.
    async fn close(&self) -> io::Result<()> {
        if self.closed.load(Ordering::Relaxed) {
            return Err(errors::database_closed());
        }

        self.closed.store(true, Ordering::Relaxed);
        Ok(())
    }
}

#[tonic::async_trait]
impl crate::subnet::rpc::health::Checkable for Database {
    /// Checks if the database has been closed.
    async fn health_check(&self) -> io::Result<Vec<u8>> {
        if self.closed.load(Ordering::Relaxed) {
            return Err(errors::database_closed());
        }
        Ok(vec![])
    }
}

#[tokio::test]
async fn test_memdb() {
    let mut db = Database::new();
    let _ = db.put("foo".as_bytes(), "bar".as_bytes()).await;
    let resp = db.get("notfound".as_bytes()).await;
    assert!(resp.is_err());
    assert_eq!(resp.err().unwrap().kind(), io::ErrorKind::NotFound);

    let mut db = Database::new();
    let _ = db.close().await;
    let resp = db.put("foo".as_bytes(), "bar".as_bytes()).await;
    assert!(resp.is_err());
    assert_eq!(resp.err().unwrap().to_string(), "database closed");

    let db = Database::new();
    let _ = db.close().await;
    let resp = db.get("foo".as_bytes()).await;
    print!("found {:?}", resp);
    assert!(resp.is_err());
    assert_eq!(resp.err().unwrap().to_string(), "database closed");

    let mut db = Database::new();
    let _ = db.put("foo".as_bytes(), "bar".as_bytes()).await;
    let resp = db.has("foo".as_bytes()).await;
    assert!(!resp.is_err());
    assert_eq!(resp.unwrap(), true);

    let mut db = Database::new();
    let _ = db.put("foo".as_bytes(), "bar".as_bytes()).await;
    let _ = db.delete("foo".as_bytes()).await;
    let resp = db.has("foo".as_bytes()).await;
    assert!(!resp.is_err());
    assert_eq!(resp.unwrap(), false);

    let db = Database::new();
    let resp = db.health_check().await;
    assert!(!resp.is_err());
    let _ = db.close().await;
    let resp = db.health_check().await;
    assert_eq!(resp.err().unwrap().to_string(), "database closed");
}
