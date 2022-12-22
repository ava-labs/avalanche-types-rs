//! RPC Chain VM implementation.
pub mod server;

pub trait Vm: crate::subnet::rpc::snowman::block::ChainVm {}
