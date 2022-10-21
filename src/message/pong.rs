use std::io::{Error, ErrorKind, Result};

use crate::message::{self, Outbound};

/// Message that contains the uptime of the message sender (remote peer)
/// from the receiver's point of view, in response to "ping" message.
///
/// On receiving "pong", the local node updates the observed uptime
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
    pub uptime_pct: u8,
}

impl Message {
    pub fn create(uptime_pct: u8) -> impl Outbound {
        Self { uptime_pct }
    }
}

/// ref. https://doc.rust-lang.org/std/string/trait.ToString.html
/// ref. https://doc.rust-lang.org/std/fmt/trait.Display.html
/// Use "Self.to_string()" to directly invoke this
impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "msg pong")
    }
}

impl Outbound for Message {
    fn serialize_with_header(&self) -> Result<bytes::Bytes> {
        let type_id = message::TYPES
            .get("pong")
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "unknown type name"))?;

        let packer = message::default_packer_with_header();
        packer.pack_byte(*type_id)?;
        packer.pack_byte(self.uptime_pct)?;

        Ok(packer.take_bytes())
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- message::pong::test_message --exact --show-output
#[test]
fn test_message() {
    let msg = Message::create(7);
    let data_with_header = msg.serialize_with_header().unwrap();
    // for c in &data_with_header {
    //     print!("{:#02x},", *c);
    // }

    let expected_data: &[u8] = &[
        0x00, 0x00, 0x00, 0x02, // message length
        0x3, 0x7,
    ];
    assert!(cmp_manager::eq_vectors(&expected_data, &data_with_header));
}
