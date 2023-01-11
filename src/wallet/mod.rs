pub mod p;
pub mod x;

#[cfg(feature = "wallet_evm")]
pub mod evm;

use std::{
    fmt, io,
    sync::{Arc, Mutex},
};

use crate::{
    ids::{self, short},
    jsonrpc::client::{info as api_info, x as api_x},
    key, units,
};
use url::Url;

#[derive(Debug, Clone)]
pub struct Wallet<T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone> {
    pub key_type: key::secp256k1::KeyType,
    pub keychain: key::secp256k1::keychain::Keychain<T>,

    /// Base HTTP URLs without RPC endpoint path.
    pub base_http_urls: Vec<String>,
    pub base_http_url_cursor: Arc<Mutex<usize>>, // to roundrobin

    pub network_id: u32,
    pub network_name: String,

    pub x_address: String,
    pub p_address: String,
    pub short_address: short::Id,
    pub eth_address: String,
    pub h160_address: primitive_types::H160,

    pub blockchain_id_x: ids::Id,
    pub blockchain_id_p: ids::Id,

    pub avax_asset_id: ids::Id,

    /// Fee that is burned by every non-state creating transaction.
    pub tx_fee: u64,
    /// Transaction fee for adding a primary network validator.
    pub add_primary_network_validator_fee: u64,
    /// Transaction fee to create a new subnet.
    pub create_subnet_tx_fee: u64,
    /// Transaction fee to create a new blockchain.
    pub create_blockchain_tx_fee: u64,
}

/// ref. <https://doc.rust-lang.org/std/string/trait.ToString.html>
/// ref. <https://doc.rust-lang.org/std/fmt/trait.Display.html>
/// Use "Self.to_string()" to directly invoke this
impl<T> fmt::Display for Wallet<T>
where
    T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "key_type: {}\n", self.key_type.as_str())?;
        write!(f, "http_rpcs: {:?}\n", self.base_http_urls)?;
        write!(f, "network_id: {}\n", self.network_id)?;
        write!(f, "network_name: {}\n", self.network_name)?;

        write!(f, "x_address: {}\n", self.x_address)?;
        write!(f, "p_address: {}\n", self.p_address)?;
        write!(f, "short_address: {}\n", self.short_address)?;
        write!(f, "eth_address: {}\n", self.eth_address)?;
        write!(f, "h160_address: {}\n", self.h160_address)?;

        write!(f, "blockchain_id_x: {}\n", self.blockchain_id_x)?;
        write!(f, "blockchain_id_p: {}\n", self.blockchain_id_p)?;

        write!(f, "avax_asset_id: {}\n", self.avax_asset_id)?;

        write!(f, "tx_fee: {}\n", self.tx_fee)?;
        write!(
            f,
            "add_primary_network_validator_fee: {}\n",
            self.add_primary_network_validator_fee
        )?;
        write!(f, "create_subnet_tx_fee: {}\n", self.create_subnet_tx_fee)?;
        write!(
            f,
            "create_blockchain_tx_fee: {}\n",
            self.create_blockchain_tx_fee
        )
    }
}

impl<T> Wallet<T>
where
    T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone,
{
    /// Picks one endpoint in roundrobin, and updates the cursor for next calls.
    /// Returns the pair of an index and its corresponding endpoint.
    pub fn pick_base_http_url(&self) -> (usize, String) {
        let mut idx = self.base_http_url_cursor.lock().unwrap();

        let picked = *idx;
        let http_rpc = self.base_http_urls[picked].clone();
        *idx = (picked + 1) % self.base_http_urls.len();

        log::debug!("picked base http URL {http_rpc} at index {picked}");
        (picked, http_rpc)
    }
}

#[derive(Debug, Clone)]
pub struct Builder<T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone> {
    pub key: T,
    pub base_http_urls: Vec<String>,
}

impl<T> Builder<T>
where
    T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone,
{
    pub fn new(key: &T) -> Self {
        Self {
            key: key.clone(),
            base_http_urls: Vec::new(),
        }
    }

    /// Adds an HTTP rpc endpoint to the `http_rpcs` field in the Builder.
    /// If URL path is specified, it strips the URL path.
    #[must_use]
    pub fn base_http_url(mut self, u: String) -> Self {
        let url = Url::parse(&u).unwrap();
        let rpc_ep = format!("{}://{}", url.scheme(), url.host_str().unwrap());
        let rpc_url = if let Some(port) = url.port() {
            format!("{rpc_ep}:{port}")
        } else {
            rpc_ep // e.g., DNS
        };

        if self.base_http_urls.is_empty() {
            self.base_http_urls = vec![rpc_url];
        } else {
            self.base_http_urls.push(rpc_url);
        }
        self
    }

    /// Overwrites the HTTP rpc endpoints to the `urls` field in the Builder.
    /// If URL path is specified, it strips the URL path.
    #[must_use]
    pub fn base_http_urls(mut self, urls: Vec<String>) -> Self {
        let mut base_http_urls = Vec::new();
        for http_rpc in urls.iter() {
            let url = Url::parse(http_rpc).unwrap();
            let rpc_ep = format!("{}://{}", url.scheme(), url.host_str().unwrap());
            let rpc_url = if let Some(port) = url.port() {
                format!("{rpc_ep}:{port}")
            } else {
                rpc_ep // e.g., DNS
            };
            base_http_urls.push(rpc_url);
        }
        self.base_http_urls = base_http_urls;
        self
    }

    pub async fn build(&self) -> io::Result<Wallet<T>> {
        log::info!(
            "building wallet with {} endpoints",
            self.base_http_urls.len()
        );

        let keychain = key::secp256k1::keychain::Keychain::new(vec![self.key.clone()]);
        let h160_address = keychain.keys[0].h160_address();

        let resp = api_info::get_network_id(&self.base_http_urls[0]).await?;
        let network_id = resp.result.unwrap().network_id;
        let resp = api_info::get_network_name(&self.base_http_urls[0]).await?;
        let network_name = resp.result.unwrap().network_name;

        let resp = api_info::get_blockchain_id(&self.base_http_urls[0], "X").await?;
        let blockchain_id_x = resp.result.unwrap().blockchain_id;

        let resp = api_info::get_blockchain_id(&self.base_http_urls[0], "P").await?;
        let blockchain_id_p = resp.result.unwrap().blockchain_id;

        let resp = api_x::get_asset_description(&self.base_http_urls[0], "AVAX").await?;
        let resp = resp
            .result
            .expect("unexpected None GetAssetDescriptionResult");
        let avax_asset_id = resp.asset_id;

        let resp = api_info::get_tx_fee(&self.base_http_urls[0]).await?;
        let tx_fee = resp.result.unwrap().tx_fee;

        let (create_subnet_tx_fee, create_blockchain_tx_fee) = if network_id == 1 {
            // ref. "genesi/genesis_mainnet.go"
            (1 * units::AVAX, 1 * units::AVAX)
        } else {
            // ref. "genesi/genesis_fuji.go"
            // ref. "genesi/genesis_local.go"
            (100 * units::MILLI_AVAX, 100 * units::MILLI_AVAX)
        };

        let w = Wallet {
            key_type: self.key.key_type(),
            keychain,

            base_http_urls: self.base_http_urls.clone(),
            base_http_url_cursor: Arc::new(Mutex::new(0)),

            network_id,
            network_name,

            x_address: self.key.hrp_address(network_id, "X").unwrap(),
            p_address: self.key.hrp_address(network_id, "P").unwrap(),
            short_address: self.key.short_address().unwrap(),
            eth_address: self.key.eth_address(),
            h160_address,

            blockchain_id_x,
            blockchain_id_p,

            avax_asset_id,

            tx_fee,
            add_primary_network_validator_fee: ADD_PRIMARY_NETWORK_VALIDATOR_FEE,
            create_subnet_tx_fee,
            create_blockchain_tx_fee,
        };
        log::info!("initiated the wallet:\n{}", w);

        Ok(w)
    }
}

/// ref. <https://docs.avax.network/learn/platform-overview/transaction-fees/#fee-schedule>
pub const ADD_PRIMARY_NETWORK_VALIDATOR_FEE: u64 = 0;
