pub mod eip1559;

use std::{
    io::{self, Error, ErrorKind},
    ops::Div,
    sync::Arc,
    time::Duration,
};

use crate::{jsonrpc::client::evm as jsonrpc_client_evm, key, wallet};
use ethers::{
    prelude::{
        gas_escalator::{Frequency, GasEscalatorMiddleware, GeometricGasPrice},
        NonceManagerMiddleware, SignerMiddleware,
    },
    utils::Units::Gwei,
};
use ethers_providers::{Http, HttpRateLimitRetryPolicy, Provider, RetryClient};
use lazy_static::lazy_static;
use primitive_types::U256;
use reqwest::ClientBuilder;
use url::Url;

/// Make sure to not create multiple providers for the ease of nonce management.
/// ref. "Provider::<RetryClient<Http>>::new_client".
pub fn new_provider(
    chain_rpc_url: &str,
    connect_timeout: Duration,
    request_timeout: Duration,
    max_retries: u32,
    backoff_timeout: Duration,
) -> io::Result<Provider<RetryClient<Http>>> {
    let u = Url::parse(chain_rpc_url).map_err(|e| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("failed to parse chain RPC URL {}", e),
        )
    })?;

    let http_cli = ClientBuilder::new()
        .user_agent(env!("CARGO_PKG_NAME"))
        .connect_timeout(connect_timeout)
        .connection_verbose(true)
        .timeout(request_timeout)
        .danger_accept_invalid_certs(true) // make this configurable
        .build()
        .map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed ClientBuilder build {}", e),
            )
        })?;

    // TODO: make "HttpRateLimitRetryPolicy" configurable
    let retry_client = RetryClient::new(
        Http::new_with_client(u, http_cli),
        Box::new(HttpRateLimitRetryPolicy),
        max_retries,
        backoff_timeout.as_millis() as u64,
    );

    let provider = Provider::new(retry_client).interval(Duration::from_millis(2000u64));
    Ok(provider)
}

/// Make sure to not create multiple providers for the ease of nonce management.
pub fn new_middleware<'a, S>(
    provider: Arc<Provider<RetryClient<Http>>>,
    eth_signer: &'a S,
    chain_id: U256,
) -> io::Result<
    NonceManagerMiddleware<
        SignerMiddleware<
            GasEscalatorMiddleware<Arc<Provider<RetryClient<Http>>>, GeometricGasPrice>,
            S,
        >,
    >,
>
where
    S: ethers_signers::Signer + Clone,
    S::Error: 'static,
{
    // TODO: make this configurable
    let escalator = GeometricGasPrice::new(5.0, 10u64, None::<u64>);

    let gas_escalator_middleware =
        GasEscalatorMiddleware::new(Arc::clone(&provider), escalator, Frequency::PerBlock);

    let signer_middleware = SignerMiddleware::new(
        gas_escalator_middleware,
        eth_signer.clone().with_chain_id(chain_id.as_u64()),
    );

    let nonce_middleware = NonceManagerMiddleware::new(signer_middleware, eth_signer.address());
    Ok(nonce_middleware)
}

lazy_static! {
    pub static ref GWEI: U256 = U256::from(10).checked_pow(Gwei.as_num().into()).unwrap();
}

impl<T> wallet::Wallet<T>
where
    T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone,
{
    /// Sets the chain RPC URLs (can be different than base HTTP URLs).
    /// e.g., "{base_http_url}/ext/bc/{chain_id_alias}/rpc"
    /// Set "chain_id_alias" to either "C" or subnet-evm chain Id.
    #[must_use]
    pub fn evm<'a, S>(
        &self,
        eth_signer: &'a S,
        chain_rpc_url: &str,
        chain_id: U256,
    ) -> io::Result<Evm<'a, T, S>>
    where
        S: ethers_signers::Signer + Clone,
        S::Error: 'static,
    {
        // TODO: make timeouts + retries configurable
        let provider = new_provider(
            chain_rpc_url,
            Duration::from_secs(20),
            Duration::from_secs(70),
            10,
            Duration::from_secs(3),
        )?;
        let provider_arc = Arc::new(provider);

        let nonce_middleware = new_middleware(Arc::clone(&provider_arc), eth_signer, chain_id)?;
        let middleware = Arc::new(nonce_middleware);

        Ok(Evm::<'a, T, S> {
            inner: self.clone(),
            eth_signer,

            chain_rpc_url: chain_rpc_url.to_string(),
            provider: provider_arc,
            middleware,

            chain_id,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Evm<'a, T, S>
where
    T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone,
    S: ethers_signers::Signer + Clone,
    S::Error: 'static,
{
    pub inner: wallet::Wallet<T>,
    pub eth_signer: &'a S,

    pub chain_rpc_url: String,
    pub provider: Arc<Provider<RetryClient<Http>>>,

    /// Middleware created on the picked RPC endpoint and signer address.
    /// ref. "ethers-middleware::signer::SignerMiddleware"
    /// ref. "ethers-signers::LocalWallet"
    /// ref. "ethers-signers::wallet::Wallet"
    /// ref. "ethers-signers::wallet::Wallet::sign_transaction_sync"
    /// ref. <https://github.com/giantbrain0216/ethers_rs/blob/master/ethers-middleware/tests/nonce_manager.rs>
    pub middleware: Arc<
        NonceManagerMiddleware<
            SignerMiddleware<
                GasEscalatorMiddleware<Arc<Provider<RetryClient<Http>>>, GeometricGasPrice>,
                S,
            >,
        >,
    >,

    pub chain_id: U256,
}

impl<'a, T, S> Evm<'a, T, S>
where
    T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone,
    S: ethers_signers::Signer + Clone,
    S::Error: 'static,
{
    /// Fetches the current balance of the wallet owner.
    pub async fn balance(&self) -> io::Result<U256> {
        let cur_balance =
            jsonrpc_client_evm::get_balance(&self.chain_rpc_url, self.inner.h160_address).await?;
        Ok(cur_balance)
    }
}

/// Converts WEI to GWEI.
pub fn wei_to_gwei(wei: impl Into<U256>) -> U256 {
    let wei: U256 = wei.into();
    if wei.is_zero() {
        U256::zero()
    } else {
        wei.div(*GWEI)
    }
}
