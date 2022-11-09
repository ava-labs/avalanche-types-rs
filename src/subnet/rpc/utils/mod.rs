pub mod grpc;

use std::net::{SocketAddr, UdpSocket};

/// Returns a localhost address with next available port.
pub fn new_socket_addr() -> SocketAddr {
    UdpSocket::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
}
