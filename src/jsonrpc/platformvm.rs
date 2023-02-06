use std::{
    collections::HashMap,
    io::{self, Error, ErrorKind},
};

use crate::{
    codec::serde::hex_0x_utxo::Hex0xUtxo,
    ids::{self, node},
    jsonrpc, platformvm, txs,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// ref. <https://docs.avax.network/apis/avalanchego/apis/p-chain#platformissuetx>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct IssueTxRequest {
    pub jsonrpc: String,
    pub id: u32,

    pub method: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<IssueTxParams>,
}

impl Default for IssueTxRequest {
    fn default() -> Self {
        Self::default()
    }
}

impl IssueTxRequest {
    pub fn default() -> Self {
        Self {
            jsonrpc: String::from(super::DEFAULT_VERSION),
            id: super::DEFAULT_ID,
            method: String::new(),
            params: None,
        }
    }
    pub fn encode_json(&self) -> io::Result<String> {
        serde_json::to_string(&self)
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed to serialize JSON {}", e)))
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IssueTxParams {
    pub tx: String,
    pub encoding: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct IssueTxResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<IssueTxResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<super::ResponseError>,
}

impl Default for IssueTxResponse {
    fn default() -> Self {
        Self::default()
    }
}

impl IssueTxResponse {
    pub fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: 1,
            result: None,
            error: None,
        }
    }
}

/// ref. <https://docs.avax.network/apis/avalanchego/apis/p-chain#platformissuetx>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct IssueTxResult {
    #[serde(rename = "txID")]
    pub tx_id: ids::Id,
}

impl Default for IssueTxResult {
    fn default() -> Self {
        Self::default()
    }
}

impl IssueTxResult {
    pub fn default() -> Self {
        Self {
            tx_id: ids::Id::empty(),
        }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::platformvm::test_issue_tx --exact --show-output
#[test]
fn test_issue_tx() {
    use std::str::FromStr;

    let resp: IssueTxResponse = serde_json::from_str(
        "
    
    {
        \"jsonrpc\": \"2.0\",
        \"result\": {
            \"txID\": \"G3BuH6ytQ2averrLxJJugjWZHTRubzCrUZEXoheG5JMqL5ccY\"
        },
        \"id\": 1
    }
    
    ",
    )
    .unwrap();

    let expected = IssueTxResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(IssueTxResult {
            tx_id: ids::Id::from_str("G3BuH6ytQ2averrLxJJugjWZHTRubzCrUZEXoheG5JMqL5ccY").unwrap(),
        }),
        error: None,
    };
    assert_eq!(resp, expected);
}

/// ref. <https://docs.avax.network/apis/avalanchego/apis/p-chain/#platformgettx>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetTxResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetTxResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<jsonrpc::ResponseError>,
}

impl Default for GetTxResponse {
    fn default() -> Self {
        Self::default()
    }
}

impl GetTxResponse {
    pub fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: 1,
            result: None,
            error: None,
        }
    }
}

/// ref. <https://docs.avax.network/apis/avalanchego/apis/p-chain/#platformgettx>
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
    let parsed_resp: GetTxResponse = serde_json::from_str(
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

/// ref. <https://docs.avax.network/apis/avalanchego/apis/p-chain/#platformgettxstatus>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetTxStatusResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetTxStatusResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<jsonrpc::ResponseError>,
}

impl Default for GetTxStatusResponse {
    fn default() -> Self {
        Self::default()
    }
}

impl GetTxStatusResponse {
    pub fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: 1,
            result: None,
            error: None,
        }
    }
}

/// ref. <https://docs.avax.network/apis/avalanchego/apis/p-chain/#platformgettxstatus>
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
    let resp: GetTxStatusResponse = serde_json::from_str(
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

    let expected = GetTxStatusResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetTxStatusResult {
            status: platformvm::txs::status::Status::Committed,
        }),
        error: None,
    };
    assert_eq!(resp, expected);
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetheight>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetHeightResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetHeightResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<jsonrpc::ResponseError>,
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetheight>
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
    let resp: GetHeightResponse = serde_json::from_str(
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

    let expected = GetHeightResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetHeightResult { height: 0 }),
        error: None,
    };
    assert_eq!(resp, expected);
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/issuing-api-calls>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetUtxosRequest {
    pub jsonrpc: String,
    pub id: u32,

    pub method: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<GetUtxosParams>,
}

impl Default for GetUtxosRequest {
    fn default() -> Self {
        Self::default()
    }
}

impl GetUtxosRequest {
    pub fn default() -> Self {
        Self {
            jsonrpc: String::from(super::DEFAULT_VERSION),
            id: super::DEFAULT_ID,
            method: String::new(),
            params: None,
        }
    }

    pub fn encode_json(&self) -> io::Result<String> {
        serde_json::to_string(&self)
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed to serialize JSON {}", e)))
    }
}

/// ref. <https://docs.avax.network/apis/avalanchego/apis/p-chain#platformgetutxos>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetUtxosParams {
    pub addresses: Vec<String>,
    pub limit: u32,
    pub encoding: String,
}

/// ref. <https://docs.avax.network/apis/avalanchego/apis/p-chain#platformgetutxos>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetUtxosResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetUtxosResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<super::ResponseError>,
}

/// ref. <https://docs.avax.network/apis/avalanchego/apis/p-chain#platformgetutxos>
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetUtxosResult {
    #[serde_as(as = "DisplayFromStr")]
    pub num_fetched: u32,

    #[serde_as(as = "Option<Vec<Hex0xUtxo>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxos: Option<Vec<txs::utxo::Utxo>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_index: Option<super::EndIndex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
}

impl Default for GetUtxosResult {
    fn default() -> Self {
        Self::default()
    }
}

impl GetUtxosResult {
    pub fn default() -> Self {
        Self {
            num_fetched: 0,
            utxos: None,
            end_index: None,
            encoding: None,
        }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::platformvm::test_get_utxos_empty --exact --show-output
#[test]
fn test_get_utxos_empty() {
    // ref. https://docs.avax.network/apis/avalanchego/apis/p-chain#platformgetutxos
    let resp: GetUtxosResponse = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": {
        \"numFetched\": \"0\",
        \"utxos\": [],
        \"endIndex\": {
            \"address\": \"P-custom152qlr6zunz7nw2kc4lfej3cn3wk46u3002k4w5\",
            \"utxo\": \"11111111111111111111111111111111LpoYY\"
        },
        \"encoding\":\"hex\"
    },
    \"id\": 1
}

",
    )
    .unwrap();

    let expected = GetUtxosResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetUtxosResult {
            num_fetched: 0,
            utxos: Some(Vec::new()),
            end_index: Some(super::EndIndex {
                address: String::from("P-custom152qlr6zunz7nw2kc4lfej3cn3wk46u3002k4w5"),
                utxo: String::from("11111111111111111111111111111111LpoYY"),
            }),
            encoding: Some(String::from("hex")),
        }),
        error: None,
    };
    assert_eq!(resp, expected);
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::platformvm::test_get_utxos_non_empty --exact --show-output
#[test]
fn test_get_utxos_non_empty() {
    // ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetbalance
    let resp: GetUtxosResponse = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": {
        \"numFetched\": \"1\",
        \"utxos\": [
            \"0x000000000000000000000000000000000000000000000000000000000000000000000000000088eec2e099c6a528e689618e8721e04ae85ea574c7a15a7968644d14d54780140000000702c68af0bb1400000000000000000000000000010000000165844a05405f3662c1928142c6c2a783ef871de939b564db\"
        ],
        \"endIndex\": {
            \"address\": \"X-avax1x459sj0ssujguq723cljfty4jlae28evjzt7xz\",
            \"utxo\": \"LUC1cmcxnfNR9LdkACS2ccGKLEK7SYqB4gLLTycQfg1koyfSq\"
        },
        \"encoding\": \"hex\"
    },
    \"id\": 1
}

",
    )
    .unwrap();

    let raw_utxo =  String::from("0x000000000000000000000000000000000000000000000000000000000000000000000000000088eec2e099c6a528e689618e8721e04ae85ea574c7a15a7968644d14d54780140000000702c68af0bb1400000000000000000000000000010000000165844a05405f3662c1928142c6c2a783ef871de939b564db");
    let utxo = txs::utxo::Utxo::from_hex(&raw_utxo).unwrap();

    let expected = GetUtxosResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetUtxosResult {
            num_fetched: 1,
            utxos: Some(vec![utxo]),
            end_index: Some(super::EndIndex {
                address: String::from("X-avax1x459sj0ssujguq723cljfty4jlae28evjzt7xz"),
                utxo: String::from("LUC1cmcxnfNR9LdkACS2ccGKLEK7SYqB4gLLTycQfg1koyfSq"),
            }),
            encoding: Some(String::from("hex")),
        }),
        error: None,
    };
    assert_eq!(resp, expected);
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetbalance>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetBalanceResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetBalanceResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<jsonrpc::ResponseError>,
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetbalance>
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

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<HashMap<_, DisplayFromStr>>")]
    pub balances: Option<HashMap<String, u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<HashMap<_, DisplayFromStr>>")]
    pub unlockeds: Option<HashMap<String, u64>>,

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
            balances: None,
            unlockeds: None,
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
    let resp: GetBalanceResponse = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": {
        \"balance\": \"20000000000000000\",
        \"unlocked\": \"10000000000000000\",
        \"lockedStakeable\": \"10000000000000000\",
        \"lockedNotStakeable\": \"0\",
        \"balances\": {
            \"2ZKbwERx36B5WrYesQGAeTV4NTo4dx6j8svkjwAEix89ZPencR\": \"147573952589676412\"
        },
        \"unlockeds\": {
            \"2ZKbwERx36B5WrYesQGAeTV4NTo4dx6j8svkjwAEix89ZPencR\": \"147573952589676412\"
        },
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

    let mut h = HashMap::new();
    h.insert(
        "2ZKbwERx36B5WrYesQGAeTV4NTo4dx6j8svkjwAEix89ZPencR".to_string(),
        147573952589676412_u64,
    );

    let expected = GetBalanceResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetBalanceResult {
            balance: 20000000000000000,
            unlocked: 10000000000000000,

            locked_stakeable: Some(10000000000000000),
            locked_not_stakeable: Some(0),

            balances: Some(h.clone()),
            unlockeds: Some(h.clone()),

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

/// ref. <https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators>
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GetCurrentValidatorsResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetCurrentValidatorsResult>,
}

impl Default for GetCurrentValidatorsResponse {
    fn default() -> Self {
        Self::default()
    }
}

impl GetCurrentValidatorsResponse {
    pub fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: 1,
            result: Some(GetCurrentValidatorsResult::default()),
        }
    }
}

/// ref. <https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators>
/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/platformvm#ClientPermissionlessValidator>
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

/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/platformvm#ClientPermissionlessValidator>
/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/platformvm#ClientStaker>
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

    /// None for subnet validator.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(rename = "stakeAmount")]
    pub stake_amount: Option<u64>,

    #[serde(rename = "nodeID")]
    pub node_id: node::Id,

    /// None for subnet validator.
    #[serde(rename = "rewardOwner", skip_serializing_if = "Option::is_none")]
    pub reward_owner: Option<ApiOwner>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(rename = "potentialReward")]
    pub potential_reward: Option<u64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(rename = "delegationFee", skip_serializing_if = "Option::is_none")]
    pub delegation_fee: Option<f32>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,

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
            stake_amount: None,
            node_id: node::Id::empty(),
            reward_owner: None,
            potential_reward: None,
            delegation_fee: None,
            uptime: None,
            connected: None,
            staked: None,
            delegators: None,
        }
    }
}

/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/platformvm#APIOwner>
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

/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/platformvm#ClientPermissionlessValidator>
/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/platformvm#ClientStaker>
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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

    /// None for subnet validator.
    #[serde(rename = "rewardOwner", skip_serializing_if = "Option::is_none")]
    pub reward_owner: Option<ApiOwner>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(rename = "potentialReward")]
    pub potential_reward: Option<u64>,
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
            reward_owner: None,
            potential_reward: None,
        }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::platformvm::test_get_current_validators --exact --show-output
#[test]
fn test_get_current_validators() {
    use std::str::FromStr;

    // ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators
    let resp: GetCurrentValidatorsResponse = serde_json::from_str(
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

    let expected = GetCurrentValidatorsResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetCurrentValidatorsResult {
            validators: Some(<Vec<ApiPrimaryValidator>>::from([
                ApiPrimaryValidator {
                    tx_id: ids::Id::from_str("KPkPo9EerKZhSwrA8NfLTVWsgr16Ntu8Ei4ci7GF7t75szrcQ")
                        .unwrap(),
                    start_time: 1648312635,
                    end_time: 1679843235,
                    stake_amount: Some(100000000000000000),
                    node_id: node::Id::from_str("NodeID-5wVq6KkSK3p4wQFmiVHCDq2zdg8unchaE")
                        .unwrap(),
                    reward_owner: Some(ApiOwner {
                        locktime: 0,
                        threshold: 1,
                        addresses: vec![
                            "P-custom1vkzy5p2qtumx9svjs9pvds48s0hcw80f962vrs".to_string()
                        ],
                    }),
                    potential_reward: Some(79984390135364555),
                    delegation_fee: Some(6.25),
                    uptime: Some(1.0),
                    connected: Some(true),
                    ..ApiPrimaryValidator::default()
                },
                ApiPrimaryValidator {
                    tx_id: ids::Id::from_str("EjKZm5eEajWu151cfPms7PvMjyaQk36qTSz1MfnZRU5x5bNxz")
                        .unwrap(),

                    start_time: 1648312635,
                    end_time: 1679848635,
                    stake_amount: Some(100000000000000000),
                    node_id: node::Id::from_str("NodeID-JLR7d6z9cwCbkoPcPsnjkm6gq4xz7c4oT")
                        .unwrap(),
                    reward_owner: Some(ApiOwner {
                        locktime: 0,
                        threshold: 1,
                        addresses: vec![
                            "P-custom1vkzy5p2qtumx9svjs9pvds48s0hcw80f962vrs".to_string()
                        ],
                    }),
                    potential_reward: Some(77148186230865960),
                    delegation_fee: Some(6.25),
                    uptime: Some(1.0),
                    connected: Some(true),
                    ..ApiPrimaryValidator::default()
                },
            ])),
        }),
    };
    assert_eq!(resp, expected);
}

/// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/platformvm#APIUTXO>
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
