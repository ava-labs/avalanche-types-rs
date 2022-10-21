pub mod server;

// TODO: This is where we would define alternate Vm traits based on features.
// example
// #![cfg(feature = "chainvm_statesyncvm")]
// pub trait Vm: crate::rpcchainvm::snowman::block::ChainVm + StateSyncableVm {}

/// Inner Vm trait that the gRPC server and client should take as a inner object.
pub trait Vm: crate::rpcchainvm::snowman::block::ChainVm {}
