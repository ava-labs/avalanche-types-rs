use crate::{
    proto::rpcdb::{self, database_client::DatabaseClient},
    subnet::rpc::database::{self, errors::DatabaseError},
};

use std::io::{Error, ErrorKind, Result};

use num_traits::FromPrimitive;
use tonic::transport::Channel;

use crate::subnet::rpc::database::iterator::BoxedIterator;

/// Iterator iterates over a database's key/value pairs.
///
/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/database#Iterator>
pub struct Iterator {
    id: u64,
    /// List of PutRequests.
    data: Vec<rpcdb::PutRequest>,
    error: Option<Error>,
    db: DatabaseClient<Channel>,
}

/// Iterator iterates over a rpcdb database's key/value pairs.
///
/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/database#Iterator>
impl Iterator {
    pub fn new(db: DatabaseClient<Channel>, id: u64) -> BoxedIterator {
        Box::new(Self {
            id,
            data: vec![],
            error: None,
            db,
        })
    }
}

#[tonic::async_trait]
impl database::iterator::Iterator for Iterator {
    /// Implements the [`crate::subnet::rpc::database::Iterator`] trait.
    async fn next(&mut self) -> Result<bool> {
        // Short-circuit and set an error if the underlying database has been closed
        let mut db = self.db.clone();
        // TODO: handle db closed
        if self.data.len() > 1 {
            self.data.drain(0..1);
            return Ok(true);
        }

        let resp = db
            .iterator_next(rpcdb::IteratorNextRequest { id: self.id })
            .await;

        if resp.is_err() {
            self.error = Some(Error::new(
                ErrorKind::Other,
                format!("iterator next failed: {}", resp.unwrap_err()),
            ));
            return Ok(false);
        }
        self.data = resp.unwrap().into_inner().data;

        Ok(self.data.len() > 0)
    }

    /// Implements the [`crate::subnet::rpc::database::Iterator`] trait.
    async fn error(&mut self) -> Result<()> {
        let mut db = self.db.clone();
        if let Some(err) = &self.error {
            return Err(Error::new(err.kind(), err.to_string()));
        }

        let resp = db
            .iterator_error(rpcdb::IteratorErrorRequest { id: self.id })
            .await;
        if resp.is_err() {
            self.error = Some(Error::new(
                ErrorKind::Other,
                format!("iterator next failed: {}", resp.unwrap_err()),
            ));
        } else {
            let err = DatabaseError::from_i32(resp.unwrap().into_inner().err);
            match err {
                Some(DatabaseError::None) => {}
                Some(DatabaseError::Closed) => {
                    self.error = Some(Error::new(ErrorKind::Other, "database closed"))
                }
                Some(DatabaseError::NotFound) => {
                    self.error = Some(Error::new(ErrorKind::NotFound, "not found"))
                }
                _ => self.error = Some(Error::new(ErrorKind::Other, "unexpected database error")),
            }
        }
        if let Some(err) = &self.error {
            return Err(Error::new(err.kind(), err.to_string()));
        }
        Ok(())
    }

    /// Implements the [`crate::subnet::rpc::database::Iterator`] trait.
    async fn key(&self) -> Result<&[u8]> {
        if self.data.is_empty() {
            return Ok(&[]);
        }
        Ok(&self.data[0].key)
    }

    /// Implements the [`crate::subnet::rpc::database::Iterator`] trait.
    async fn value(&self) -> Result<&[u8]> {
        if self.data.is_empty() {
            return Ok(&[]);
        }
        Ok(&self.data[0].value)
    }

    /// Implements the [`crate::subnet::rpc::database::Iterator`] trait.
    async fn release(&mut self) {
        let mut db = self.db.clone();

        let resp = db
            .iterator_release(rpcdb::IteratorReleaseRequest { id: self.id })
            .await;
        if resp.is_err() {
            self.error = Some(Error::new(
                ErrorKind::Other,
                format!("iterator release failed: {}", resp.unwrap_err()),
            ));
        } else {
            let err = DatabaseError::from_i32(resp.unwrap().into_inner().err);
            match err {
                Some(DatabaseError::None) => {}
                Some(DatabaseError::Closed) => {
                    self.error = Some(Error::new(ErrorKind::Other, "database closed"))
                }
                Some(DatabaseError::NotFound) => {
                    self.error = Some(Error::new(ErrorKind::NotFound, "not found"))
                }
                _ => self.error = Some(Error::new(ErrorKind::Other, "unexpected database error")),
            }
        }
    }
}
