pub mod grpcutil;
pub mod pb;

pub use pb::*;

/// ref. https://github.com/ava-labs/avalanchego/blob/v1.9.2/version/constants.go#L15-L17
pub const PROTOCOL_VERSION: &str = "19";
