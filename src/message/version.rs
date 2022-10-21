use std::{
    io::{Error, ErrorKind, Result},
    net::IpAddr,
};

use crate::ids;
use crate::message::{self, Outbound};

/// The first outbound message that the local node sends to its remote peer
/// when the connection is established. In order for the local node to be
/// tracked as a valid peer by the remote peer, the fields must be valid.
/// For instance, the network ID must be matched and timestamp should be in-sync.
/// Otherwise, the remote peer closes the connection.
/// ref. "avalanchego/network/peer#handleVersion"
/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/network#Network "Dispatch"
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
    pub network_id: u32,
    /// Local time in unix second.
    pub my_time: u64,
    pub ip_addr: IpAddr,
    pub ip_port: u16,
    pub my_version: String,
    pub my_version_time: u64,
    pub sig: Vec<u8>,
    pub tracked_subnets: Vec<ids::Id>,
}

pub fn create_outbound(msg: Message) -> impl Outbound {
    msg
}

/// ref. https://doc.rust-lang.org/std/string/trait.ToString.html
/// ref. https://doc.rust-lang.org/std/fmt/trait.Display.html
/// Use "Self.to_string()" to directly invoke this
impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "msg version (network ID {})", self.network_id)
    }
}

impl Outbound for Message {
    fn serialize_with_header(&self) -> Result<bytes::Bytes> {
        let type_id = message::TYPES
            .get("version")
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "unknown type name"))?;

        let packer = message::default_packer_with_header();
        packer.pack_byte(*type_id)?;
        packer.pack_u32(self.network_id)?;
        packer.pack_u32(0)?; // "node_id" is deprecated, so just encode 0
        packer.pack_u64(self.my_time)?;
        packer.pack_ip(self.ip_addr, self.ip_port)?;
        packer.pack_str(&self.my_version)?;
        packer.pack_u64(self.my_version_time)?;
        packer.pack_bytes_with_header(self.sig.as_ref())?;
        packer.pack_u32(self.tracked_subnets.len() as u32)?;
        for id in self.tracked_subnets.iter() {
            packer.pack_bytes(id.as_ref())?;
        }

        Ok(packer.take_bytes())
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- message::version::test_message --exact --show-output
#[test]
fn test_message() {
    use std::net::Ipv4Addr;

    let msg = create_outbound(Message {
        network_id: 100000,
        my_time: 77777777,
        ip_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
        ip_port: 8080,
        my_version: String::from("v1.2.3"),
        my_version_time: 1234567,
        sig: vec![0x01, 0x02, 0x03],
        tracked_subnets: vec![crate::ids::Id::from_slice(&[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x01, 0x01, //
        ])],
    });
    let data_with_header = msg.serialize_with_header().unwrap();
    // for c in &data_with_header {
    //     print!("{:#02x},", *c);
    // }

    let expected_data: &[u8] = &[
        0x00, 0x00, 0x00, 0x5e, // message length
        0x13, // type_id
        0x00, 0x01, 0x86, 0xa0, // network_id
        0x00, 0x00, 0x00, 0x00, // node_id
        0x00, 0x00, 0x00, 0x00, 0x04, 0xa2, 0xcb, 0x71, // my_time
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x00, 0x00,
        0x01, // ip_addr
        0x1f, 0x90, // ip_port
        0x00, 0x06, // length of my_version
        0x76, 0x31, 0x2e, 0x32, 0x2e, 0x33, // my_version
        0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0xd6, 0x87, // my version time
        0x00, 0x00, 0x00, 0x03, // length of signature
        0x01, 0x02, 0x03, // signature
        0x00, 0x00, 0x00, 0x01, // length of tracked_subnets
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x01, 0x01, // tracked_subnet
    ];
    assert!(cmp_manager::eq_vectors(&expected_data, &data_with_header));
}
