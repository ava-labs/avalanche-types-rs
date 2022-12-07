use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{self, Error, ErrorKind, Write},
    path::Path,
};

use crate::key;
use serde::{Deserialize, Serialize};

/// ref. https://pkg.go.dev/github.com/ava-labs/subnet-evm/core#Genesis
/// ref. https://pkg.go.dev/github.com/ava-labs/subnet-evm/params#ChainConfig
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Genesis {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ChainConfig>,

    #[serde(with = "crate::codec::serde::hex_0x_primitive_types_u256")]
    pub nonce: primitive_types::U256,
    #[serde(with = "crate::codec::serde::hex_0x_primitive_types_u256")]
    pub timestamp: primitive_types::U256,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<String>,

    /// Make sure this is set equal to "ChainConfig.FeeConfig.gas_limit".
    /// ref. https://github.com/ava-labs/subnet-evm/pull/63
    ///
    /// Use https://www.rapidtables.com/convert/number/decimal-to-hex.html to convert.
    #[serde(with = "crate::codec::serde::hex_0x_primitive_types_u256")]
    pub gas_limit: primitive_types::U256,
    #[serde(with = "crate::codec::serde::hex_0x_primitive_types_u256")]
    pub difficulty: primitive_types::U256,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mix_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coinbase: Option<String>,

    /// MUST BE ordered by its key in order for all nodes to have the same JSON outputs.
    /// And expressed as hex strings with the canonical 0x prefix.
    /// ref. https://doc.rust-lang.org/std/collections/index.html#use-a-btreemap-when
    /// ref. https://docs.avax.network/subnets/customize-a-subnet#setting-the-genesis-allocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc: Option<BTreeMap<String, AllocAccount>>,

    /// WARNING: Big airdrop data may cause OOM in subnet-evm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub airdrop_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub airdrop_amount: Option<String>,

    #[serde(with = "crate::codec::serde::hex_0x_primitive_types_u256")]
    pub number: primitive_types::U256,
    #[serde(with = "crate::codec::serde::hex_0x_primitive_types_u256")]
    pub gas_used: primitive_types::U256,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_hash: Option<String>,
    #[serde(rename = "baseFeePerGas", skip_serializing_if = "Option::is_none")]
    pub base_fee: Option<String>,
}

/// On the X-Chain, one AVAX is 10^9  units.
/// On the P-Chain, one AVAX is 10^9  units.
/// On the C-Chain, one AVAX is 10^18 units.
/// "0x204FCE5E3E25026110000000" is "10000000000000000000000000000" (10,000,000,000 AVAX).
/// ref. https://www.rapidtables.com/convert/number/decimal-to-hex.html
/// ref. https://www.rapidtables.com/convert/number/hex-to-decimal.html
pub const DEFAULT_INITIAL_AMOUNT: &str = "0x204FCE5E3E25026110000000";

impl Default for Genesis {
    fn default() -> Self {
        Self::default()
    }
}

impl Genesis {
    pub fn default() -> Self {
        let mut alloc = BTreeMap::new();
        alloc.insert(
            // ref. https://github.com/ava-labs/subnet-evm/blob/master/networks/11111/genesis.json
            String::from("6f0f6DA1852857d7789f68a28bba866671f3880D"),
            AllocAccount::default(),
        );
        Self {
            config: Some(ChainConfig::default()),

            nonce: primitive_types::U256::default(),
            timestamp: primitive_types::U256::default(),
            extra_data: Some(String::from("0x00")),

            // ref. https://www.rapidtables.com/convert/number/decimal-to-hex.html
            // ref. https://www.rapidtables.com/convert/number/hex-to-decimal.html
            gas_limit: primitive_types::U256::from_str_radix("0x1C9C380", 16).unwrap(),

            difficulty: primitive_types::U256::default(),
            mix_hash: Some(String::from(
                "0x0000000000000000000000000000000000000000000000000000000000000000",
            )),
            coinbase: Some(String::from("0x0000000000000000000000000000000000000000")),

            alloc: Some(alloc),

            airdrop_hash: None,
            airdrop_amount: None,

            number: primitive_types::U256::default(),
            gas_used: primitive_types::U256::default(),
            parent_hash: Some(String::from(
                "0x0000000000000000000000000000000000000000000000000000000000000000",
            )),
            base_fee: None,
        }
    }

    /// Creates a new Genesis object with "keys" number of generated pre-funded keys.
    pub fn new<T: key::secp256k1::ReadOnly>(seed_keys: &[T]) -> io::Result<Self> {
        // maximize total supply
        let max_total_alloc = i128::MAX;
        let total_keys = seed_keys.len();
        let alloc_per_key = if total_keys > 0 {
            max_total_alloc / total_keys as i128
        } else {
            0i128
        };
        // divide by 2, allow more room for transfers
        let alloc_per_key = alloc_per_key / 2;

        let mut default_alloc = AllocAccount::default();
        default_alloc.balance = primitive_types::U256::from(alloc_per_key);

        let mut allocs = BTreeMap::new();
        for k in seed_keys.iter() {
            let eth_addr = k.eth_address();
            allocs.insert(
                eth_addr.trim_start_matches("0x").to_string(),
                default_alloc.clone(),
            );
        }

        let mut subnet_evm_genesis = Self::default();
        subnet_evm_genesis.alloc = Some(allocs);

        Ok(subnet_evm_genesis)
    }

    pub fn encode_json(&self) -> io::Result<String> {
        match serde_json::to_string(&self) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("failed to serialize to JSON {}", e),
            )),
        }
    }

    /// Encodes the genesis to bytes.
    pub fn to_bytes(&self) -> io::Result<Vec<u8>> {
        serde_json::to_vec(self).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed encode genesis to JSON {}", e),
            )
        })
    }

    /// Saves the current anchor node to disk
    /// and overwrites the file.
    pub fn sync(&self, file_path: &str) -> io::Result<()> {
        log::info!("syncing Genesis to '{}'", file_path);
        let path = Path::new(file_path);
        let parent_dir = path.parent().expect("unexpected None parent");
        fs::create_dir_all(parent_dir)?;

        let ret = serde_json::to_vec(self);
        let d = match ret {
            Ok(d) => d,
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("failed to serialize Genesis to YAML {}", e),
                ));
            }
        };
        let mut f = File::create(file_path)?;
        f.write_all(&d)?;

        Ok(())
    }
}

/// ref. https://pkg.go.dev/github.com/ava-labs/subnet-evm/params#ChainConfig
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChainConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub homestead_block: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip150_block: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip150_hash: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip155_block: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip158_block: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub byzantium_block: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constantinople_block: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub petersburg_block: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub istanbul_block: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muir_glacier_block: Option<u64>,

    #[serde(rename = "subnetEVMTimestamp", skip_serializing_if = "Option::is_none")]
    pub subnet_evm_timestamp: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_config: Option<FeeConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_fee_recipients: Option<bool>,

    /// ref. https://docs.avax.network/subnets/customize-a-subnet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_deployer_allow_list_config: Option<ContractDeployerAllowListConfig>,
    /// ref. https://docs.avax.network/subnets/customize-a-subnet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_native_minter_config: Option<ContractNativeMinterConfig>,
    /// ref. https://docs.avax.network/subnets/customize-a-subnet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_allow_list_config: Option<TxAllowListConfig>,
    /// ref. https://docs.avax.network/subnets/customize-a-subnet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_manager_config: Option<FeeManagerConfig>,
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self::default()
    }
}

impl ChainConfig {
    pub fn default() -> Self {
        Self {
            // don't use local ID "43112" to avoid config override
            // ref. https://github.com/ava-labs/coreth/blob/v0.8.6/plugin/evm/vm.go#L326-L328
            // ref. https://github.com/ava-labs/avalanche-ops/issues/8
            chain_id: Some(2000777),
            homestead_block: Some(0),

            eip150_block: Some(0),
            eip150_hash: Some(String::from(
                "0x2086799aeebeae135c246c65021c82b4e15a2c451340993aacfd2751886514f0",
            )),

            eip155_block: Some(0),
            eip158_block: Some(0),

            byzantium_block: Some(0),
            constantinople_block: Some(0),
            petersburg_block: Some(0),
            istanbul_block: Some(0),
            muir_glacier_block: Some(0),

            subnet_evm_timestamp: Some(0),

            fee_config: Some(FeeConfig::default()),
            allow_fee_recipients: None,

            contract_deployer_allow_list_config: None,
            contract_native_minter_config: None,
            tx_allow_list_config: None,
            fee_manager_config: None,
        }
    }
}

/// ref. https://pkg.go.dev/github.com/ava-labs/subnet-evm/commontype#FeeConfig
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeConfig {
    /// Make sure this is set equal to "Genesis.gas_limit".
    /// ref. https://github.com/ava-labs/subnet-evm/pull/63
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_block_rate: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_base_fee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_gas: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_fee_change_denominator: Option<u64>,

    /// USE WITH CAUTION!
    /// Set min/max block gas cost the same and low to
    /// keep the gas price constant (useful for load tests).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_block_gas_cost: Option<u64>,
    /// USE WITH CAUTION!
    /// Set min/max block gas cost the same and low to
    /// keep the gas price constant (useful for load tests).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_block_gas_cost: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_gas_cost_step: Option<u64>,
}

impl Default for FeeConfig {
    fn default() -> Self {
        Self::default()
    }
}

/// C-chain is 8-million
/// ref. https://www.rapidtables.com/convert/number/decimal-to-hex.html
/// ref. https://www.rapidtables.com/convert/number/hex-to-decimal.html
/// ref. https://github.com/ava-labs/public-chain-assets/blob/main/chains/53935/genesis.json
pub const DEFAULT_GAS_LIMIT: u64 = 30000000;

pub const DEFAULT_TARGET_BLOCK_RATE: u64 = 2;

impl FeeConfig {
    pub fn default() -> Self {
        Self {
            gas_limit: Some(DEFAULT_GAS_LIMIT),
            target_block_rate: Some(DEFAULT_TARGET_BLOCK_RATE),

            min_base_fee: Some(40000000),
            target_gas: Some(75000000),
            base_fee_change_denominator: Some(48),

            min_block_gas_cost: Some(0),
            max_block_gas_cost: Some(2000000000000000000),
            block_gas_cost_step: Some(1000000000000000000),
        }
    }
}

/// ref. https://github.com/ava-labs/subnet-evm/blob/master/precompile/contract_deployer_allow_list.go
/// ref. https://github.com/ava-labs/subnet-evm/blob/master/precompile/upgradeable.go
/// ref. https://github.com/ava-labs/subnet-evm/blob/master/params/precompile_config.go
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractDeployerAllowListConfig {
    #[serde(rename = "adminAddresses", skip_serializing_if = "Option::is_none")]
    pub allow_list_admins: Option<Vec<String>>,

    /// Timestamp for the upgrade.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_timestamp: Option<u64>,
    /// Set to "true" for the upgrade to deactivate the precompile and reset its storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
}

impl Default for ContractDeployerAllowListConfig {
    fn default() -> Self {
        Self::default()
    }
}

impl ContractDeployerAllowListConfig {
    pub fn default() -> Self {
        Self {
            allow_list_admins: None,
            block_timestamp: Some(0),
            disable: None,
        }
    }
}

/// ref. https://github.com/ava-labs/subnet-evm/blob/master/precompile/contract_native_minter.go
/// ref. https://github.com/ava-labs/subnet-evm/blob/master/precompile/upgradeable.go
/// ref. https://github.com/ava-labs/subnet-evm/blob/master/params/precompile_config.go
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractNativeMinterConfig {
    #[serde(rename = "adminAddresses", skip_serializing_if = "Option::is_none")]
    pub allow_list_admins: Option<Vec<String>>,

    /// Timestamp for the upgrade.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_timestamp: Option<u64>,

    /// Set to "true" for the upgrade to deactivate the precompile and reset its storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
}

impl Default for ContractNativeMinterConfig {
    fn default() -> Self {
        Self::default()
    }
}

impl ContractNativeMinterConfig {
    pub fn default() -> Self {
        Self {
            allow_list_admins: None,
            block_timestamp: Some(0),
            disable: None,
        }
    }
}

/// ref. https://github.com/ava-labs/subnet-evm/blob/master/precompile/tx_allow_list.go
/// ref. https://github.com/ava-labs/subnet-evm/blob/master/precompile/upgradeable.go
/// ref. https://github.com/ava-labs/subnet-evm/blob/master/params/precompile_config.go
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TxAllowListConfig {
    #[serde(rename = "adminAddresses", skip_serializing_if = "Option::is_none")]
    pub allow_list_admins: Option<Vec<String>>,

    /// Timestamp for the upgrade.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_timestamp: Option<u64>,
    /// Set to "true" for the upgrade to deactivate the precompile and reset its storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
}

impl Default for TxAllowListConfig {
    fn default() -> Self {
        Self::default()
    }
}

impl TxAllowListConfig {
    pub fn default() -> Self {
        Self {
            allow_list_admins: None,
            block_timestamp: Some(0),
            disable: None,
        }
    }
}

/// ref. https://github.com/ava-labs/subnet-evm/blob/master/precompile/fee_config_manager.go
/// ref. https://github.com/ava-labs/subnet-evm/blob/master/precompile/upgradeable.go
/// ref. https://github.com/ava-labs/subnet-evm/blob/master/params/precompile_config.go
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeManagerConfig {
    #[serde(rename = "adminAddresses", skip_serializing_if = "Option::is_none")]
    pub allow_list_admins: Option<Vec<String>>,

    /// Timestamp for the upgrade.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_timestamp: Option<u64>,
    /// Set to "true" for the upgrade to deactivate the precompile and reset its storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
}

impl Default for FeeManagerConfig {
    fn default() -> Self {
        Self::default()
    }
}

impl FeeManagerConfig {
    pub fn default() -> Self {
        Self {
            allow_list_admins: None,
            block_timestamp: Some(0),
            disable: None,
        }
    }
}

/// ref. https://pkg.go.dev/github.com/ava-labs/subnet-evm/core#GenesisAlloc
/// ref. https://pkg.go.dev/github.com/ava-labs/subnet-evm/core#GenesisAccount
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AllocAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<BTreeMap<String, String>>,

    #[serde(with = "crate::codec::serde::hex_0x_primitive_types_u256")]
    pub balance: primitive_types::U256,

    /// ref. https://pkg.go.dev/github.com/ava-labs/subnet-evm/core#GenesisMultiCoinBalance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcbalance: Option<BTreeMap<String, u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<u64>,
}

impl Default for AllocAccount {
    fn default() -> Self {
        Self::default()
    }
}

impl AllocAccount {
    pub fn default() -> Self {
        Self {
            code: None,
            storage: None,
            balance: primitive_types::U256::from_str_radix(DEFAULT_INITIAL_AMOUNT, 16).unwrap(),
            mcbalance: None,
            nonce: None,
        }
    }
}

#[test]
fn test_parse() {
    let _ = env_logger::builder().is_test(true).try_init();

    // ref. https://github.com/ava-labs/subnet-evm/blob/master/networks/11111/genesis.json
    let resp: Genesis = serde_json::from_str(
        r#"
{
        "config": {
            "chainId": 2000777,
            "homesteadBlock": 0,
            "eip150Block": 0,
            "eip150Hash": "0x2086799aeebeae135c246c65021c82b4e15a2c451340993aacfd2751886514f0",
            "eip155Block": 0,
            "eip158Block": 0,
            "byzantiumBlock": 0,
            "constantinopleBlock": 0,
            "petersburgBlock": 0,
            "istanbulBlock": 0,
            "muirGlacierBlock": 0,
            "subnetEVMTimestamp": 0,
            "feeConfig": {
                "gasLimit": 30000000,
                "minBaseFee": 40000000,
                "targetGas": 75000000,
                "baseFeeChangeDenominator": 48,
                "minBlockGasCost": 0,
                "maxBlockGasCost": 2000000000000000000,
                "targetBlockRate": 2,
                "blockGasCostStep": 1000000000000000000
            }
        },
        "alloc": {
            "6f0f6DA1852857d7789f68a28bba866671f3880D": {
                "balance": "0x204FCE5E3E25026110000000"
            }
        },
        "nonce": "0x0",
        "timestamp": "0x0",
        "extraData": "0x00",
        "gasLimit": "0x1C9C380",
        "difficulty": "0x0",
        "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "coinbase": "0x0000000000000000000000000000000000000000",
        "number": "0x0",
        "gasUsed": "0x0",
        "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000"
}
"#,
    )
    .unwrap();

    let expected = Genesis::default();
    assert_eq!(resp, expected);

    let d = Genesis::default();
    let d = d.encode_json().unwrap();
    log::info!("{}", d);
}
