use std::io::{Error, ErrorKind, Result};

use log::debug;

use crate::message::{self, compress, Compressor};

impl Compressor for message::app_request::Message {
    fn compress_with_header(&self) -> Result<bytes::Bytes> {
        let type_id = message::TYPES
            .get("app_request")
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "unknown type name"))?;

        // first build uncompressed data
        let packer = message::default_packer();
        packer.pack_bytes(self.chain_id.as_ref())?;
        packer.pack_u32(self.request_id)?;
        packer.pack_u64(self.deadline.as_nanos() as u64)?;
        packer.pack_bytes_with_header(self.app_bytes.as_ref())?;

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
                "app_request compression saved {} bytes",
                bytes_uncompressed - bytes_compressed
            );
        } else {
            debug!(
                "app_request compression added {} byte(s)",
                bytes_compressed - bytes_uncompressed
            );
        }
        Ok(packer.take_bytes())
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- message::app_request_gzip::test_message --exact --show-output
#[test]
fn test_message() {
    use crate::ids;
    let msg = message::app_request::Message::create(
        ids::Id::empty(),
        7,
        std::time::Duration::from_secs(10),
        vec![0x01, 0x02, 0x03, 0x04],
    );
    let data_with_header = msg.compress_with_header().unwrap();
    // for c in &data_with_header {
    //     print!("{:#02x},", *c);
    // }

    let expected_data: &[u8] = &[
        0x00, 0x00, 0x00, 0x33, // message length
        0x14, // type_id
        0x01, // compressible
        0x1f, 0x8b, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xff, 0x85, 0xc6, 0xb1, 0x9, 0x0, 0x0, 0x8,
        0x3, 0xc1, 0x18, 0x3, 0x16, 0xae, 0xe7, 0xfe, 0x95, 0x83, 0xa8, 0x13, 0xf8, 0xf0, 0x70,
        0xc0, 0x5b, 0xec, 0xac, 0xec, 0xb3, 0x8c, 0xae, 0x1, 0x9d, 0xf, 0x48, 0xcc, 0x34, 0x0, 0x0,
        0x0, // compressed data
    ];
    assert!(cmp_manager::eq_vectors(expected_data, &data_with_header));
}
