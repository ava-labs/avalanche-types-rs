use std::io::{Error, ErrorKind, Result};

use log::debug;

use crate::message::{self, compress, Compressor};

impl Compressor for message::app_gossip::Message {
    fn compress_with_header(&self) -> Result<bytes::Bytes> {
        let type_id = message::TYPES
            .get("app_gossip")
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "unknown type name"))?;

        // first build uncompressed data
        let packer = message::default_packer();
        packer.pack_bytes(self.chain_id.as_ref())?;
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
                "app_gossip compression saved {} bytes",
                bytes_uncompressed - bytes_compressed
            );
        } else {
            debug!(
                "app_gossip compression added {} byte(s)",
                bytes_compressed - bytes_uncompressed
            );
        }
        Ok(packer.take_bytes())
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- message::app_gossip_gzip::test_message --exact --show-output
#[test]
fn test_message() {
    use crate::ids;
    let msg = message::app_gossip::Message::create(ids::Id::empty(), vec![0x01, 0x02, 0x03, 0x04]);
    let data_with_header = msg.compress_with_header().unwrap();
    // for c in &data_with_header {
    //     print!("{:#02x},", *c);
    // }

    let expected_data: &[u8] = &[
        0x00, 0x00, 0x00, 0x1e, // message length
        0x16, // type_id
        0x01, // compressible
        0x1f, 0x8b, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xff, 0x63, 0x60, 0x20, 0x8, 0x58, 0x18,
        0x99, 0x98, 0x59, 0x0, 0xa0, 0xbf, 0x14, 0x8b, 0x28, 0x0, 0x0, 0x0, // compressed data
    ];
    assert!(cmp_manager::eq_vectors(expected_data, &data_with_header));
}
