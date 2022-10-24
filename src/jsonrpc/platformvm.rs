use crate::{
    ids::{self, node},
    jsonrpc, platformvm, txs,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// ref. https://docs.avax.network/apis/avalanchego/apis/p-chain/#platformgettx
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetTx {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetTxResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<jsonrpc::ResponseError>,
}

impl Default for GetTx {
    fn default() -> Self {
        Self::default()
    }
}

impl GetTx {
    pub fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: 1,
            result: None,
            error: None,
        }
    }
}

/// ref. https://docs.avax.network/apis/avalanchego/apis/p-chain/#platformgettx
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetTxResult {
    pub tx: platformvm::txs::Tx,
    pub encoding: String,
}

impl Default for GetTxResult {
    fn default() -> Self {
        Self::default()
    }
}

impl GetTxResult {
    pub fn default() -> Self {
        Self {
            tx: platformvm::txs::Tx::default(),
            encoding: String::new(),
        }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::platformvm::test_get_tx --exact --show-output
#[test]
fn test_get_tx() {
    let parsed_resp: GetTx = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": {
        \"tx\": {
            \"unsignedTx\": {
                \"networkID\": 1000000,
                \"blockchainID\": \"11111111111111111111111111111111LpoYY\",
                \"outputs\": [
                    {
                        \"assetID\": \"u8aaQ7MxyW32iHuP2xMXgYPrWYAsSbh8RJV9C6p1UeuGvqR3\",
                        \"fxID\": \"spdxUxVJQbX85MGxMHbKw1sHxMnSqJ3QBzDyDYEP3h6TLuxqQ\",
                        \"output\": {
                            \"addresses\": [
                                \"P-custom12szthht8tnl455u4mz3ns3nvvkel8ezvw2n8cx\"
                            ],
                            \"amount\": 245952587549460688,
                            \"locktime\": 0,
                            \"threshold\": 1
                        }
                    }
                ],
                \"inputs\": [
                    {
                        \"txID\": \"nN5QsURgEpM8D3e9q8FonS4EE13mnaBDtnQmgSwwUfBZ6FSW1\",
                        \"outputIndex\": 0,
                        \"assetID\": \"u8aaQ7MxyW32iHuP2xMXgYPrWYAsSbh8RJV9C6p1UeuGvqR3\",
                        \"fxID\": \"spdxUxVJQbX85MGxMHbKw1sHxMnSqJ3QBzDyDYEP3h6TLuxqQ\",
                        \"input\": {
                            \"amount\": 245952587649460688,
                            \"signatureIndices\": [
                                0
                            ]
                        }
                    }
                ],
                \"memo\": \"0x\",
                \"owner\": {
                    \"addresses\": [
                        \"P-custom12szthht8tnl455u4mz3ns3nvvkel8ezvw2n8cx\"
                    ],
                    \"locktime\": 0,
                    \"threshold\": 1
                }
            },
            \"credentials\": [
                {
                    \"signatures\": [
                        \"0xcb356822dc8990672b5777ec50b57da91baf572240e7d4e9e38f26ec9dbdfd8e376fdc5f30769b842668cd8d81bd71db926dfbe326585137d363566ee500369f01\"
                    ]
                }
            ]
        },
        \"encoding\": \"json\"
    },
    \"id\": 1
}

",
    )
    .unwrap();

    assert_eq!(parsed_resp.jsonrpc, "2.0");
    assert_eq!(parsed_resp.result.clone().unwrap().encoding, "json");
}

/// ref. https://docs.avax.network/apis/avalanchego/apis/p-chain/#platformgettxstatus
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetTxStatus {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetTxStatusResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<jsonrpc::ResponseError>,
}

impl Default for GetTxStatus {
    fn default() -> Self {
        Self::default()
    }
}

impl GetTxStatus {
    pub fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: 1,
            result: None,
            error: None,
        }
    }
}

/// ref. https://docs.avax.network/apis/avalanchego/apis/p-chain/#platformgettxstatus
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetTxStatusResult {
    #[serde_as(as = "DisplayFromStr")]
    pub status: platformvm::txs::status::Status,
}

impl Default for GetTxStatusResult {
    fn default() -> Self {
        Self::default()
    }
}

impl GetTxStatusResult {
    pub fn default() -> Self {
        Self {
            status: platformvm::txs::status::Status::Unknown(String::new()),
        }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::platformvm::test_get_tx_status --exact --show-output
#[test]
fn test_get_tx_status() {
    let resp: GetTxStatus = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": {
        \"status\": \"Committed\"
    },
    \"id\": 1
}

",
    )
    .unwrap();

    let expected = GetTxStatus {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetTxStatusResult {
            status: platformvm::txs::status::Status::Committed,
        }),
        error: None,
    };
    assert_eq!(resp, expected);
}

/// ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetheight
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetHeight {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetHeightResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<jsonrpc::ResponseError>,
}

/// ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetheight
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetHeightResult {
    #[serde_as(as = "DisplayFromStr")]
    pub height: u64,
}

impl Default for GetHeightResult {
    fn default() -> Self {
        Self::default()
    }
}

impl GetHeightResult {
    pub fn default() -> Self {
        Self { height: 0 }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::platformvm::test_get_height --exact --show-output
#[test]
fn test_get_height() {
    // ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetheight
    let resp: GetHeight = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": {
        \"height\": \"0\"
    },
    \"id\": 1
}

",
    )
    .unwrap();

    let expected = GetHeight {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetHeightResult { height: 0 }),
        error: None,
    };
    assert_eq!(resp, expected);
}

/// ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetbalance
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetBalance {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetBalanceResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<jsonrpc::ResponseError>,
}

/// ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetbalance
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetBalanceResult {
    #[serde_as(as = "DisplayFromStr")]
    pub balance: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub unlocked: u64,

    #[serde(rename = "lockedStakeable", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub locked_stakeable: Option<u64>,
    #[serde(rename = "lockedNotStakeable", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub locked_not_stakeable: Option<u64>,

    #[serde(rename = "utxoIDs", skip_serializing_if = "Option::is_none")]
    pub utxo_ids: Option<Vec<txs::utxo::Id>>,
}

impl Default for GetBalanceResult {
    fn default() -> Self {
        Self::default()
    }
}

impl GetBalanceResult {
    pub fn default() -> Self {
        Self {
            balance: 0,
            unlocked: 0,
            locked_stakeable: None,
            locked_not_stakeable: None,
            utxo_ids: None,
        }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::platformvm::test_get_balance --exact --show-output
#[test]
fn test_get_balance() {
    use crate::ids;
    use std::str::FromStr;

    // ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetbalance
    let resp: GetBalance = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": {
        \"balance\": \"20000000000000000\",
        \"unlocked\": \"10000000000000000\",
        \"lockedStakeable\": \"10000000000000000\",
        \"lockedNotStakeable\": \"0\",
        \"utxoIDs\": [
            {
                \"txID\": \"11111111111111111111111111111111LpoYY\",
                \"outputIndex\": 1
            },
            {
                \"txID\": \"11111111111111111111111111111111LpoYY\",
                \"outputIndex\": 0
            }
        ]
    },
    \"id\": 1
}

",
    )
    .unwrap();

    let expected = GetBalance {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetBalanceResult {
            balance: 20000000000000000,
            unlocked: 10000000000000000,
            locked_stakeable: Some(10000000000000000),
            locked_not_stakeable: Some(0),
            utxo_ids: Some(vec![
                txs::utxo::Id {
                    tx_id: ids::Id::from_str("11111111111111111111111111111111LpoYY").unwrap(),
                    output_index: 1,
                    ..txs::utxo::Id::default()
                },
                txs::utxo::Id {
                    tx_id: ids::Id::from_str("11111111111111111111111111111111LpoYY").unwrap(),
                    output_index: 0,
                    ..txs::utxo::Id::default()
                },
            ]),
        }),
        error: None,
    };
    assert_eq!(resp, expected);
}

/// ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GetCurrentValidators {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetCurrentValidatorsResult>,
}

impl Default for GetCurrentValidators {
    fn default() -> Self {
        Self::default()
    }
}

impl GetCurrentValidators {
    pub fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: 1,
            result: Some(GetCurrentValidatorsResult::default()),
        }
    }
}

/// ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators
/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/platformvm#APIPrimaryValidator
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GetCurrentValidatorsResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<ApiPrimaryValidator>>,
}

impl Default for GetCurrentValidatorsResult {
    fn default() -> Self {
        Self::default()
    }
}

impl GetCurrentValidatorsResult {
    pub fn default() -> Self {
        Self { validators: None }
    }
}

/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/platformvm#APIPrimaryValidator
/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/platformvm#APIStaker
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ApiPrimaryValidator {
    #[serde(rename = "txID")]
    pub tx_id: ids::Id,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "startTime")]
    pub start_time: u64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "endTime")]
    pub end_time: u64,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u64>,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "stakeAmount")]
    pub stake_amount: u64,

    #[serde(rename = "nodeID")]
    pub node_id: node::Id,

    #[serde(rename = "rewardOwner")]
    pub reward_owner: ApiOwner,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "potentialReward")]
    pub potential_reward: u64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "delegationFee")]
    pub delegation_fee: f32,

    #[serde_as(as = "DisplayFromStr")]
    pub uptime: f32,
    pub connected: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegators: Option<Vec<ApiPrimaryDelegator>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub staked: Option<Vec<ApiUtxo>>,
}

impl Default for ApiPrimaryValidator {
    fn default() -> Self {
        Self::default()
    }
}

impl ApiPrimaryValidator {
    pub fn default() -> Self {
        Self {
            tx_id: ids::Id::empty(),
            start_time: 0,
            end_time: 0,
            weight: None,
            stake_amount: 0,
            node_id: node::Id::empty(),
            reward_owner: ApiOwner::default(),
            potential_reward: 0_u64,
            delegation_fee: 0_f32,
            uptime: 0_f32,
            connected: false,
            staked: None,
            delegators: None,
        }
    }
}

/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/platformvm#APIOwner
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct ApiOwner {
    #[serde_as(as = "DisplayFromStr")]
    pub locktime: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub threshold: u32,
    pub addresses: Vec<String>,
}

impl Default for ApiOwner {
    fn default() -> Self {
        Self::default()
    }
}

impl ApiOwner {
    pub fn default() -> Self {
        Self {
            locktime: 0,
            threshold: 0,
            addresses: Vec::new(),
        }
    }
}

/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/platformvm#APIPrimaryValidator
/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/platformvm#APIStaker
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ApiPrimaryDelegator {
    #[serde(rename = "txID")]
    pub tx_id: ids::Id,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "startTime")]
    pub start_time: u64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "endTime")]
    pub end_time: u64,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u64>,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "stakeAmount")]
    pub stake_amount: u64,

    #[serde(rename = "nodeID")]
    pub node_id: node::Id,

    #[serde(rename = "rewardOwner")]
    pub reward_owner: ApiOwner,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "potentialReward")]
    pub potential_reward: u64,
}

impl Default for ApiPrimaryDelegator {
    fn default() -> Self {
        Self::default()
    }
}

impl ApiPrimaryDelegator {
    pub fn default() -> Self {
        Self {
            tx_id: ids::Id::empty(),
            start_time: 0,
            end_time: 0,
            weight: None,
            stake_amount: 0,
            node_id: node::Id::empty(),
            reward_owner: ApiOwner::default(),
            potential_reward: 0,
        }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::platformvm::test_get_current_validators --exact --show-output
#[test]
fn test_get_current_validators() {
    use std::str::FromStr;

    // ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators
    let resp: GetCurrentValidators = serde_json::from_str(
        "
{
    \"jsonrpc\": \"2.0\",
    \"result\": {
        \"validators\": [
            {
                \"txID\": \"KPkPo9EerKZhSwrA8NfLTVWsgr16Ntu8Ei4ci7GF7t75szrcQ\",
                \"startTime\": \"1648312635\",
                \"endTime\": \"1679843235\",
                \"stakeAmount\": \"100000000000000000\",
                \"nodeID\": \"NodeID-5wVq6KkSK3p4wQFmiVHCDq2zdg8unchaE\",
                \"rewardOwner\": {
                    \"locktime\": \"0\",
                    \"threshold\": \"1\",
                    \"addresses\": [
                        \"P-custom1vkzy5p2qtumx9svjs9pvds48s0hcw80f962vrs\"
                    ]
                },
                \"potentialReward\": \"79984390135364555\",
                \"delegationFee\": \"6.2500\",
                \"uptime\": \"1.0000\",
                \"connected\": true,
                \"delegators\": null
            },
            {
                \"txID\": \"EjKZm5eEajWu151cfPms7PvMjyaQk36qTSz1MfnZRU5x5bNxz\",
                \"startTime\": \"1648312635\",
                \"endTime\": \"1679848635\",
                \"stakeAmount\": \"100000000000000000\",
                \"nodeID\": \"NodeID-JLR7d6z9cwCbkoPcPsnjkm6gq4xz7c4oT\",
                \"rewardOwner\": {
                    \"locktime\": \"0\",
                    \"threshold\": \"1\",
                    \"addresses\": [
                        \"P-custom1vkzy5p2qtumx9svjs9pvds48s0hcw80f962vrs\"
                    ]
                },
                \"potentialReward\": \"77148186230865960\",
                \"delegationFee\": \"6.2500\",
                \"uptime\": \"1.0000\",
                \"connected\": true,
                \"delegators\": null
            }
        ]
    },
    \"id\": 1
}

",
    )
    .unwrap();

    let expected = GetCurrentValidators {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetCurrentValidatorsResult {
            validators: Some(<Vec<ApiPrimaryValidator>>::from([
                ApiPrimaryValidator {
                    tx_id: ids::Id::from_str("KPkPo9EerKZhSwrA8NfLTVWsgr16Ntu8Ei4ci7GF7t75szrcQ")
                        .unwrap(),
                    start_time: 1648312635,
                    end_time: 1679843235,
                    stake_amount: 100000000000000000,
                    node_id: node::Id::from_str("NodeID-5wVq6KkSK3p4wQFmiVHCDq2zdg8unchaE")
                        .unwrap(),
                    reward_owner: ApiOwner {
                        locktime: 0,
                        threshold: 1,
                        addresses: vec![
                            "P-custom1vkzy5p2qtumx9svjs9pvds48s0hcw80f962vrs".to_string()
                        ],
                    },
                    potential_reward: 79984390135364555,
                    delegation_fee: 6.25,
                    uptime: 1.0,
                    connected: true,
                    ..ApiPrimaryValidator::default()
                },
                ApiPrimaryValidator {
                    tx_id: ids::Id::from_str("EjKZm5eEajWu151cfPms7PvMjyaQk36qTSz1MfnZRU5x5bNxz")
                        .unwrap(),

                    start_time: 1648312635,
                    end_time: 1679848635,
                    stake_amount: 100000000000000000,
                    node_id: node::Id::from_str("NodeID-JLR7d6z9cwCbkoPcPsnjkm6gq4xz7c4oT")
                        .unwrap(),
                    reward_owner: ApiOwner {
                        locktime: 0,
                        threshold: 1,
                        addresses: vec![
                            "P-custom1vkzy5p2qtumx9svjs9pvds48s0hcw80f962vrs".to_string()
                        ],
                    },
                    potential_reward: 77148186230865960,
                    delegation_fee: 6.25,
                    uptime: 1.0,
                    connected: true,
                    ..ApiPrimaryValidator::default()
                },
            ])),
        }),
    };
    assert_eq!(resp, expected);
}

/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/platformvm#APIUTXO
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct ApiUtxo {
    #[serde_as(as = "DisplayFromStr")]
    pub locktime: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub amount: u64,

    pub address: String,
    pub message: Option<String>,
}

impl Default for ApiUtxo {
    fn default() -> Self {
        Self::default()
    }
}

impl ApiUtxo {
    pub fn default() -> Self {
        Self {
            locktime: 0,
            amount: 0,
            address: String::new(),
            message: None,
        }
    }
}
