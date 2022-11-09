use std::sync::Arc;

use crate::{
    proto::pb::{
        self,
        google::protobuf::Empty,
        rpcdb::{
            CloseRequest, CloseResponse, CompactRequest, CompactResponse, DeleteRequest,
            DeleteResponse, GetRequest, GetResponse, HasRequest, HasResponse, HealthCheckResponse,
            IteratorErrorRequest, IteratorErrorResponse, IteratorNextRequest, IteratorNextResponse,
            IteratorReleaseRequest, IteratorReleaseResponse, NewIteratorWithStartAndPrefixRequest,
            NewIteratorWithStartAndPrefixResponse, PutRequest, PutResponse, WriteBatchRequest,
            WriteBatchResponse,
        },
    },
    subnet::rpc::database::{rpcdb::error_to_error_code, DatabaseError},
};
use prost::bytes::Bytes;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};
use zerocopy::AsBytes;

pub struct Server {
    pub inner: Arc<RwLock<Box<dyn crate::subnet::rpc::database::Database + Send + Sync>>>,
}

/// A gRPC server which wraps a subnet::rpc::database::Database impl allowing client control over over RPC.
impl Server {
    pub fn new(
        db: Box<dyn crate::subnet::rpc::database::Database + Send + Sync>,
    ) -> impl pb::rpcdb::database_server::Database {
        Server {
            inner: Arc::new(RwLock::new(db)),
        }
    }
}

#[tonic::async_trait]
impl pb::rpcdb::database_server::Database for Server {
    async fn has(&self, request: Request<HasRequest>) -> Result<Response<HasResponse>, Status> {
        let req = request.into_inner();
        let db = self.inner.read().await;

        match db.has(req.key.as_bytes()).await {
            Ok(has) => Ok(Response::new(HasResponse {
                has,
                err: DatabaseError::None as u32,
            })),
            Err(e) => Ok(Response::new(HasResponse {
                has: false,
                err: error_to_error_code(&e.to_string()).unwrap(),
            })),
        }
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let req = request.into_inner();
        let db = self.inner.read().await;

        match db.get(req.key.as_bytes()).await {
            Ok(resp) => Ok(Response::new(GetResponse {
                value: Bytes::from(resp),
                err: DatabaseError::None as u32,
            })),
            Err(e) => Ok(Response::new(GetResponse {
                value: Bytes::from(""),
                err: error_to_error_code(&e.to_string()).unwrap(),
            })),
        }
    }

    async fn put(&self, request: Request<PutRequest>) -> Result<Response<PutResponse>, Status> {
        let req = request.into_inner();
        let mut db = self.inner.write().await;

        match db.put(req.key.as_bytes(), req.value.as_bytes()).await {
            Ok(_) => Ok(Response::new(PutResponse {
                err: DatabaseError::None as u32,
            })),
            Err(e) => Ok(Response::new(PutResponse {
                err: error_to_error_code(&e.to_string()).unwrap(),
            })),
        }
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let req = request.into_inner();
        let mut db = self.inner.write().await;

        match db.delete(req.key.as_bytes()).await {
            Ok(_) => Ok(Response::new(DeleteResponse {
                err: DatabaseError::None as u32,
            })),
            Err(e) => Ok(Response::new(DeleteResponse {
                err: error_to_error_code(&e.to_string()).unwrap(),
            })),
        }
    }

    async fn compact(
        &self,
        _request: Request<CompactRequest>,
    ) -> Result<Response<CompactResponse>, Status> {
        Err(Status::unimplemented("compact"))
    }

    async fn close(
        &self,
        _request: Request<CloseRequest>,
    ) -> Result<Response<CloseResponse>, Status> {
        let db = self.inner.read().await;

        match db.close().await {
            Ok(_) => Ok(Response::new(CloseResponse {
                err: DatabaseError::None as u32,
            })),
            Err(e) => Ok(Response::new(CloseResponse {
                err: error_to_error_code(&e.to_string()).unwrap(),
            })),
        }
    }

    async fn health_check(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        let db = self.inner.read().await;

        match db.health_check().await {
            Ok(health) => match serde_json::to_string(&health) {
                Ok(details) => Ok(Response::new(HealthCheckResponse {
                    details: Bytes::from(details),
                })),
                Err(e) => Err(tonic::Status::unknown(e.to_string())),
            },
            Err(e) => Err(tonic::Status::unknown(e.to_string())),
        }
    }

    async fn write_batch(
        &self,
        _request: Request<WriteBatchRequest>,
    ) -> Result<Response<WriteBatchResponse>, Status> {
        Err(Status::unimplemented("write batch"))
    }

    async fn new_iterator_with_start_and_prefix(
        &self,
        _request: Request<NewIteratorWithStartAndPrefixRequest>,
    ) -> Result<Response<NewIteratorWithStartAndPrefixResponse>, Status> {
        Err(Status::unimplemented("new iterator with start and prefix"))
    }

    async fn iterator_next(
        &self,
        _request: Request<IteratorNextRequest>,
    ) -> Result<Response<IteratorNextResponse>, Status> {
        Err(Status::unimplemented("iterator next"))
    }

    async fn iterator_error(
        &self,
        _request: Request<IteratorErrorRequest>,
    ) -> Result<Response<IteratorErrorResponse>, Status> {
        Err(Status::unimplemented("iterator error"))
    }

    async fn iterator_release(
        &self,
        _request: Request<IteratorReleaseRequest>,
    ) -> Result<Response<IteratorReleaseResponse>, Status> {
        Err(Status::unimplemented("iterator release"))
    }
}
