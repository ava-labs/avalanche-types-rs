pub mod avm;
pub mod eth;
pub mod health;
pub mod info;
pub mod platformvm;

use crate::{ids, txs};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{self, Error, ErrorKind},
    str::FromStr,
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

/// ref. https://docs.avax.network/build/avalanchego-apis/x-chain#avmgetbalance
/// ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetbalance
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct RawUtxoId {
    #[serde(rename = "txID")]
    pub tx_id: String,
    #[serde(rename = "outputIndex")]
    pub output_index: u32,
}

impl RawUtxoId {
    pub fn convert(&self) -> io::Result<crate::txs::utxo::Id> {
        let tx_id = crate::ids::Id::from_str(&self.tx_id)?;
        Ok(crate::txs::utxo::Id {
            tx_id,
            output_index: self.output_index,
            ..crate::txs::utxo::Id::default()
        })
    }
}

/// ref. https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgetutxos
/// ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetutxos
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub struct EndIndex {
    pub address: String,
    pub utxo: String,
}

/// ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
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
pub struct IssueTxResponse {
    pub jsonrpc: String,
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<IssueTxResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,
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

/// ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct IssueTxResult {
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

/// ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct RawIssueTxResponse {
    pub jsonrpc: String,
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<RawIssueTxResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,
}

impl Default for RawIssueTxResponse {
    fn default() -> Self {
        Self::default()
    }
}

impl RawIssueTxResponse {
    pub fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: 1,
            result: None,
            error: None,
        }
    }

    pub fn convert(&self) -> IssueTxResponse {
        let result = {
            if self.result.is_some() {
                let raw_result = self.result.clone().expect("unexpected None result");
                Some(raw_result.convert())
            } else {
                None
            }
        };
        IssueTxResponse {
            jsonrpc: self.jsonrpc.clone(),
            id: self.id,
            result,
            error: self.error.clone(),
        }
    }
}

/// ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct RawIssueTxResult {
    #[serde(rename = "txID", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
}

impl RawIssueTxResult {
    pub fn default() -> Self {
        Self { tx_id: None }
    }

    pub fn convert(&self) -> IssueTxResult {
        let tx_id = {
            if self.tx_id.is_some() {
                ids::Id::from_str(&self.tx_id.clone().unwrap()).unwrap()
            } else {
                ids::Id::empty()
            }
        };
        IssueTxResult { tx_id }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- api::platformvm::test_issue_tx --exact --show-output
#[test]
fn test_issue_tx() {
    // ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetcurrentvalidators
    let resp: RawIssueTxResponse = serde_json::from_str(
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
    let converted = resp.convert();

    let expected = IssueTxResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(IssueTxResult {
            tx_id: ids::Id::from_str("G3BuH6ytQ2averrLxJJugjWZHTRubzCrUZEXoheG5JMqL5ccY").unwrap(),
        }),
        error: None,
    };
    assert_eq!(converted, expected);
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
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
#[serde(rename_all = "snake_case")]
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
pub struct GetUtxosResponse {
    pub jsonrpc: String,
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetUtxosResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,
}

/// ref. https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgetutxos
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetUtxosResult {
    #[serde(rename = "numFetched", skip_serializing_if = "Option::is_none")]
    pub num_fetched: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxos: Option<Vec<txs::utxo::Utxo>>,
    #[serde(rename = "endIndex", skip_serializing_if = "Option::is_none")]
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
            num_fetched: None,
            utxos: None,
            end_index: None,
            encoding: None,
        }
    }
}

/// ref. https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgetutxos
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct RawGetUtxosResponse {
    pub jsonrpc: String,
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<RawGetUtxosResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,
}

/// ref. https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgetutxos
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct RawGetUtxosResult {
    #[serde(rename = "numFetched", skip_serializing_if = "Option::is_none")]
    pub num_fetched: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxos: Option<Vec<String>>,
    #[serde(rename = "endIndex", skip_serializing_if = "Option::is_none")]
    pub end_index: Option<EndIndex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
}

impl RawGetUtxosResponse {
    pub fn convert(&self) -> io::Result<GetUtxosResponse> {
        let result = {
            if self.result.is_some() {
                let mut result = GetUtxosResult::default();
                if self
                    .result
                    .clone()
                    .expect("unexpected None result")
                    .num_fetched
                    .is_some()
                {
                    let num_fetched = self
                        .result
                        .clone()
                        .expect("unexpected None result")
                        .num_fetched
                        .expect("unexpected None num_fetched");
                    let num_fetched = num_fetched.parse::<u32>().unwrap();
                    result.num_fetched = Some(num_fetched);
                }

                if self
                    .result
                    .clone()
                    .expect("unexpected None result")
                    .utxos
                    .is_some()
                {
                    let utxos_raw = self
                        .result
                        .clone()
                        .expect("unexpected None result")
                        .utxos
                        .expect("unexpected None utxos");

                    let mut utxos: Vec<txs::utxo::Utxo> = Vec::new();
                    for s in utxos_raw.iter() {
                        let utxo = txs::utxo::Utxo::from_hex(s)
                            .expect("failed to unpack raw utxo from hex string");
                        if utxo.transfer_output.is_none() && utxo.stakeable_lock_out.is_none() {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                "both Utxo.transfer_output and stakeable_lock_out None",
                            ));
                        }
                        utxos.push(utxo);
                    }
                    result.utxos = Some(utxos);
                }

                if self
                    .result
                    .clone()
                    .expect("unexpected None result")
                    .end_index
                    .is_some()
                {
                    let end_index = self
                        .result
                        .clone()
                        .expect("unexpected None result")
                        .end_index
                        .expect("unexpected None end_index");
                    result.end_index = Some(end_index);
                }

                if self
                    .result
                    .clone()
                    .expect("unexpected None result")
                    .encoding
                    .is_some()
                {
                    let encoding = self
                        .result
                        .clone()
                        .expect("unexpected None result")
                        .encoding
                        .expect("unexpected None encoding");
                    result.encoding = Some(encoding);
                }
                Some(result)
            } else {
                None
            }
        };
        Ok(GetUtxosResponse {
            jsonrpc: self.jsonrpc.clone(),
            id: self.id,
            result,
            error: self.error.clone(),
        })
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- api::avm::test_convert_get_utxos_empty --exact --show-output
#[test]
fn test_convert_get_utxos_empty() {
    // ref. https://docs.avax.network/apis/avalanchego/apis/x-chain/#avmgetutxos
    let resp: RawGetUtxosResponse = serde_json::from_str(
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
    let parsed = resp.convert().unwrap();
    let expected = GetUtxosResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetUtxosResult {
            num_fetched: Some(0),
            utxos: Some(Vec::new()),
            end_index: Some(EndIndex {
                address: String::from("P-custom152qlr6zunz7nw2kc4lfej3cn3wk46u3002k4w5"),
                utxo: String::from("11111111111111111111111111111111LpoYY"),
            }),
            encoding: Some(String::from("hex")),
        }),
        error: None,
    };
    assert_eq!(parsed, expected);
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- api::avm::test_convert_get_utxos_non_empty --exact --show-output
#[test]
fn test_convert_get_utxos_non_empty() {
    // ref. https://docs.avax.network/build/avalanchego-apis/p-chain/#platformgetbalance
    let resp: RawGetUtxosResponse = serde_json::from_str(
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

    let parsed = resp.convert().unwrap();
    let expected = GetUtxosResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetUtxosResult {
            num_fetched: Some(1),
            utxos: Some(vec![utxo]),
            end_index: Some(EndIndex {
                address: String::from("X-avax1x459sj0ssujguq723cljfty4jlae28evjzt7xz"),
                utxo: String::from("LUC1cmcxnfNR9LdkACS2ccGKLEK7SYqB4gLLTycQfg1koyfSq"),
            }),
            encoding: Some(String::from("hex")),
        }),
        error: None,
    };
    assert_eq!(parsed, expected);
}
