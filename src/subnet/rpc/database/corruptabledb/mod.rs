use std::{
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use super::errors;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Database {
    db: Box<dyn crate::subnet::rpc::database::Database + Send + Sync>,
    corrupted: Arc<AtomicBool>,
    corrupted_error: Arc<RwLock<String>>,
}

impl Database {
    pub fn new(
        db: Box<dyn crate::subnet::rpc::database::Database + Send + Sync>,
    ) -> Box<dyn crate::subnet::rpc::database::Database + Send + Sync> {
        Box::new(Database {
            db,
            corrupted: Arc::new(AtomicBool::new(false)),
            corrupted_error: Arc::new(RwLock::new("".into())),
        })
    }
}

/// Database is a wrapper around Database it prevents any future calls
/// in case a corruption occurs.
/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/database/corruptabledb#Database
impl crate::subnet::rpc::database::Database for Database {}

#[tonic::async_trait]
impl crate::subnet::rpc::database::KeyValueReaderWriterDeleter for Database {
    /// Attempts to return if the database has a key with the provided value.
    async fn has(&self, key: &[u8]) -> io::Result<bool> {
        if self.corrupted.load(Ordering::Relaxed) {
            return Err(errors::from_string(
                self.corrupted_error.read().await.to_string(),
            ));
        }

        let db = &self.db;
        match db.get(key).await {
            Ok(_) => Ok(true),
            Err(err) => {
                let (is_corrupted, err) = errors::is_corruptible(err).await;
                if is_corrupted {
                    *self.corrupted_error.write().await = err.to_string();
                    self.corrupted.store(true, Ordering::Relaxed);
                    return Err(errors::from_string(
                        self.corrupted_error.read().await.to_string(),
                    ));
                }
                if errors::is_not_found(&err) {
                    return Ok(false);
                }
                return Err(err);
            }
        }
    }

    /// Attempts to return the value that was mapped to the key that was provided.
    async fn get(&self, key: &[u8]) -> io::Result<Vec<u8>> {
        if self.corrupted.load(Ordering::Relaxed) {
            return Err(errors::from_string(
                self.corrupted_error.read().await.to_string(),
            ));
        }

        let db = &self.db;
        match db.get(key).await {
            Ok(resp) => Ok(resp),
            Err(err) => {
                let (is_corrupted, err) = errors::is_corruptible(err).await;
                if is_corrupted {
                    *self.corrupted_error.write().await = err.to_string();
                    self.corrupted.store(true, Ordering::Relaxed);
                    return Err(errors::from_string(
                        self.corrupted_error.read().await.to_string(),
                    ));
                }
                return Err(err);
            }
        }
    }

    /// Attempts to set the value this key maps to.
    async fn put(&mut self, key: &[u8], value: &[u8]) -> io::Result<()> {
        if self.corrupted.load(Ordering::Relaxed) {
            return Err(errors::from_string(
                self.corrupted_error.read().await.to_string(),
            ));
        }

        let db = &mut self.db;
        match db.put(key, value).await {
            Ok(_) => Ok(()),
            Err(err) => {
                let (is_corrupted, err) = errors::is_corruptible(err).await;
                if is_corrupted {
                    *self.corrupted_error.write().await = err.to_string();
                    self.corrupted.store(true, Ordering::Relaxed);
                    return Err(errors::from_string(
                        self.corrupted_error.read().await.to_string(),
                    ));
                }
                return Err(err);
            }
        }
    }

    /// Attempts to remove any mapping from the key.
    async fn delete(&mut self, key: &[u8]) -> io::Result<()> {
        if self.corrupted.load(Ordering::Relaxed) {
            return Err(errors::from_string(
                self.corrupted_error.read().await.to_string(),
            ));
        }

        let db = &mut self.db;
        match db.delete(key).await {
            Ok(_) => Ok(()),
            Err(err) => {
                let (is_corrupted, err) = errors::is_corruptible(err).await;
                if is_corrupted {
                    *self.corrupted_error.write().await = err.to_string();
                    self.corrupted.store(true, Ordering::Relaxed);
                    return Err(errors::from_string(
                        self.corrupted_error.read().await.to_string(),
                    ));
                }
                return Err(err);
            }
        }
    }
}

#[tonic::async_trait]
impl crate::subnet::rpc::database::Closer for Database {
    /// Attempts to close the database.
    async fn close(&self) -> io::Result<()> {
        if self.corrupted.load(Ordering::Relaxed) {
            return Err(errors::from_string(
                self.corrupted_error.read().await.to_string(),
            ));
        }

        let db = &self.db;
        match db.close().await {
            Ok(_) => Ok(()),
            Err(err) => {
                let (is_corrupted, err) = errors::is_corruptible(err).await;
                if is_corrupted {
                    *self.corrupted_error.write().await = err.to_string();
                    self.corrupted.store(true, Ordering::Relaxed);
                    return Err(errors::from_string(
                        self.corrupted_error.read().await.to_string(),
                    ));
                }
                return Err(err);
            }
        }
    }
}

#[tonic::async_trait]
impl crate::subnet::rpc::health::Checkable for Database {
    /// Checks if the database has been closed.
    async fn health_check(&self) -> io::Result<Vec<u8>> {
        if self.corrupted.load(Ordering::Relaxed) {
            return Err(errors::from_string(
                self.corrupted_error.read().await.to_string(),
            ));
        }

        let db = &self.db;
        match db.health_check().await {
            Ok(resp) => Ok(resp),
            Err(err) => {
                let (is_corrupted, err) = errors::is_corruptible(err).await;
                if is_corrupted {
                    *self.corrupted_error.write().await = err.to_string();
                    self.corrupted.store(true, Ordering::Relaxed);
                    return Err(errors::from_string(
                        self.corrupted_error.read().await.to_string(),
                    ));
                }
                return Err(err);
            }
        }
    }
}
