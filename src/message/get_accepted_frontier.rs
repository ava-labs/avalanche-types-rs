use std::io::{Error, ErrorKind, Result};

use crate::ids;
use crate::message::{self, Outbound};

/// Message to request for the accepted frontier of the "remote" peer.
/// For instance, the accepted frontier of X-chain DAG is the set of
/// accepted vertices that do not have any accepted descendants thus frontier.
/// During bootstrap, the local node sends out this message to validators
/// (see "avalanchego/snow/engine/common/bootstrapper.Startup").
/// The expected response is "accepted_frontier".
///
/// On receiving "accepted_frontier", the bootstrapper recursively sends
/// "get_accepted_frontier" to other validators, thus continuously syncing.
/// See "snow/engine/common/bootstrapper.go#AcceptedFrontier".
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
    pub chain_id: ids::Id,
    pub request_id: u32,
    pub deadline: std::time::Duration,
}

impl Message {
    pub fn create(
        chain_id: ids::Id,
        request_id: u32,
        deadline: std::time::Duration,
    ) -> impl Outbound {
        Self {
            chain_id,
            request_id,
            deadline,
        }
    }
}

/// ref. https://doc.rust-lang.org/std/string/trait.ToString.html
/// ref. https://doc.rust-lang.org/std/fmt/trait.Display.html
/// Use "Self.to_string()" to directly invoke this
impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "msg get_accepted_frontier")
    }
}

impl Outbound for Message {
    fn serialize_with_header(&self) -> Result<bytes::Bytes> {
        let type_id = message::TYPES
            .get("get_accepted_frontier")
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "unknown type name"))?;

        let packer = message::default_packer_with_header();
        packer.pack_byte(*type_id)?;
        packer.pack_bytes(self.chain_id.as_ref())?;
        packer.pack_u32(self.request_id)?;
        packer.pack_u64(self.deadline.as_nanos() as u64)?;

        Ok(packer.take_bytes())
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- message::get_accepted_frontier::test_message --exact --show-output
#[test]
fn test_message() {
    let msg = Message::create(ids::Id::empty(), 7, std::time::Duration::from_secs(10));
    let data_with_header = msg.serialize_with_header().unwrap();
    // for c in &data_with_header {
    //     print!("{:#02x},", *c);
    // }

    let expected_data: &[u8] = &[
        0x00, 0x00, 0x00, 0x2d, // message length
        0x06, // type_id
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x00, 0x00, // chain_id
        0x00, 0x00, 0x00, 0x07, // request_id
        0x00, 0x00, 0x00, 0x02, 0x54, 0x0b, 0xe4, 0x00, // deadline
    ];
    assert!(cmp_manager::eq_vectors(&expected_data, &data_with_header));
}
