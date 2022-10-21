use std::io::{Error, ErrorKind, Result};

use crate::message::{self, Compressor, Outbound};

/// Message that contains a list of peer information (IP, certs, etc.)
/// in response of "version" message, and sent periodically to a set of
/// validators.
/// ref. "avalanchego/network/network#Dispatch.runtTimers"
///
/// On receiving "peerlist", the engine starts/updates the tracking information
/// of the remote peer.
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub struct Message {
    pub peers: Vec<Peer>,

    pub bypass_throttling: bool,
}

impl Message {
    pub fn create(peers: Vec<Peer>, bypass_throttling: bool) -> impl Outbound + Compressor {
        Self {
            peers,
            bypass_throttling,
        }
    }
}

#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub struct Peer {
    pub certificate: Vec<u8>,
    pub ip_addr: std::net::IpAddr,
    pub ip_port: u16,
    pub time: u64,
    pub sig: Vec<u8>,
}

/// ref. https://doc.rust-lang.org/std/string/trait.ToString.html
/// ref. https://doc.rust-lang.org/std/fmt/trait.Display.html
/// Use "Self.to_string()" to directly invoke this
impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "msg peerlist")
    }
}

impl Outbound for Message {
    fn serialize_with_header(&self) -> Result<bytes::Bytes> {
        let type_id = message::TYPES
            .get("peerlist")
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "unknown type name"))?;

        let packer = message::default_packer_with_header();
        packer.pack_byte(*type_id)?;
        packer.pack_bool(false)?; // compressible
        packer.pack_u32(self.peers.len() as u32)?;
        // ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/utils/wrappers#Packer.PackIPCert
        for p in self.peers.iter() {
            packer.pack_bytes_with_header(p.certificate.as_ref())?;
            packer.pack_ip(p.ip_addr, p.ip_port)?;
            packer.pack_u64(p.time)?;
            packer.pack_bytes_with_header(p.sig.as_ref())?;
        }

        Ok(packer.take_bytes())
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- message::peerlist::test_message --exact --show-output
#[test]
fn test_message() {
    let msg = Message::create(
        vec![
            Peer {
                certificate: vec![0x01, 0x02, 0x03],
                ip_addr: std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
                ip_port: 8080,
                time: 7,
                sig: vec![0x01, 0x02, 0x03, 0x04],
            },
            Peer {
                certificate: vec![0x01, 0x02, 0x03],
                ip_addr: std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
                ip_port: 8081,
                time: 7,
                sig: vec![0x01, 0x02, 0x03, 0x04],
            },
        ],
        false,
    );
    let data_with_header = msg.serialize_with_header().unwrap();
    // for c in &data_with_header {
    //     print!("{:#02x},", *c);
    // }

    let expected_data: &[u8] = &[
        0x00, 0x00, 0x00, 0x58, // message length
        0x12, // type_id
        0x00, // compressible
        0x00, 0x00, 0x00, 0x02, // peers length
        //
        0x00, 0x00, 0x00, 0x03, // certificate length
        0x1, 0x2, 0x3, // certificate
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x0, 0x0,
        0x1, // peer_ip
        0x1f, 0x90, // peer_port
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, // time
        0x00, 0x00, 0x00, 0x04, // signature length
        0x01, 0x02, 0x03, 0x04, // signature
        //
        0x00, 0x00, 0x00, 0x03, // certificate length
        0x1, 0x2, 0x3, // certificate
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x0, 0x0,
        0x1, // peer_ip
        0x1f, 0x91, // peer_port
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, // time
        0x00, 0x00, 0x00, 0x04, // signature length
        0x01, 0x02, 0x03, 0x04, // signature
    ];
    assert!(cmp_manager::eq_vectors(&expected_data, &data_with_header));
}
