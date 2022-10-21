use std::io::{Error, ErrorKind, Result};

use log::debug;

use crate::message::{self, compress, Compressor};

impl Compressor for message::accepted_state_summary::Message {
    fn compress_with_header(&self) -> Result<bytes::Bytes> {
        let type_id = message::TYPES
            .get("accepted_state_summary")
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "unknown type name"))?;

        // first build uncompressed data
        let packer = message::default_packer();
        packer.pack_bytes(self.chain_id.as_ref())?;
        packer.pack_u32(self.request_id)?;
        packer.pack_u32(self.summary_ids.len() as u32)?;
        for id in self.summary_ids.iter() {
            packer.pack_bytes(id.as_ref())?;
        }

        // compress the data
        let bytes_uncompressed = packer.bytes_len();
        let compressed = compress::pack_gzip(&packer.take_bytes())?;
        let bytes_compressed = compressed.len();

        // serialize with compressed data
        let packer = message::default_packer_with_header();
        packer.pack_byte(*type_id)?;
        packer.pack_bool(true)?; // compressible
        packer.pack_bytes(compressed.as_ref())?;

        if bytes_uncompressed > bytes_compressed {
            debug!(
                "accepted_state_summary compression saved {} bytes",
                bytes_uncompressed - bytes_compressed
            );
        } else {
            debug!(
                "accepted_state_summary compression added {} byte(s)",
                bytes_compressed - bytes_uncompressed
            );
        }
        Ok(packer.take_bytes())
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- message::accepted_state_summary_gzip::test_message --exact --show-output
#[test]
fn test_message() {
    let msg = message::accepted_state_summary::Message::create(
        crate::ids::Id::empty(),
        7,
        vec![
            crate::ids::Id::from_slice(&[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                0x01, 0x01, //
            ]),
            crate::ids::Id::from_slice(&[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                0x02, 0x02, //
            ]),
            crate::ids::Id::from_slice(&[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                0x03, 0x03, //
            ]),
        ],
    );
    let data_with_header = msg.compress_with_header().unwrap();
    // for c in &data_with_header {
    //     print!("{:#02x},", *c);
    // }

    let expected_data: &[u8] = &[
        0x00, 0x00, 0x00, 0x30, // message length
        0x1a, // type_id
        0x01, // compressible
        0x1f, 0x8b, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xff, 0x85, 0x8a, 0xb9, 0xd, 0x0, 0x0, 0x8,
        0x84, 0xee, 0x29, 0xdc, 0x7f, 0x63, 0x9d, 0x0, 0x49, 0xa8, 0x40, 0x7a, 0x99, 0xb3, 0xbc,
        0xd8, 0xdc, 0x13, 0xee, 0xed, 0x2, 0xe0, 0xfe, 0xb9, 0xa8, 0x88, 0x0, 0x0,
        0x0, // compressed data
    ];
    assert!(cmp_manager::eq_vectors(expected_data, &data_with_header));
}
