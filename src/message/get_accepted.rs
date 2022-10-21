use std::io::{Error, ErrorKind, Result};

use crate::ids;
use crate::message::{self, Outbound};

/// Message to request for the accepted blocks/vertices of the "remote" peer.
/// The local node sends out this message during bootstrap, following "get_accepted_frontier".
/// Basically, sending the list of the accepted frontier and expects the response of
/// the accepted IDs from the remote peer.
/// See "avalanchego/snow/engine/common/bootstrapper.Startup" and "sendGetAccepted".
///
/// On receiving "get_accepted", the engine queries the containers from the storage,
/// and responds back with the set of accepted containers in the "accepted" message.
///
/// On receiving "accepted", the bootstrapper recursively sends "get_accepted" to
/// other validators, thus continuously syncing.
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
    pub container_ids: Vec<ids::Id>,
}

impl Message {
    pub fn create(
        chain_id: ids::Id,
        request_id: u32,
        deadline: std::time::Duration,
        container_ids: Vec<ids::Id>,
    ) -> impl Outbound {
        Self {
            chain_id,
            request_id,
            deadline,
            container_ids,
        }
    }
}

/// ref. https://doc.rust-lang.org/std/string/trait.ToString.html
/// ref. https://doc.rust-lang.org/std/fmt/trait.Display.html
/// Use "Self.to_string()" to directly invoke this
impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "msg get_accepted")
    }
}

impl Outbound for Message {
    fn serialize_with_header(&self) -> Result<bytes::Bytes> {
        let type_id = message::TYPES
            .get("get_accepted")
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "unknown type name"))?;

        let packer = message::default_packer_with_header();
        packer.pack_byte(*type_id)?;
        packer.pack_bytes(self.chain_id.as_ref())?;
        packer.pack_u32(self.request_id)?;
        packer.pack_u64(self.deadline.as_nanos() as u64)?;
        packer.pack_u32(self.container_ids.len() as u32)?;
        for id in self.container_ids.iter() {
            packer.pack_bytes(id.as_ref())?;
        }

        Ok(packer.take_bytes())
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- message::get_accepted::test_message --exact --show-output
#[test]
fn test_message() {
    let msg = Message::create(
        ids::Id::empty(),
        7,
        std::time::Duration::from_secs(10),
        vec![
            ids::Id::from_slice(&[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                0x01, 0x01, //
            ]),
            ids::Id::from_slice(&[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                0x02, 0x02, //
            ]),
            ids::Id::from_slice(&[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                0x03, 0x03, //
            ]),
        ],
    );
    let data_with_header = msg.serialize_with_header().unwrap();
    // for c in &data_with_header {
    //     print!("{:#02x},", *c);
    // }

    let expected_data: &[u8] = &[
        0x00, 0x00, 0x00, 0x91, // message length
        0x08, // type_id
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x00, 0x00, // chain_id
        0x00, 0x00, 0x00, 0x07, // request_id
        0x00, 0x00, 0x00, 0x02, 0x54, 0x0b, 0xe4, 0x00, // deadline
        0x00, 0x00, 0x00, 0x03, // length of container_ids
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x01, 0x01, //
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x02, 0x02, //
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x03, 0x03, //
    ];
    assert!(cmp_manager::eq_vectors(&expected_data, &data_with_header));
}
