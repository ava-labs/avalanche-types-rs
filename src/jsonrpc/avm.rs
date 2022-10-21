use std::{io, str::FromStr};

use crate::{choices, ids, jsonrpc, txs};
use serde::{Deserialize, Serialize};

/// ref. https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgettxstatus
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

/// ref. https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgettxstatus
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetTxStatusResult {
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

/// ref. https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgettxstatus
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct RawGetTxStatusResponse {
    pub jsonrpc: String,
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<RawGetTxStatusResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<jsonrpc::ResponseError>,
}

/// ref. https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgettxstatus
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct RawGetTxStatusResult {
    pub status: String,
}

impl RawGetTxStatusResponse {
    pub fn convert(&self) -> io::Result<GetTxStatusResponse> {
        let result = {
            if self.result.is_some() {
                let mut result = GetTxStatusResult::default();

                let status = self
                    .result
                    .clone()
                    .expect("unexpected None result")
                    .status
                    .clone();
                result.status = choices::status::Status::from(status.as_str());

                Some(result)
            } else {
                Some(GetTxStatusResult::default())
            }
        };
        Ok(GetTxStatusResponse {
            jsonrpc: self.jsonrpc.clone(),
            id: self.id,
            result,
            error: self.error.clone(),
        })
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::test_convert_get_tx_status --exact --show-output
#[test]
fn test_convert_get_tx_status() {
    // ref. https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgettxstatus
    let resp: RawGetTxStatusResponse = serde_json::from_str(
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
    let parsed = resp.convert().unwrap();
    let expected = GetTxStatusResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetTxStatusResult {
            status: choices::status::Status::Accepted,
        }),
        error: None,
    };
    assert_eq!(parsed, expected);
}

/// ref. https://docs.avax.network/build/avalanchego-apis/x-chain#avmgetbalance
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetBalanceResponse {
    pub jsonrpc: String,
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetBalanceResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<jsonrpc::ResponseError>,
}

/// ref. https://docs.avax.network/build/avalanchego-apis/x-chain#avmgetbalance
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetBalanceResult {
    pub balance: u64,
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

/// ref. https://docs.avax.network/build/avalanchego-apis/x-chain#avmgetbalance
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct RawGetBalanceResponse {
    jsonrpc: String,
    id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<RawGetBalanceResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<jsonrpc::ResponseError>,
}

/// ref. https://docs.avax.network/build/avalanchego-apis/x-chain#avmgetbalance
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct RawGetBalanceResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    balance: Option<String>,
    #[serde(rename = "utxoIDs", skip_serializing_if = "Option::is_none")]
    utxo_ids: Option<Vec<crate::jsonrpc::RawUtxoId>>,
}

impl RawGetBalanceResponse {
    pub fn convert(&self) -> io::Result<GetBalanceResponse> {
        let result = {
            if self.result.is_some() {
                let mut result = GetBalanceResult::default();
                if self
                    .result
                    .clone()
                    .expect("unexpected None result")
                    .balance
                    .is_some()
                {
                    let balance = self
                        .result
                        .clone()
                        .expect("unexpected None result")
                        .balance
                        .expect("unexpected None balance");
                    result.balance = balance.parse::<u64>().unwrap();
                }

                if self
                    .result
                    .clone()
                    .expect("unexpected None result")
                    .utxo_ids
                    .is_some()
                {
                    let utxo_ids = self
                        .result
                        .clone()
                        .expect("unexpected None result")
                        .utxo_ids
                        .expect("unexpected None utxo_ids");
                    let mut converts: Vec<txs::utxo::Id> = Vec::new();
                    for v in utxo_ids.iter() {
                        let converted = v.convert()?;
                        converts.push(converted);
                    }
                    result.utxo_ids = Some(converts);
                }
                Some(result)
            } else {
                None
            }
        };
        Ok(GetBalanceResponse {
            jsonrpc: self.jsonrpc.clone(),
            id: self.id,
            result,
            error: self.error.clone(),
        })
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- api::avm::test_get_balance_response_convert --exact --show-output
#[test]
fn test_get_balance_response_convert() {
    // ref. https://docs.avax.network/build/avalanchego-apis/x-chain#avmgetbalance
    let resp: RawGetBalanceResponse = serde_json::from_str(
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
    let parsed = resp.convert().unwrap();
    let expected = GetBalanceResponse {
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
    assert_eq!(parsed, expected);
}

/// ref. https://docs.avax.network/build/avalanchego-apis/x-chain/#avmgetassetdescription
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetAssetDescriptionResponse {
    pub jsonrpc: String,
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetAssetDescriptionResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<jsonrpc::ResponseError>,
}

/// ref. https://docs.avax.network/build/avalanchego-apis/x-chain/#avmgetassetdescription
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetAssetDescriptionResult {
    pub asset_id: ids::Id,
    pub name: String,
    pub symbol: String,
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

/// ref. https://docs.avax.network/build/avalanchego-apis/x-chain/#avmgetassetdescription
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct RawGetAssetDescriptionResponse {
    jsonrpc: String,
    id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<RawGetAssetDescriptionResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<jsonrpc::ResponseError>,
}

/// ref. https://docs.avax.network/build/avalanchego-apis/x-chain/#avmgetassetdescription
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct RawGetAssetDescriptionResult {
    #[serde(rename = "assetID")]
    asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    denomination: Option<String>,
}

impl RawGetAssetDescriptionResponse {
    pub fn convert(&self) -> io::Result<GetAssetDescriptionResponse> {
        let result = {
            if self.result.is_some() {
                let mut result = GetAssetDescriptionResult::default();
                let asset_id = self
                    .result
                    .clone()
                    .expect("unexpected None result")
                    .asset_id;
                result.asset_id = {
                    if asset_id.is_empty() {
                        ids::Id::empty()
                    } else {
                        ids::Id::from_str(&asset_id).unwrap()
                    }
                };

                let name = self
                    .result
                    .clone()
                    .expect("unexpected None result")
                    .name
                    .unwrap_or_default();
                result.name = name;

                let symbol = self
                    .result
                    .clone()
                    .expect("unexpected None result")
                    .symbol
                    .unwrap_or_default();
                result.symbol = symbol;

                let denomination = self
                    .result
                    .clone()
                    .expect("unexpected None result")
                    .denomination;
                result.denomination = {
                    if let Some(d) = denomination {
                        d.parse::<usize>().unwrap()
                    } else {
                        0_usize
                    }
                };
                Some(result)
            } else {
                None
            }
        };
        Ok(GetAssetDescriptionResponse {
            jsonrpc: self.jsonrpc.clone(),
            id: self.id,
            result,
            error: self.error.clone(),
        })
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- api::avm::test_get_asset_description_response_convert --exact --show-output
#[test]
fn test_get_asset_description_response_convert() {
    // ref. https://docs.avax.network/build/avalanchego-apis/x-chain/#avmgetassetdescription
    let resp: RawGetAssetDescriptionResponse = serde_json::from_str(
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
    let parsed = resp.convert().unwrap();
    let expected = GetAssetDescriptionResponse {
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
    assert_eq!(parsed, expected);
}
