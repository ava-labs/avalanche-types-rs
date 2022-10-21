use std::io::{Error, ErrorKind, Result};

use log::debug;

use crate::message::{self, compress, Compressor};

impl Compressor for message::peerlist::Message {
    fn compress_with_header(&self) -> Result<bytes::Bytes> {
        let type_id = message::TYPES
            .get("peerlist")
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "unknown type name"))?;

        // first build uncompressed data
        let packer = message::default_packer();
        packer.pack_u32(self.peers.len() as u32)?;
        // ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/utils/wrappers#Packer.PackIPCert
        for p in self.peers.iter() {
            packer.pack_bytes_with_header(p.certificate.as_ref())?;
            packer.pack_ip(p.ip_addr, p.ip_port)?;
            packer.pack_u64(p.time)?;
            packer.pack_bytes_with_header(p.sig.as_ref())?;
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
                "peerlist compression saved {} bytes",
                bytes_uncompressed - bytes_compressed
            );
        } else {
            debug!(
                "peerlist compression added {} byte(s)",
                bytes_compressed - bytes_uncompressed
            );
        }
        Ok(packer.take_bytes())
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- message::peerlist_gzip::test_message --exact --show-output
#[test]
fn test_message() {
    let msg = message::peerlist::Message::create(
        vec![
            message::peerlist::Peer {
                certificate: vec![0x01, 0x02, 0x03],
                ip_addr: std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
                ip_port: 8080,
                time: 7,
                sig: vec![0x01, 0x02, 0x03, 0x04],
            },
            message::peerlist::Peer {
                certificate: vec![0x01, 0x02, 0x03],
                ip_addr: std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
                ip_port: 8081,
                time: 7,
                sig: vec![0x01, 0x02, 0x03, 0x04],
            },
        ],
        false,
    );
    let data_with_header = msg.compress_with_header().unwrap();
    // for c in &data_with_header {
    //     print!("{:#02x},", *c);
    // }

    let expected_data: &[u8] = &[
        0x00, 0x00, 0x00, 0x3f, // message length
        0x12, // type_id
        0x01, // compressible
        0x1f, 0x8b, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xff, 0x75, 0xca, 0xcb, 0x9, 0x0, 0x20, 0xc,
        0x4, 0xd1, 0xc9, 0x7, 0x2c, 0xc3, 0x52, 0x6d, 0x41, 0x3b, 0x36, 0x87, 0x1c, 0x42, 0x20,
        0xb, 0x7b, 0x18, 0x78, 0x80, 0x2, 0x26, 0x6a, 0x94, 0x1d, 0x90, 0x7d, 0x33, 0x56, 0xdc,
        0x3, 0xf8, 0x4, 0x5f, 0x87, 0x1f, 0x57, 0xe3, 0x44, 0x24, 0x56, 0x0, 0x0,
        0x0, // compressed data
    ];
    assert!(cmp_manager::eq_vectors(expected_data, &data_with_header));
}
