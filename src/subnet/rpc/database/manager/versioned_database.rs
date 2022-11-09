use std::io::Result;

use semver::Version;

/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/database/manager#VersionedDatabase
#[derive(Clone)]
pub struct VersionedDatabase {
    pub db: Box<dyn crate::subnet::rpc::database::Database + Send + Sync>,
    pub version: Version,
}

impl VersionedDatabase {
    pub fn new(
        db: Box<dyn crate::subnet::rpc::database::Database + Send + Sync>,
        version: Version,
    ) -> Self {
        Self { db, version }
    }
}

#[tonic::async_trait]
impl crate::subnet::rpc::database::VersionedDatabase for VersionedDatabase {
    async fn close(&mut self) -> Result<()> {
        let db = &self.db;
        match db.close().await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
