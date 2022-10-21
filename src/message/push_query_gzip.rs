use std::io::{Error, ErrorKind, Result};

use log::debug;

use crate::message::{self, compress, Compressor};

impl Compressor for message::push_query::Message {
    fn compress_with_header(&self) -> Result<bytes::Bytes> {
        let type_id = message::TYPES
            .get("push_query")
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "unknown type name"))?;

        // first build uncompressed data
        let packer = message::default_packer();
        packer.pack_bytes(self.chain_id.as_ref())?;
        packer.pack_u32(self.request_id)?;
        packer.pack_u64(self.deadline.as_nanos() as u64)?;

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
                "push_query compression saved {} bytes",
                bytes_uncompressed - bytes_compressed
            );
        } else {
            debug!(
                "push_query compression added {} byte(s)",
                bytes_compressed - bytes_uncompressed
            );
        }
        Ok(packer.take_bytes())
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- message::push_query_gzip::test_message --exact --show-output
#[test]
fn test_message() {
    use crate::ids;
    let msg = message::push_query::Message::create(
        ids::Id::empty(),
        7,
        std::time::Duration::from_secs(10),
        vec![0x01, 0x02, 0x03],
    );
    let data_with_header = msg.compress_with_header().unwrap();
    // for c in &data_with_header {
    //     print!("{:#02x},", *c);
    // }

    // 48 in Golang but it's ok -- Golang can still decompress!
    // Golang compressed
    // 0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff,
    // 0x62, 0x20, 0x0c, 0xd8, 0x19, 0x18, 0x18, 0x98, 0x42, 0xb8,
    // 0x9f, 0x10, 0x50, 0xc6, 0xc8, 0xc8, 0xc0, 0xc0, 0xc0, 0xcc,
    // 0xc8, 0xc4, 0x0c, 0x08, 0x00, 0x00, 0xff, 0xff, 0x19, 0x60,
    // 0x0e, 0xc5, 0x53, 0x00, 0x00, 0x00,
    let expected_data: &[u8] = &[
        0x00, 0x00, 0x00, 0x31, // message length
        0x0e, // type_id
        0x01, // compressible
        0x1f, 0x8b, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xff, 0x8d, 0xc3, 0x31, 0xd, 0x0, 0x0, 0x8,
        0x3, 0xb0, 0xb1, 0x1d, 0x1c, 0xd8, 0xc3, 0xff, 0x85, 0x10, 0x70, 0xc0, 0x9a, 0x14, 0x78,
        0xe5, 0x65, 0xd7, 0xc0, 0xa0, 0xa0, 0x16, 0xc4, 0x74, 0xdf, 0x1e, 0x53, 0x0, 0x0,
        0x0, // compressed data
    ];
    assert!(cmp_manager::eq_vectors(expected_data, &data_with_header));
}
