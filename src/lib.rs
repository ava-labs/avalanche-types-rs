#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod avm;
pub mod choices;
pub mod codec;
pub mod constants;
pub mod errors;
pub mod formatting;
pub mod hash;
pub mod ids;
pub mod jsonrpc;
pub mod key;
pub mod node;
pub mod packer;
pub mod platformvm;
pub mod txs;
pub mod units;
pub mod utils;
pub mod verify;

#[cfg(feature = "avalanchego")]
#[cfg_attr(docsrs, doc(cfg(feature = "avalanchego")))]
pub mod avalanchego;

#[cfg(feature = "avalanchego")]
#[cfg_attr(docsrs, doc(cfg(feature = "avalanchego")))]
pub mod coreth;

#[cfg(feature = "subnet_evm")]
#[cfg_attr(docsrs, doc(cfg(feature = "subnet_evm")))]
pub mod subnet_evm;

#[cfg(feature = "xsvm")]
#[cfg_attr(docsrs, doc(cfg(feature = "xsvm")))]
pub mod xsvm;

#[cfg(feature = "evm")]
#[cfg_attr(docsrs, doc(cfg(feature = "evm")))]
pub mod evm;

#[cfg(feature = "message")]
#[cfg_attr(docsrs, doc(cfg(feature = "message")))]
pub mod message;

#[cfg(feature = "wallet")]
#[cfg_attr(docsrs, doc(cfg(feature = "wallet")))]
pub mod wallet;

#[cfg(feature = "proto")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto")))]
pub mod proto;

#[cfg(feature = "subnet")]
#[cfg_attr(docsrs, doc(cfg(feature = "subnet")))]
pub mod subnet;
