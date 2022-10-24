use crate::formatting::serde::hex_0x_bytes::HexBytes;
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Response for "eth_blockNumber".
/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_blocknumber
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct BlockNumber {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(with = "crate::formatting::serde::hex_0x_big_int")]
    pub result: BigInt,
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::eth::test_block_number --exact --show-output
#[test]
fn test_block_number() {
    let resp: BlockNumber = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": \"0x4b7\",
    \"id\": 83
}

",
    )
    .unwrap();
    let expected = BlockNumber {
        jsonrpc: "2.0".to_string(),
        id: 83,
        result: big_num_manager::from_hex_to_big_int("0x4b7").unwrap(),
    };
    assert_eq!(resp, expected);
}

/// Response for "eth_gasPrice".
/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_gasprice
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GasPrice {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(with = "crate::formatting::serde::hex_0x_big_int")]
    pub result: BigInt,
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::eth::test_gas_price --exact --show-output
#[test]
fn test_gas_price() {
    let resp: GasPrice = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": \"0x1dfd14000\",
    \"id\": 1
}

",
    )
    .unwrap();
    let expected = GasPrice {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: big_num_manager::from_hex_to_big_int("0x1dfd14000").unwrap(),
    };
    assert_eq!(resp, expected);
}

/// Response for "eth_getBalance".
/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_getbalance
/// ref. https://docs.avax.network/build/avalanchego-apis/c-chain#eth_getassetbalance
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetBalance {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(with = "crate::formatting::serde::hex_0x_big_int")]
    pub result: BigInt,
}

/// Response for "eth_getBalance".
/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::eth::test_get_balance --exact --show-output
#[test]
fn test_get_balance() {
    // ref. https://docs.avax.network/build/avalanchego-apis/c-chain#eth_getassetbalance
    let resp: GetBalance = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": \"0x1388\",
    \"id\": 1
}

",
    )
    .unwrap();
    let expected = GetBalance {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: big_num_manager::from_hex_to_big_int("0x1388").unwrap(),
    };
    assert_eq!(resp, expected);

    let resp: GetBalance = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": \"0x0234c8a3397aab58\",
    \"id\": 1
}

",
    )
    .unwrap();
    let expected = GetBalance {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: big_num_manager::from_hex_to_big_int("0x0234c8a3397aab58").unwrap(),
    };
    assert_eq!(resp, expected);
}

/// Response for "eth_getTransactionCount".
/// Returns the number of transactions send from this address.
/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_gettransactioncount
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetTransactionCount {
    pub jsonrpc: String,
    pub id: u32,

    /// The number of transactions send from this address.
    #[serde(with = "crate::formatting::serde::hex_0x_big_int")]
    pub result: BigInt,
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::eth::test_get_transaction_count --exact --show-output
#[test]
fn test_get_transaction_count() {
    let resp: GetTransactionCount = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": \"0x1\",
    \"id\": 1
}

",
    )
    .unwrap();
    let expected = GetTransactionCount {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: big_num_manager::from_hex_to_big_int("0x1").unwrap(),
    };
    assert_eq!(resp, expected);
}

/// Response for "eth_getTransactionReceipt".
/// Returns the receipt of a transaction by transaction hash.
/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_gettransactionreceipt
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct GetTransactionReceipt {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GetTransactionReceiptResult>,
}

/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_gettransactionreceipt
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionReceiptResult {
    pub from: String,
    pub to: String,

    #[serde(with = "crate::formatting::serde::hex_0x_big_int")]
    pub block_number: BigInt,
    #[serde_as(as = "HexBytes")]
    pub block_hash: Vec<u8>,

    /// Null, if none was created.
    #[serde_as(as = "Option<HexBytes>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<Vec<u8>>,

    #[serde(with = "crate::formatting::serde::hex_0x_big_int")]
    pub cumulative_gas_used: BigInt,
    #[serde(with = "crate::formatting::serde::hex_0x_big_int")]
    pub gas_used: BigInt,

    #[serde(with = "crate::formatting::serde::hex_0x_big_int")]
    pub transaction_index: BigInt,
    #[serde_as(as = "HexBytes")]
    pub transaction_hash: Vec<u8>,

    #[serde(with = "crate::formatting::serde::hex_0x_big_int")]
    pub status: BigInt,
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- jsonrpc::eth::test_get_transaction_receipt --exact --show-output
#[test]
fn test_get_transaction_receipt() {
    let resp: GetTransactionReceipt = serde_json::from_str(
        "

{
    \"jsonrpc\": \"2.0\",
    \"result\": {
        \"from\": \"0x7eb4c9d6b763324eea4852f5d40985bbf0f29832\",
        \"to\": \"0x3c42649799074b438889b80312ea9f62bc798aa8\",
        \"blockHash\": \"0xc6ef2fc5426d6ad6fd9e2a26abeab0aa2411b7ab17f30a99d3cb96aed1d1055b\",
        \"blockNumber\": \"0xb\",
        \"cumulativeGasUsed\": \"0x33bc\",
        \"gasUsed\": \"0x4dc\",
        \"transactionIndex\": \"0x1\",
        \"transactionHash\": \"0xb903239f8543d04b5dc1ba6579132b143087c68db1b2168786408fcbce568238\",
        \"status\": \"0x1\"
    },
    \"id\": 1
}

",
    )
    .unwrap();
    // println!("{:?}", resp);

    let expected = GetTransactionReceipt {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(GetTransactionReceiptResult {
            from: String::from("0x7eb4c9d6b763324eea4852f5d40985bbf0f29832"),
            to: String::from("0x3c42649799074b438889b80312ea9f62bc798aa8"),

            block_number: big_num_manager::from_hex_to_big_int("0xb").unwrap(),
            block_hash: vec![
                198, 239, 47, 197, 66, 109, 106, 214, 253, 158, 42, 38, 171, 234, 176, 170, 36, 17,
                183, 171, 23, 243, 10, 153, 211, 203, 150, 174, 209, 209, 5, 91,
            ],

            contract_address: None,

            cumulative_gas_used: big_num_manager::from_hex_to_big_int("0x33bc").unwrap(),
            gas_used: big_num_manager::from_hex_to_big_int("0x4dc").unwrap(),

            transaction_index: big_num_manager::from_hex_to_big_int("0x1").unwrap(),
            transaction_hash: vec![
                185, 3, 35, 159, 133, 67, 208, 75, 93, 193, 186, 101, 121, 19, 43, 20, 48, 135,
                198, 141, 177, 178, 22, 135, 134, 64, 143, 203, 206, 86, 130, 56,
            ],

            status: big_num_manager::from_hex_to_big_int("0x1").unwrap(),
        }),
    };
    assert_eq!(resp, expected);
}
