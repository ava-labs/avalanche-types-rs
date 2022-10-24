pub mod avm;
pub mod eth;
pub mod health;
pub mod info;
pub mod platformvm;

use crate::{formatting::serde::hex_0x_utxo::HexUtxo, ids, txs};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::{
    collections::HashMap,
    io::{self, Error, ErrorKind},
};

pub const DEFAULT_VERSION: &str = "2.0";
pub const DEFAULT_ID: u32 = 1;

/// ref. https://www.jsonrpc.org/specification
/// ref. https://docs.avax.network/build/avalanchego-apis/issuing-api-calls
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Data {
    pub jsonrpc: String,
    pub id: u32,

    pub method: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, String>>,
}

impl Default for Data {
    fn default() -> Self {
        Self::default()
    }
}

impl Data {
    pub fn default() -> Self {
        Self {
            jsonrpc: String::from(DEFAULT_VERSION),
            id: DEFAULT_ID,
            method: String::new(),
            params: None,
        }
    }

    pub fn encode_json(&self) -> io::Result<String> {
        match serde_json::to_string(&self) {
            Ok(s) => Ok(s),
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("failed to serialize to JSON {}", e),
                ));
            }
        }
    }
}

/// ref. https://docs.avax.network/build/avalanchego-apis/c-chain#eth_getassetbalance
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct DataWithParamsArray {
    pub jsonrpc: String,
    pub id: u32,

    pub method: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<String>>,
}

impl Default for DataWithParamsArray {
    fn default() -> Self {
        Self::default()
    }
}

impl DataWithParamsArray {
    pub fn default() -> Self {
        Self {
            jsonrpc: String::from(DEFAULT_VERSION),
            id: DEFAULT_ID,
            method: String::new(),
            params: None,
        }
    }

    pub fn encode_json(&self) -> io::Result<String> {
        match serde_json::to_string(&self) {
            Ok(s) => Ok(s),
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("failed to serialize to JSON {}", e),
                ));
            }
        }
    }
}

/// ref. https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgetutxos
/// ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetutxos
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EndIndex {
    pub address: String,
    pub utxo: String,
}

/// ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IssueTxRequest {
    pub tx: String,
    pub encoding: String,
}

/// ref. https://docs.avax.network/build/avalanchego-apis/issuing-api-calls
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct DataForIssueTx {
    pub jsonrpc: String,
    pub id: u32,

    pub method: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<IssueTxRequest>,
}

impl Default for DataForIssueTx {
    fn default() -> Self {
        Self::default()
    }
}

impl DataForIssueTx {
    pub fn default() -> Self {
        Self {
            jsonrpc: String::from(DEFAULT_VERSION),
            id: DEFAULT_ID,
            method: String::new(),
            params: None,
        }
    }

    pub fn encode_json(&self) -> io::Result<String> {
        match serde_json::to_string(&self) {
            Ok(s) => Ok(s),
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("failed to serialize to JSON {}", e),
                ));
            }
        }
    }
}

/// e.g., {"jsonrpc":"2.0","error":{"code":-32000,"message":"problem decoding transaction: invalid input checksum","data":null},"id":1}
/// e.g., {"jsonrpc":"2.0","error":{"code":-32000,"message":"problem decoding transaction: missing 0x prefix to hex encoding","data":null},"id":1}
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct ResponseError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

impl Default for ResponseError {
    fn default() -> Self {
        Self::default()
    }
}

impl ResponseError {
    pub fn default() -> Self {
        Self {
            code: 0,
            message: String::new(),
            data: None,
        }
    }
}

/// ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct IssueTx {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<IssueTxResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,
}

impl Default for IssueTx {
    fn default() -> Self {
        Self::default()
    }
}

impl IssueTx {
    pub fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: 1,
            result: None,
            error: None,
        }
    }
}

/// ref. https://docs.avax.network/apis/avalanchego/apis/p-chain#platformissuetx
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

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::test_issue_tx --exact --show-output
#[test]
fn test_issue_tx() {
    use std::str::FromStr;

    // ref. https://docs.avax.network/apis/avalanchego/apis/p-chain#platformissuetx
    let resp: IssueTx = serde_json::from_str(
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

    let expected = IssueTx {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(IssueTxResult {
            tx_id: ids::Id::from_str("G3BuH6ytQ2averrLxJJugjWZHTRubzCrUZEXoheG5JMqL5ccY").unwrap(),
        }),
        error: None,
    };
    assert_eq!(resp, expected);
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IssueStopVertexRequest {}

/// ref. https://docs.avax.network/build/avalanchego-apis/issuing-api-calls
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct DataForIssueStopVertex {
    pub jsonrpc: String,
    pub id: u32,

    pub method: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<IssueStopVertexRequest>,
}

impl Default for DataForIssueStopVertex {
    fn default() -> Self {
        Self::default()
    }
}

impl DataForIssueStopVertex {
    pub fn default() -> Self {
        Self {
            jsonrpc: String::from(DEFAULT_VERSION),
            id: DEFAULT_ID,
            method: String::new(),
            params: None,
        }
    }

    pub fn encode_json(&self) -> io::Result<String> {
        match serde_json::to_string(&self) {
            Ok(s) => Ok(s),
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("failed to serialize to JSON {}", e),
                ));
            }
        }
    }
}

/// ref. https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgetutxos
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetUtxosRequest {
    pub addresses: Vec<String>,
    pub limit: u32,
    pub encoding: String,
}

/// ref. https://docs.avax.network/build/avalanchego-apis/issuing-api-calls
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct DataForGetUtxos {
    pub jsonrpc: String,
    pub id: u32,

    pub method: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<GetUtxosRequest>,
}

impl Default for DataForGetUtxos {
    fn default() -> Self {
        Self::default()
    }
}

impl DataForGetUtxos {
    pub fn default() -> Self {
        Self {
            jsonrpc: String::from(DEFAULT_VERSION),
            id: DEFAULT_ID,
            method: String::new(),
            params: None,
        }
    }

    pub fn encode_json(&self) -> io::Result<String> {
        match serde_json::to_string(&self) {
            Ok(s) => Ok(s),
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("failed to serialize to JSON {}", e),
                ));
            }
        }
    }
}

/// ref. https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgetutxos
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetUtxos {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetUtxosResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,
}

/// ref. https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgetutxos
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetUtxosResult {
    #[serde_as(as = "DisplayFromStr")]
    pub num_fetched: u32,

    #[serde_as(as = "Option<Vec<HexUtxo>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxos: Option<Vec<txs::utxo::Utxo>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_index: Option<EndIndex>,
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

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::test_get_utxos_empty --exact --show-output
#[test]
fn test_get_utxos_empty() {
    // ref. https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgetutxos
    let resp: GetUtxos = serde_json::from_str(
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

    let expected = GetUtxos {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetUtxosResult {
            num_fetched: 0,
            utxos: Some(Vec::new()),
            end_index: Some(EndIndex {
                address: String::from("P-custom152qlr6zunz7nw2kc4lfej3cn3wk46u3002k4w5"),
                utxo: String::from("11111111111111111111111111111111LpoYY"),
            }),
            encoding: Some(String::from("hex")),
        }),
        error: None,
    };
    assert_eq!(resp, expected);
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::test_get_utxos_non_empty --exact --show-output
#[test]
fn test_get_utxos_non_empty() {
    // ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetbalance
    let resp: GetUtxos = serde_json::from_str(
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

    let expected = GetUtxos {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetUtxosResult {
            num_fetched: 1,
            utxos: Some(vec![utxo]),
            end_index: Some(EndIndex {
                address: String::from("X-avax1x459sj0ssujguq723cljfty4jlae28evjzt7xz"),
                utxo: String::from("LUC1cmcxnfNR9LdkACS2ccGKLEK7SYqB4gLLTycQfg1koyfSq"),
            }),
            encoding: Some(String::from("hex")),
        }),
        error: None,
    };
    assert_eq!(resp, expected);
}
