use std::collections::HashMap;

use crate::ids::{self, node};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetnetworkname>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetNetworkNameResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetNetworkNameResult>,
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetnetworkname>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetNetworkNameResult {
    pub network_name: String,
}

impl Default for GetNetworkNameResult {
    fn default() -> Self {
        Self::default()
    }
}

impl GetNetworkNameResult {
    pub fn default() -> Self {
        Self {
            network_name: String::new(),
        }
    }
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetnetworkid>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetNetworkIdResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetNetworkIdResult>,
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetnetworkid>
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetNetworkIdResult {
    #[serde(rename = "networkID")]
    #[serde_as(as = "DisplayFromStr")]
    pub network_id: u32,
}

impl Default for GetNetworkIdResult {
    fn default() -> Self {
        Self::default()
    }
}

impl GetNetworkIdResult {
    pub fn default() -> Self {
        Self { network_id: 1 }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::info::test_get_network_id --exact --show-output
#[test]
fn test_get_network_id() {
    // ref. https://docs.avax.network/build/avalanchego-apis/info/#infogetnetworkid
    let resp: GetNetworkIdResponse = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": {
        \"networkID\": \"9999999\"
    },
    \"id\": 1
}

",
    )
    .unwrap();

    let expected = GetNetworkIdResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetNetworkIdResult {
            network_id: 9999999_u32,
        }),
    };
    assert_eq!(resp, expected);
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetblockchainid>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetBlockchainIdResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetBlockchainIdResult>,
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetblockchainid>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetBlockchainIdResult {
    #[serde(rename = "blockchainID")]
    pub blockchain_id: ids::Id,
}

impl Default for GetBlockchainIdResult {
    fn default() -> Self {
        Self::default()
    }
}

impl GetBlockchainIdResult {
    pub fn default() -> Self {
        Self {
            blockchain_id: ids::Id::default(),
        }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::info::test_get_blockchain_id --exact --show-output
#[test]
fn test_get_blockchain_id() {
    use std::str::FromStr;

    // ref. https://docs.avax.network/build/avalanchego-apis/info/#infogetblockchainid
    let resp: GetBlockchainIdResponse = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": {
        \"blockchainID\": \"sV6o671RtkGBcno1FiaDbVcFv2sG5aVXMZYzKdP4VQAWmJQnM\"
    },
    \"id\": 1
}

",
    )
    .unwrap();

    let expected = GetBlockchainIdResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetBlockchainIdResult {
            blockchain_id: ids::Id::from_str("sV6o671RtkGBcno1FiaDbVcFv2sG5aVXMZYzKdP4VQAWmJQnM")
                .unwrap(),
        }),
    };
    assert_eq!(resp, expected);
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetnodeid>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetNodeIdResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetNodeIdResult>,
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetnodeid>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetNodeIdResult {
    #[serde(rename = "nodeID")]
    pub node_id: node::Id,
}

impl Default for GetNodeIdResult {
    fn default() -> Self {
        Self::default()
    }
}

impl GetNodeIdResult {
    pub fn default() -> Self {
        Self {
            node_id: node::Id::default(),
        }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::info::test_get_node_id --exact --show-output
#[test]
fn test_get_node_id() {
    use std::str::FromStr;

    // ref. https://docs.avax.network/build/avalanchego-apis/info/#infogetnodeid
    let resp: GetNodeIdResponse = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": {
        \"nodeID\": \"NodeID-5mb46qkSBj81k9g9e4VFjGGSbaaSLFRzD\"
    },
    \"id\": 1
}

",
    )
    .unwrap();
    let expected = GetNodeIdResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetNodeIdResult {
            node_id: node::Id::from_str("NodeID-5mb46qkSBj81k9g9e4VFjGGSbaaSLFRzD").unwrap(),
        }),
    };
    assert_eq!(resp, expected);
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetnodeversion>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetNodeVersionResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetNodeVersionResult>,
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetnodeversion>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetNodeVersionResult {
    pub version: String,
    pub database_version: String,
    pub git_commit: String,
    pub vm_versions: VmVersions,
}

impl Default for GetNodeVersionResult {
    fn default() -> Self {
        Self::default()
    }
}

impl GetNodeVersionResult {
    pub fn default() -> Self {
        Self {
            version: String::new(),
            database_version: String::new(),
            git_commit: String::new(),
            vm_versions: VmVersions::default(),
        }
    }
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetnodeversion>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VmVersions {
    pub avm: String,
    pub evm: String,
    pub platform: String,
}

impl Default for VmVersions {
    fn default() -> Self {
        Self::default()
    }
}

impl VmVersions {
    pub fn default() -> Self {
        Self {
            avm: String::new(),
            evm: String::new(),
            platform: String::new(),
        }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::info::test_get_node_version --exact --show-output
#[test]
fn test_get_node_version() {
    let resp: GetNodeVersionResponse = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": {
        \"version\": \"avalanche/1.4.10\",
        \"databaseVersion\": \"v1.4.5\",
        \"gitCommit\": \"a3930fe3fa115c018e71eb1e97ca8cec34db67f1\",
        \"vmVersions\": {
          \"avm\": \"v1.4.10\",
          \"evm\": \"v0.5.5-rc.1\",
          \"platform\": \"v1.4.10\"
        }
    },
    \"id\": 1
}

",
    )
    .unwrap();
    let expected = GetNodeVersionResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetNodeVersionResult {
            version: String::from("avalanche/1.4.10"),
            database_version: String::from("v1.4.5"),
            git_commit: String::from("a3930fe3fa115c018e71eb1e97ca8cec34db67f1"),
            vm_versions: VmVersions {
                avm: String::from("v1.4.10"),
                evm: String::from("v0.5.5-rc.1"),
                platform: String::from("v1.4.10"),
            },
        }),
    };
    assert_eq!(resp, expected);
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetvms>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetVmsResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetVmsResult>,
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogetvms>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetVmsResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vms: Option<HashMap<String, Vec<String>>>,
}

impl Default for GetVmsResult {
    fn default() -> Self {
        Self::default()
    }
}

impl GetVmsResult {
    pub fn default() -> Self {
        Self { vms: None }
    }
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infoisbootstrapped>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct IsBootstrappedResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<IsBootstrappedResult>,
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infoisbootstrapped>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IsBootstrappedResult {
    pub is_bootstrapped: bool,
}

impl Default for IsBootstrappedResult {
    fn default() -> Self {
        Self::default()
    }
}

impl IsBootstrappedResult {
    pub fn default() -> Self {
        Self {
            is_bootstrapped: false,
        }
    }
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogettxfee>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetTxFeeResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetTxFeeResult>,
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogettxfee>
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetTxFeeResult {
    #[serde_as(as = "DisplayFromStr")]
    pub tx_fee: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub create_asset_tx_fee: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub create_subnet_tx_fee: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub transform_subnet_tx_fee: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub create_blockchain_tx_fee: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub add_primary_network_validator_fee: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub add_primary_network_delegator_fee: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub add_subnet_validator_fee: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub add_subnet_delegator_fee: u64,
}

impl Default for GetTxFeeResult {
    fn default() -> Self {
        Self::default()
    }
}

impl GetTxFeeResult {
    pub fn default() -> Self {
        Self {
            tx_fee: 0,
            create_asset_tx_fee: 0,
            create_subnet_tx_fee: 0,
            transform_subnet_tx_fee: 0,
            create_blockchain_tx_fee: 0,
            add_primary_network_validator_fee: 0,
            add_primary_network_delegator_fee: 0,
            add_subnet_validator_fee: 0,
            add_subnet_delegator_fee: 0,
        }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::info::test_get_tx_fee --exact --show-output
#[test]
fn test_get_tx_fee() {
    // ref. <https://docs.avax.network/build/avalanchego-apis/info/#infogettxfee>
    // default local network fees
    let resp: GetTxFeeResponse = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": {
        \"txFee\": \"1000000\",
        \"createAssetTxFee\": \"1000000\",
        \"createSubnetTxFee\": \"100000000\",
        \"transformSubnetTxFee\": \"100000000\",
        \"createBlockchainTxFee\": \"100000000\",
        \"addPrimaryNetworkValidatorFee\": \"0\",
        \"addPrimaryNetworkDelegatorFee\": \"1000000\",
        \"addSubnetValidatorFee\": \"1000000\",
        \"addSubnetDelegatorFee\": \"1000000\"
    },
    \"id\": 1
}

",
    )
    .unwrap();

    let expected = GetTxFeeResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetTxFeeResult {
            tx_fee: 1000000,
            create_asset_tx_fee: 1000000,
            create_subnet_tx_fee: 100000000,
            transform_subnet_tx_fee: 100000000,
            create_blockchain_tx_fee: 100000000,
            add_primary_network_validator_fee: 0,
            add_primary_network_delegator_fee: 1000000,
            add_subnet_validator_fee: 1000000,
            add_subnet_delegator_fee: 1000000,
        }),
    };
    assert_eq!(resp, expected);
}

/// ref. <https://docs.avax.network/apis/avalanchego/apis/info#infouptime>
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct UptimeResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<UptimeResult>,
}

/// ref. <https://docs.avax.network/apis/avalanchego/apis/info#infouptime>
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UptimeResult {
    #[serde_as(as = "DisplayFromStr")]
    pub rewarding_stake_percentage: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub weighted_average_percentage: f64,
}

impl Default for UptimeResult {
    fn default() -> Self {
        Self::default()
    }
}

impl UptimeResult {
    pub fn default() -> Self {
        Self {
            rewarding_stake_percentage: 0_f64,
            weighted_average_percentage: 0_f64,
        }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::info::test_uptime --exact --show-output
#[test]
fn test_uptime() {
    // ref. https://docs.avax.network/apis/avalanchego/apis/info#infouptime
    let resp: UptimeResponse = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": {
        \"rewardingStakePercentage\": \"100.0000\",
        \"weightedAveragePercentage\": \"99.0000\"
    },
    \"id\": 1
}

",
    )
    .unwrap();

    let expected = UptimeResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(UptimeResult {
            rewarding_stake_percentage: 100.0000_f64,
            weighted_average_percentage: 99.0000_f64,
        }),
    };
    assert_eq!(resp, expected);
}
