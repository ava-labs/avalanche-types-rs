//! Database manager.
pub mod versioned_database;

use std::{
    io::{self, Error, ErrorKind},
    sync::Arc,
};

use crate::subnet::rpc::database::manager::versioned_database::VersionedDatabase;
use tokio::sync::RwLock;

/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/database/manager#Manager>
#[tonic::async_trait]
pub trait Manager {
    async fn current(&self) -> io::Result<VersionedDatabase>;
    async fn previous(&self) -> Option<VersionedDatabase>;
    async fn close(&self) -> io::Result<()>;
}

#[derive(Clone)]
pub struct DatabaseManager {
    inner: Arc<RwLock<Vec<VersionedDatabase>>>,
}

impl DatabaseManager {
    /// Returns a database manager from a Vec of versioned database.
    pub fn new_from_databases(databases: Vec<VersionedDatabase>) -> Box<dyn Manager + Send + Sync> {
        Box::new(DatabaseManager {
            inner: Arc::new(RwLock::new(databases)),
        })
    }
}

#[tonic::async_trait]
impl Manager for DatabaseManager {
    /// Returns the database with the current database version.
    async fn current(&self) -> io::Result<VersionedDatabase> {
        let databases = self.inner.read().await;
        return Ok(databases[0].clone());
    }

    /// Returns the database prior to the current database and true if a
    // previous database exists.
    async fn previous(&self) -> Option<VersionedDatabase> {
        let databases = self.inner.read().await;

        if databases.len() < 2 {
            return None;
        }
        return Some(databases[1].clone());
    }

    /// Close all of the databases controlled by the manager.
    async fn close(&self) -> io::Result<()> {
        let dbs = self.inner.read().await;

        let mut errors = Vec::with_capacity(dbs.len());
        for db in dbs.iter() {
            let db = &db.db;
            match db.close().await {
                Ok(_) => continue,
                Err(e) => errors.push(e.to_string()),
            }
        }

        if !errors.is_empty() {
            return Err(Error::new(
                ErrorKind::Other,
                errors.first().unwrap().to_string(),
            ));
        }
        Ok(())
    }
}
