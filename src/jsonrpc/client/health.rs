use std::{
    io::{self, Error, ErrorKind},
    sync::Arc,
};

use crate::jsonrpc::health;

/// "If a single piece of data must be accessible from more than one task
/// concurrently, then it must be shared using synchronization primitives such as Arc."
/// ref. <https://tokio.rs/tokio/tutorial/spawning>
pub async fn check(http_rpc: Arc<String>, liveness: bool) -> io::Result<health::Response> {
    let url_path = {
        if liveness {
            "ext/health/liveness"
        } else {
            "ext/health"
        }
    };
    let joined = http_manager::join_uri(http_rpc.as_str(), url_path)?;
    log::info!("checking for {:?}", joined);

    let rb = http_manager::get_non_tls(http_rpc.as_str(), url_path).await?;

    serde_json::from_slice(&rb)
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed health '{}'", e)))
}

pub async fn spawn_check(http_rpc: &str, liveness: bool) -> io::Result<health::Response> {
    let ep_arc = Arc::new(http_rpc.to_string());
    tokio::spawn(async move { check(ep_arc, liveness).await })
        .await
        .expect("failed spawn await")
}
