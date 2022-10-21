use crate::{ids::node::Id as NodeId, ids::Id};
use avalanche_proto::{
    aliasreader::alias_reader_client::AliasReaderClient, keystore::keystore_client::KeystoreClient,
    sharedmemory::shared_memory_client::SharedMemoryClient,
    subnetlookup::subnet_lookup_client::SubnetLookupClient,
};
use tonic::transport::Channel;

/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/snow#Context
#[derive(Debug, Clone)]
pub struct Context {
    pub network_id: u32,
    pub subnet_id: Id,
    pub chain_id: Id,
    pub node_id: NodeId,
    pub x_chain_id: Id,
    pub avax_asset_id: Id,
    pub keystore: KeystoreClient<Channel>,
    pub shared_memory: SharedMemoryClient<Channel>,
    pub bc_lookup: AliasReaderClient<Channel>,
    pub sn_lookup: SubnetLookupClient<Channel>,
    // TODO metrics
}
