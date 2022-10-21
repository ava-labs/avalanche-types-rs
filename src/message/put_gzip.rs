use std::io::{Error, ErrorKind, Result};

use log::debug;

use crate::message::{self, compress, Compressor};

impl Compressor for message::put::Message {
    fn compress_with_header(&self) -> Result<bytes::Bytes> {
        let type_id = message::TYPES
            .get("put")
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "unknown type name"))?;

        // first build uncompressed data
        let packer = message::default_packer();
        packer.pack_bytes(self.chain_id.as_ref())?;
        packer.pack_u32(self.request_id)?;

        // populated only for backward compatibilities
        // ref. https://github.com/ava-labs/avalanchego/commit/ae9ffcecdb53712f1c7071634fc37512057adf20
        packer.pack_bytes(self.container_id.as_ref())?;

        packer.pack_bytes_with_header(self.container_bytes.as_ref())?;

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
                "put compression saved {} bytes",
                bytes_uncompressed - bytes_compressed
            );
        } else {
            debug!(
                "put compression added {} byte(s)",
                bytes_compressed - bytes_uncompressed
            );
        }
        Ok(packer.take_bytes())
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- message::put_gzip::test_message --exact --show-output
#[test]
fn test_message() {
    use crate::ids;
    let msg = message::put::Message::create(ids::Id::empty(), 7, vec![0x01, 0x02, 0x03]);
    let data_with_header = msg.compress_with_header().unwrap();
    // for c in &data_with_header {
    //     print!("{:#02x},", *c);
    // }

    // Golang compressed
    // 0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x62, 0x20, 0x0c, 0xd8, 0xf1,
    // 0x4b, 0x33, 0x32, 0x32, 0x30, 0x30, 0x30, 0x33, 0x32, 0x31, 0x03, 0x02, 0x00, 0x00, 0xff,
    // 0xff, 0x47, 0xe6, 0x86, 0x7b, 0x4b, 0x00, 0x00, 0x00,
    let expected_data: &[u8] = &[
        0x00, 0x00, 0x00, 0x29, // message length
        0x0d, // type_id
        0x01, // compressible
        0x1f, 0x8b, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xff, 0x85, 0xca, 0xb1, 0x1, 0x0, 0x0, 0x0,
        0xc1, 0xb0, 0x62, 0xf0, 0xff, 0xc7, 0x4e, 0x90, 0x39, 0x70, 0xf5, 0x17, 0x22, 0x67, 0x9a,
        0xf2, 0x57, 0xa0, 0x4b, 0x0, 0x0, 0x0, // compressed data
    ];
    assert!(cmp_manager::eq_vectors(expected_data, &data_with_header));
}
