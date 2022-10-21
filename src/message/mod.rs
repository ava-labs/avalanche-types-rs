pub mod accepted;
pub mod accepted_frontier;
pub mod accepted_state_summary;
pub mod ancestors;
pub mod app_gossip;
pub mod app_request;
pub mod app_request_failed;
pub mod app_response;
pub mod chits;
pub mod chits_v2;
pub mod compress;
pub mod connected;
pub mod disconnected;
pub mod get;
pub mod get_accepted;
pub mod get_accepted_failed;
pub mod get_accepted_frontier;
pub mod get_accepted_frontier_failed;
pub mod get_accepted_state_summary;
pub mod get_accepted_state_summary_failed;
pub mod get_ancestors;
pub mod get_ancestors_failed;
pub mod get_failed;
pub mod get_state_summary_frontier;
pub mod get_state_summary_frontier_failed;
pub mod gossip_request;
pub mod notify;
pub mod peerlist;
pub mod ping;
pub mod pong;
pub mod pull_query;
pub mod push_query;
pub mod put;
pub mod query_failed;
pub mod state_summary_frontier;
pub mod timeout;
pub mod version;

#[cfg(feature = "message_compress_gzip")]
pub mod accepted_state_summary_gzip;

#[cfg(feature = "message_compress_gzip")]
pub mod ancestors_gzip;

#[cfg(feature = "message_compress_gzip")]
pub mod app_gossip_gzip;

#[cfg(feature = "message_compress_gzip")]
pub mod app_request_gzip;

#[cfg(feature = "message_compress_gzip")]
pub mod app_response_gzip;

#[cfg(feature = "message_compress_gzip")]
pub mod get_accepted_state_summary_gzip;

#[cfg(feature = "message_compress_gzip")]
pub mod peerlist_gzip;

#[cfg(feature = "message_compress_gzip")]
pub mod push_query_gzip;

#[cfg(feature = "message_compress_gzip")]
pub mod put_gzip;

#[cfg(feature = "message_compress_gzip")]
pub mod state_summary_frontier_gzip;

use std::{collections::HashMap, io::Result};

use bytes::Bytes;
use lazy_static::lazy_static;

lazy_static! {
    /// Defines the integer ordinal number (iota) as defined in avalanchego.
    /// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/message#Op
    pub static ref TYPES: HashMap<String, u8> = {
        let mut m = HashMap::new();
        m.insert("pong".to_string(), 3);
        m.insert("ping".to_string(), 4);

        // bootstrapping
        m.insert("get_accepted_frontier".to_string(), 6);
        m.insert("accepted_frontier".to_string(), 7);
        m.insert("get_accepted".to_string(), 8);
        m.insert("accepted".to_string(), 9);
        m.insert("get_ancestors".to_string(), 10);
        m.insert("ancestors".to_string(), 11);

        // consensus
        m.insert("get".to_string(), 12);
        m.insert("put".to_string(), 13);
        m.insert("push_query".to_string(), 14);
        m.insert("pull_query".to_string(), 15);
        m.insert("chits".to_string(), 16);

        // handshake / peer gossiping
        m.insert("peerlist".to_string(), 18);
        m.insert("version".to_string(), 19);

        // application level
        m.insert("app_request".to_string(), 20);
        m.insert("app_response".to_string(), 21);
        m.insert("app_gossip".to_string(), 22);

        // state sync
        m.insert("get_state_summary_frontier".to_string(), 23);
        m.insert("state_summary_frontier".to_string(), 24);
        m.insert("get_accepted_state_summary".to_string(), 25);
        m.insert("accepted_state_summary".to_string(), 26);

        // x-chain linearization
        m.insert("chits_v2".to_string(), 27);

        // internal messages (iota can be anything)
        m.insert("get_accepted_frontier_failed".to_string(), 30);
        m.insert("get_accepted_failed".to_string(), 31);
        m.insert("get_failed".to_string(), 32);
        m.insert("query_failed".to_string(), 33);
        m.insert("get_ancestors_failed".to_string(), 34);
        m.insert("app_request_failed".to_string(), 35);
        m.insert("timeout".to_string(), 36);
        m.insert("connected".to_string(), 37);
        m.insert("disconnected".to_string(), 38);
        m.insert("notify".to_string(), 39);
        m.insert("gossip_request".to_string(), 40);
        m.insert("get_state_summary_frontier_failed".to_string(), 41);
        m.insert("get_accepted_state_summary_failed".to_string(), 42);
        m
    };
}

pub trait Outbound {
    /// Serializes the message with its size header.
    /// The first 32-bit (4-byte) represents the length of the message.
    /// ref. "avalanchego/network/peer.writeMessages"
    fn serialize_with_header(&self) -> Result<Bytes>;
}

pub trait Compressor: Outbound {
    /// Compresses the message with the header that represents the size of the compressed message.
    /// The first 32-bit (4-byte) represents the length of the message.
    /// ref. "avalanchego/network/peer.writeMessages"
    fn compress_with_header(&self) -> Result<Bytes>;
}

pub fn default_packer() -> crate::packer::Packer {
    // ref. "math.MaxInt32" and "constants.DefaultByteSliceCap" in Go
    crate::packer::Packer::new((1 << 31) - 1, 128)
}

pub fn default_packer_with_header() -> crate::packer::Packer {
    // ref. "math.MaxInt32" and "constants.DefaultByteSliceCap" in Go
    crate::packer::Packer::new_with_header((1 << 31) - 1, 128)
}
