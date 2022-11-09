use std::io::{self, Error, ErrorKind};

use crate::{
    proto::pb::{
        google::protobuf::Empty,
        rpcdb::{
            database_client::DatabaseClient as RpcDbDatabaseClient, CloseRequest, DeleteRequest,
            GetRequest, PutRequest,
        },
    },
    subnet::rpc::database::{errors, DatabaseError},
};
use num_traits::FromPrimitive;
use prost::bytes::Bytes;
use tonic::transport::Channel;

#[derive(Clone)]
pub struct DatabaseClient {
    inner: RpcDbDatabaseClient<Channel>,
}

impl DatabaseClient {
    pub fn new(
        client_conn: Channel,
    ) -> Box<dyn crate::subnet::rpc::database::Database + Send + Sync> {
        Box::new(DatabaseClient {
            inner: RpcDbDatabaseClient::new(client_conn),
        })
    }
}

/// DatabaseClient is an implementation of Database that talks over RPC.
/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/database/rpcdb#DatabaseClient
impl crate::subnet::rpc::database::Database for DatabaseClient {}

#[tonic::async_trait]
impl crate::subnet::rpc::database::KeyValueReaderWriterDeleter for DatabaseClient {
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

        let err = DatabaseError::from_u32(resp.err);
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
        let err = DatabaseError::from_u32(resp.err);
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

        let err = DatabaseError::from_u32(resp.err);
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
        let err = DatabaseError::from_u32(resp.err);
        match err {
            Some(DatabaseError::None) => Ok(()),
            Some(DatabaseError::Closed) => Err(errors::database_closed()),
            Some(DatabaseError::NotFound) => Err(errors::not_found()),
            _ => Err(Error::new(ErrorKind::Other, "unexpected database error")),
        }
    }
}

#[tonic::async_trait]
impl crate::subnet::rpc::database::Closer for DatabaseClient {
    /// Attempts to close the database.
    async fn close(&self) -> io::Result<()> {
        let mut db = self.inner.clone();
        let resp = db
            .close(CloseRequest {})
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("close request failed: {:?}", e)))?
            .into_inner();

        let err = DatabaseError::from_u32(resp.err);
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
