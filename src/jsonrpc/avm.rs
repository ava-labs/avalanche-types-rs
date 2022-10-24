use crate::{choices, ids, jsonrpc, txs};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// ref. https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgettxstatus
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
            result: Some(GetTxStatusResult::default()),
            error: None,
        }
    }
}

/// ref. https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgettxstatus
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetTxStatusResult {
    #[serde_as(as = "DisplayFromStr")]
    pub status: choices::status::Status,
}

impl Default for GetTxStatusResult {
    fn default() -> Self {
        Self::default()
    }
}

impl GetTxStatusResult {
    pub fn default() -> Self {
        Self {
            status: choices::status::Status::Unknown(String::new()),
        }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::avm::test_get_tx_status --exact --show-output
#[test]
fn test_get_tx_status() {
    // ref. https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgettxstatus
    let resp: GetTxStatus = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": {
        \"status\": \"Accepted\"
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
            status: choices::status::Status::Accepted,
        }),
        error: None,
    };
    assert_eq!(resp, expected);
}

/// ref. https://docs.avax.network/build/avalanchego-apis/x-chain#avmgetbalance
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetBalance {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetBalanceResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<jsonrpc::ResponseError>,
}

/// ref. https://docs.avax.network/build/avalanchego-apis/x-chain#avmgetbalance
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetBalanceResult {
    #[serde_as(as = "DisplayFromStr")]
    pub balance: u64,

    #[serde(rename = "utxoIDs")]
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
            utxo_ids: None,
        }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::avm::test_get_balance --exact --show-output
#[test]
fn test_get_balance() {
    use std::str::FromStr;

    // ref. https://docs.avax.network/build/avalanchego-apis/x-chain#avmgetbalance
    let resp: GetBalance = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": {
        \"balance\": \"299999999999900\",
        \"utxoIDs\": [
            {
                \"txID\": \"WPQdyLNqHfiEKp4zcCpayRHYDVYuh1hqs9c1RqgZXS4VPgdvo\",
                \"outputIndex\": 1
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
            balance: 299999999999900,
            utxo_ids: Some(vec![txs::utxo::Id {
                tx_id: ids::Id::from_str("WPQdyLNqHfiEKp4zcCpayRHYDVYuh1hqs9c1RqgZXS4VPgdvo")
                    .unwrap(),
                output_index: 1,
                ..txs::utxo::Id::default()
            }]),
        }),
        error: None,
    };
    assert_eq!(resp, expected);
}

/// ref. https://docs.avax.network/build/avalanchego-apis/x-chain/#avmgetassetdescription
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetAssetDescription {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetAssetDescriptionResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<jsonrpc::ResponseError>,
}

/// ref. https://docs.avax.network/build/avalanchego-apis/x-chain/#avmgetassetdescription
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetAssetDescriptionResult {
    #[serde(rename = "assetID")]
    pub asset_id: ids::Id,

    pub name: String,
    pub symbol: String,

    #[serde_as(as = "DisplayFromStr")]
    pub denomination: usize,
}

impl Default for GetAssetDescriptionResult {
    fn default() -> Self {
        Self::default()
    }
}

impl GetAssetDescriptionResult {
    pub fn default() -> Self {
        Self {
            asset_id: ids::Id::default(),
            name: String::new(),
            symbol: String::new(),
            denomination: 0,
        }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::avm::test_get_asset_description --exact --show-output
#[test]
fn test_get_asset_description() {
    use std::str::FromStr;

    // ref. https://docs.avax.network/build/avalanchego-apis/x-chain/#avmgetassetdescription
    let resp: GetAssetDescription = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": {
        \"assetID\": \"2fombhL7aGPwj3KH4bfrmJwW6PVnMobf9Y2fn9GwxiAAJyFDbe\",
        \"name\": \"Avalanche\",
        \"symbol\": \"AVAX\",
        \"denomination\": \"9\"
    },
    \"id\": 1
}

",
    )
    .unwrap();

    let expected = GetAssetDescription {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetAssetDescriptionResult {
            asset_id: ids::Id::from_str("2fombhL7aGPwj3KH4bfrmJwW6PVnMobf9Y2fn9GwxiAAJyFDbe")
                .unwrap(),
            name: String::from("Avalanche"),
            symbol: String::from("AVAX"),
            denomination: 9,
        }),
        error: None,
    };
    assert_eq!(resp, expected);
}
