use std::io::{Error, ErrorKind, Result};

use crate::message::{self, Outbound};

/// Message that the local node sends to its remote peers,
/// in order to periodically check its uptime.
///
/// On receiving "ping", the remote peer responds with the observed
/// uptime value of this local node in "pong" message.
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub struct Message {}

impl Message {
    pub fn create() -> impl Outbound {
        Self {}
    }
}

/// ref. https://doc.rust-lang.org/std/string/trait.ToString.html
/// ref. https://doc.rust-lang.org/std/fmt/trait.Display.html
/// Use "Self.to_string()" to directly invoke this
impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "msg ping")
    }
}

impl Outbound for Message {
    fn serialize_with_header(&self) -> Result<bytes::Bytes> {
        let type_id = message::TYPES
            .get("ping")
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "unknown type name"))?;

        let packer = message::default_packer_with_header();
        packer.pack_byte(*type_id)?;

        Ok(packer.take_bytes())
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- message::ping::test_message --exact --show-output
#[test]
fn test_message() {
    let msg = Message::create();
    let data_with_header = msg.serialize_with_header().unwrap();
    // for c in &data_with_header {
    //     print!("{:#02x},", *c);
    // }

    let expected_data: &[u8] = &[
        0x00, 0x00, 0x00, 0x01, // message length
        0x4,  // type_id
    ];
    assert!(cmp_manager::eq_vectors(&expected_data, &data_with_header));
}
