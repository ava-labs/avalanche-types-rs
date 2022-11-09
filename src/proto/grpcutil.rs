use std::time::Duration;

use crate::proto::pb::google::protobuf::Timestamp;
use chrono::{DateTime, Utc};
use tonic::transport::Server;

pub fn timestamp_from_time(dt: &DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    }
}

/// Sets the [`SETTINGS_MAX_CONCURRENT_STREAMS`][spec] option for HTTP2
/// connections.
///
/// Tonic default is no limit (`None`) which is the same as u32::MAX.
///
/// [spec]: https://http2.github.io/http2-spec/#SETTINGS_MAX_CONCURRENT_STREAMS
pub const DEFAULT_MAX_CONCURRENT_STREAMS: u32 = u32::MAX;

/// Sets a timeout for receiving an acknowledgement of the keepalive ping.
///
/// If the ping is not acknowledged within the timeout, the connection will be closed.
/// Does nothing if http2_keep_alive_interval is disabled.
///
/// Tonic default is 20 seconds.
pub const DEFAULT_SERVER_KEEP_ALIVE_TIMEOUT: Duration = Duration::from_secs(20);

/// Set whether HTTP2 Ping frames are enabled on accepted connections.
///
/// If `None` is specified, HTTP2 keepalive is disabled, otherwise the duration
/// specified will be the time interval between HTTP2 Ping frames.
/// The timeout for receiving an acknowledgement of the keepalive ping
/// can be set with [`Server::http2_keepalive_timeout`].
///
/// Tonic default is no HTTP2 keepalive (`None`)
/// Avalanche default is 2 hours.
pub const DEFAULT_SERVER_KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(2 * 60 * 60);

/// Set whether TCP keepalive messages are enabled on accepted connections.
///
/// If `None` is specified, keepalive is disabled, otherwise the duration
/// specified will be the time to remain idle before sending TCP keepalive
/// probes.
///
/// Default is no keepalive (`None`)
/// Avalanche default is 5 seconds.
pub const DEFAULT_SERVER_KEEP_ALIVE_MIN_TIME: Duration = Duration::from_secs(5);

/// Returns a  gRPC server with proper defaults.
pub fn default_server() -> Server {
    Server::builder()
        .max_concurrent_streams(DEFAULT_MAX_CONCURRENT_STREAMS)
        .http2_keepalive_timeout(Some(DEFAULT_SERVER_KEEP_ALIVE_TIMEOUT))
        .http2_keepalive_interval(Some(DEFAULT_SERVER_KEEP_ALIVE_INTERVAL))
        .tcp_keepalive(Some(DEFAULT_SERVER_KEEP_ALIVE_MIN_TIME))
}
