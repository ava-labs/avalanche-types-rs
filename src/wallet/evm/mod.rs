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
use ethers_providers::{Http, Provider, RetryClient};
use lazy_static::lazy_static;
use primitive_types::U256;

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
        // TODO: make this configurable
        let max_retries = 10;
        let backoff_timeout = 3000; // in ms

        // do not create multiple providers for the ease of nonce management
        let provider =
            Provider::<RetryClient<Http>>::new_client(chain_rpc_url, max_retries, backoff_timeout)
                .map_err(|e| {
                    Error::new(
                        ErrorKind::Other,
                        format!("failed to create provider '{}'", e),
                    )
                })?
                .interval(Duration::from_millis(2000u64));
        let provider_arc = Arc::new(provider);

        // TODO: make this configurable
        let escalator = GeometricGasPrice::new(5.0, 10u64, None::<u64>);
        let gas_escalator_middleware =
            GasEscalatorMiddleware::new(Arc::clone(&provider_arc), escalator, Frequency::PerBlock);
        let signer_middleware = SignerMiddleware::new(
            gas_escalator_middleware,
            eth_signer.clone().with_chain_id(chain_id.as_u64()),
        );
        let nonce_middleware = NonceManagerMiddleware::new(signer_middleware, eth_signer.address());
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
