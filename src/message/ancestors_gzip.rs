use std::io::{Error, ErrorKind, Result};

use log::debug;

use crate::message::{self, compress, Compressor};

impl Compressor for message::ancestors::Message {
    fn compress_with_header(&self) -> Result<bytes::Bytes> {
        let type_id = message::TYPES
            .get("ancestors")
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "unknown type name"))?;

        // first build uncompressed data
        let packer = message::default_packer();
        packer.pack_bytes(self.chain_id.as_ref())?;
        packer.pack_u32(self.request_id)?;
        packer.pack_u32(self.containers.len() as u32)?;
        for container in self.containers.iter() {
            packer.pack_bytes_with_header(container.as_ref())?;
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
                "ancestors compression saved {} bytes",
                bytes_uncompressed - bytes_compressed
            );
        } else {
            debug!(
                "ancestors compression added {} byte(s)",
                bytes_compressed - bytes_uncompressed
            );
        }
        Ok(packer.take_bytes())
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- message::ancestors_gzip::test_message --exact --show-output
#[test]
fn test_message() {
    let msg = message::ancestors::Message::create(
        crate::ids::Id::empty(),
        7,
        vec![vec![0x01, 0x02], vec![0x03, 0x04]],
    );
    let data_with_header = msg.compress_with_header().unwrap();
    // for c in &data_with_header {
    //     print!("{:#02x},", *c);
    // }

    // Golang compressed
    // 0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x62, 0x20, 0x0c, 0xd8, 0x19,
    // 0x18, 0x18, 0x98, 0x40, 0x98, 0x11, 0x4c, 0x32, 0xb3, 0x00, 0x02, 0x00, 0x00, 0xff, 0xff,
    // 0x35, 0xbc, 0xe2, 0xd6, 0x34, 0x00, 0x00, 0x00,
    let expected_data: &[u8] = &[
        0x00, 0x00, 0x00, 0x2d, // message length
        0x0b, // type_id
        0x01, // compressible
        0x1f, 0x8b, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xff, 0x85, 0xc4, 0x89, 0x9, 0x0, 0x0, 0xc,
        0xc2, 0x40, 0xfb, 0x40, 0xf7, 0xdf, 0xb8, 0xea, 0x2, 0x6, 0x2e, 0x40, 0xec, 0xa8, 0xa5,
        0xfc, 0xd9, 0x7, 0x35, 0xbc, 0xe2, 0xd6, 0x34, 0x0, 0x0, 0x0, // compressed data
    ];
    assert!(cmp_manager::eq_vectors(expected_data, &data_with_header));
}
