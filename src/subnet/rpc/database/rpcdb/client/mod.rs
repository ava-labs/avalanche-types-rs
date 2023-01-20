//! Database Client
pub mod iterator;

use std::io::{self, Error, ErrorKind};

use crate::{
    proto::{
        pb::{
            google::protobuf::Empty,
            rpcdb::{
                database_client::DatabaseClient as RpcDbDatabaseClient, CloseRequest,
                DeleteRequest, GetRequest, PutRequest,
            },
        },
        rpcdb::NewIteratorWithStartAndPrefixRequest,
    },
    subnet::rpc::database::{
        self,
        errors::{self, DatabaseError},
        iterator::BoxedIterator,
        BoxedDatabase,
    },
};
use num_traits::FromPrimitive;
use prost::bytes::Bytes;
use tonic::transport::Channel;

/// DatabaseClient is an implementation of [`crate::subnet::rpc::database::Database`] that talks over RPC.
///
/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/database/rpcdb#DatabaseClient>
#[derive(Clone)]
pub struct DatabaseClient {
    inner: RpcDbDatabaseClient<Channel>,
}

impl DatabaseClient {
    pub fn new(client_conn: Channel) -> BoxedDatabase {
        Box::new(Self {
            inner: RpcDbDatabaseClient::new(client_conn),
        })
    }
}

#[tonic::async_trait]
impl database::KeyValueReaderWriterDeleter for DatabaseClient {
    /// Attempts to return if the database has a key with the provided value.
    async fn has(&self, key: &[u8]) -> io::Result<bool> {
        let mut db = self.inner.clone();
        let resp = db
            .get(GetRequest {
                key: Bytes::from(key.to_owned()),
            })
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("has request failed: {:?}", e)))?
            .into_inner();

        let err = DatabaseError::from_i32(resp.err);
        match err {
            Some(DatabaseError::Closed) => Err(errors::database_closed()),
            Some(DatabaseError::NotFound) => Ok(false),
            Some(DatabaseError::None) => Ok(true),
            _ => Err(Error::new(ErrorKind::Other, "unexpected database error")),
        }
    }

    /// Attempts to return the value that was mapped to the key that was provided.
    async fn get(&self, key: &[u8]) -> io::Result<Vec<u8>> {
        let mut db = self.inner.clone();
        let resp = db
            .get(GetRequest {
                key: Bytes::from(key.to_owned()),
            })
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("get request failed: {:?}", e)))?
            .into_inner();

        log::debug!("get response: {:?}", resp);
        let err = DatabaseError::from_i32(resp.err);
        match err {
            Some(DatabaseError::None) => Ok(resp.value.to_vec()),
            Some(DatabaseError::Closed) => Err(errors::database_closed()),
            Some(DatabaseError::NotFound) => Err(errors::not_found()),
            _ => Err(Error::new(ErrorKind::Other, "unexpected database error")),
        }
    }

    /// Attempts to set the value this key maps to.
    async fn put(&mut self, key: &[u8], value: &[u8]) -> io::Result<()> {
        let mut db = self.inner.clone();
        let resp = db
            .put(PutRequest {
                key: Bytes::from(key.to_owned()),
                value: Bytes::from(value.to_owned()),
            })
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("put request failed: {:?}", e)))?
            .into_inner();

        let err = DatabaseError::from_i32(resp.err);
        match err {
            Some(DatabaseError::None) => Ok(()),
            Some(DatabaseError::Closed) => Err(errors::database_closed()),
            Some(DatabaseError::NotFound) => Err(errors::not_found()),
            _ => Err(Error::new(ErrorKind::Other, "unexpected database error")),
        }
    }

    /// Attempts to remove any mapping from the key.
    async fn delete(&mut self, key: &[u8]) -> io::Result<()> {
        let mut client = self.inner.clone();
        let resp = client
            .delete(DeleteRequest {
                key: Bytes::from(key.to_owned()),
            })
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("delete request failed: {:?}", e)))?
            .into_inner();
        let err = DatabaseError::from_i32(resp.err);
        match err {
            Some(DatabaseError::None) => Ok(()),
            Some(DatabaseError::Closed) => Err(errors::database_closed()),
            Some(DatabaseError::NotFound) => Err(errors::not_found()),
            _ => Err(Error::new(ErrorKind::Other, "unexpected database error")),
        }
    }
}

#[tonic::async_trait]
impl database::Closer for DatabaseClient {
    /// Attempts to close the database.
    async fn close(&self) -> io::Result<()> {
        let mut db = self.inner.clone();
        let resp = db
            .close(CloseRequest {})
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("close request failed: {:?}", e)))?
            .into_inner();

        let err = DatabaseError::from_i32(resp.err);
        match err {
            Some(DatabaseError::None) => Ok(()),
            Some(DatabaseError::Closed) => Err(errors::database_closed()),
            Some(DatabaseError::NotFound) => Err(errors::not_found()),
            _ => Err(Error::new(ErrorKind::Other, "unexpected database error")),
        }
    }
}

#[tonic::async_trait]
impl crate::subnet::rpc::health::Checkable for DatabaseClient {
    /// Attempts to perform a health check against the underlying database.
    async fn health_check(&self) -> io::Result<Vec<u8>> {
        let mut db = self.inner.clone();
        let resp = db
            .health_check(Empty {})
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("health check failed: {:?}", e)))?;

        Ok(resp.into_inner().details.to_vec())
    }
}

#[tonic::async_trait]
impl database::iterator::Iteratee for DatabaseClient {
    /// Implements the [`crate::subnet::rpc::database::Iteratee`] trait.
    async fn new_iterator(&self) -> io::Result<BoxedIterator> {
        self.new_iterator_with_start_and_prefix(&[], &[]).await
    }

    /// Implements the [`crate::subnet::rpc::database::Iteratee`] trait.
    async fn new_iterator_with_start(&self, start: &[u8]) -> io::Result<BoxedIterator> {
        self.new_iterator_with_start_and_prefix(start, &[]).await
    }

    /// Implements the [`crate::subnet::rpc::database::Iteratee`] trait.
    async fn new_iterator_with_prefix(&self, prefix: &[u8]) -> io::Result<BoxedIterator> {
        self.new_iterator_with_start_and_prefix(&[], prefix).await
    }

    /// Implements the [`crate::subnet::rpc::database::Iteratee`] trait.
    async fn new_iterator_with_start_and_prefix(
        &self,
        start: &[u8],
        prefix: &[u8],
    ) -> io::Result<BoxedIterator> {
        let mut db = self.inner.clone();
        match db
            .new_iterator_with_start_and_prefix(NewIteratorWithStartAndPrefixRequest {
                start: Bytes::from(start.to_owned()),
                prefix: Bytes::from(prefix.to_owned()),
            })
            .await
        {
            Ok(resp) => Ok(iterator::Iterator::new(
                self.inner.clone(),
                resp.into_inner().id,
            )),
            Err(e) => Ok(crate::subnet::rpc::database::nodb::Iterator::new(Some(
                Error::new(ErrorKind::Other, e),
            ))),
        }
    }
}

impl database::Database for DatabaseClient {}
