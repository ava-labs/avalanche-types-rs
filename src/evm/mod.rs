pub mod txs;

use primitive_types::{H160, H256};
use rlp_derive::{RlpDecodable, RlpDecodableWrapper, RlpEncodable, RlpEncodableWrapper};
use serde::{self, Deserialize, Serialize};

/// ref. "ethers-core::types::transaction::eip2930::AccessListItem"
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, RlpEncodable, RlpDecodable,
)]
pub struct AccessListItem {
    pub address: H160,
    pub storage_keys: Vec<H256>,
}

/// ref. "ethers-core::types::transaction::eip2930::AccessListItem"
/// NB: Need to use `RlpEncodableWrapper` else we get an extra [] in the output
/// https://github.com/gakonst/ethers-rs/pull/353#discussion_r680683869
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    RlpEncodableWrapper,
    RlpDecodableWrapper,
)]
pub struct AccessList(pub Vec<AccessListItem>);

impl From<Vec<AccessListItem>> for AccessList {
    fn from(src: Vec<AccessListItem>) -> AccessList {
        AccessList(src)
    }
}
