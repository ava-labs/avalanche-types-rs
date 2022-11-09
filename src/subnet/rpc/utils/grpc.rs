use std::{
    convert::Infallible,
    io::{Error, ErrorKind, Result},
    net::SocketAddr,
};

use crate::proto;
use http::{Request, Response};
use hyper::Body;
use jsonrpc_core::futures::FutureExt;
use tokio::sync::broadcast::Receiver;
use tonic::{body::BoxBody, transport::NamedService};
use tower_service::Service;

/// gRPC server lifecycle manager.
pub struct Server {
    /// Waits for the broadcasted stop signal to shutdown gRPC server.
    pub stop_ch: Receiver<()>,

    /// Server address.
    pub addr: SocketAddr,
}

impl Server {
    pub fn new(addr: SocketAddr, stop_ch: Receiver<()>) -> Self {
        Self { stop_ch, addr }
    }
}

// TODO: add support for multiple services.
impl Server {
    /// Attempts to start a gRPC server for the provided service.
    pub fn serve<S>(mut self, svc: S) -> Result<()>
    where
        S: Service<Request<Body>, Response = Response<BoxBody>, Error = Infallible>
            + NamedService
            + Clone
            + Send
            + 'static,
        S::Future: Send + 'static,
    {
        tokio::spawn(async move {
            proto::grpcutil::default_server()
                .add_service(svc)
                .serve_with_shutdown(self.addr, self.stop_ch.recv().map(|_| ()))
                .await
                .map_err(|e| Error::new(ErrorKind::Other, format!("grpc server failed: {:?}", e)))
        });
        log::info!("gRPC server started: {}", self.addr);

        Ok(())
    }
}
