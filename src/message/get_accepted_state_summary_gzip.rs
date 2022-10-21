use std::io::{Error, ErrorKind, Result};

use log::debug;

use crate::message::{self, compress, Compressor};

impl Compressor for message::get_accepted_state_summary::Message {
    fn compress_with_header(&self) -> Result<bytes::Bytes> {
        let type_id = message::TYPES
            .get("get_accepted_state_summary")
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "unknown type name"))?;

        // first build uncompressed data
        let packer = message::default_packer();
        packer.pack_bytes(self.chain_id.as_ref())?;
        packer.pack_u32(self.request_id)?;
        packer.pack_u64(self.deadline.as_nanos() as u64)?;
        packer.pack_u32(self.heights.len() as u32)?;
        for height in self.heights.iter() {
            packer.pack_u64(*height)?;
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
                "get_accepted_state_summary compression saved {} bytes",
                bytes_uncompressed - bytes_compressed
            );
        } else {
            debug!(
                "get_accepted_state_summary compression added {} byte(s)",
                bytes_compressed - bytes_uncompressed
            );
        }
        Ok(packer.take_bytes())
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- message::get_accepted_state_summary_gzip::test_message --exact --show-output
#[test]
fn test_message() {
    use crate::ids;
    let msg = message::get_accepted_state_summary::Message::create(
        ids::Id::empty(),
        7,
        std::time::Duration::from_secs(10),
        vec![0x10, 0x11, 0x12],
    );
    let data = msg.compress_with_header().unwrap();
    // for c in &data {
    //     print!("{:#02x},", *c);
    // }

    let expected_data: &[u8] = &[
        0x00, 0x00, 0x00, 0x37, // message length
        0x19, // type_id
        0x01, // compressible
        0x1f, 0x8b, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xff, 0x85, 0x88, 0xb1, 0xd, 0x0, 0x0, 0x4,
        0x4, 0x85, 0x42, 0xa1, 0xc1, 0x6c, 0xf6, 0xaf, 0xc, 0x82, 0xe4, 0x7b, 0x9f, 0x7c, 0xee,
        0x72, 0x44, 0xef, 0x74, 0xcf, 0x65, 0x7d, 0x2e, 0x68, 0xe, 0x6, 0x98, 0x3, 0x5c, 0xa5,
        0x13, 0x78, 0x48, 0x0, 0x0, 0x0, // compressed data
    ];
    assert!(cmp_manager::eq_vectors(expected_data, &data));
}
