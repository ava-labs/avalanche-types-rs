pub mod eip1559;

use std::{
    io::{self, Error, ErrorKind},
    sync::Arc,
    time::Duration,
};

use crate::{jsonrpc::client::evm as jsonrpc_client_evm, key, wallet};
use ethers::prelude::{
    gas_escalator::{Frequency, GasEscalatorMiddleware, GeometricGasPrice},
    NonceManagerMiddleware, SignerMiddleware,
};
use ethers_providers::{Http, Provider};

impl<T> wallet::Wallet<T>
where
    T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone,
{
    /// Set "chain_id_alias" to either "C" or subnet-evm chain Id.
    /// e.g., "/ext/bc/C/rpc"
    #[must_use]
    pub fn evm<'a, S>(
        &self,
        eth_signer: &'a S,
        chain_id_alias: String,
        chain_id: primitive_types::U256,
    ) -> io::Result<Evm<'a, T, S>>
    where
        S: ethers_signers::Signer + Clone,
        S::Error: 'static,
    {
        let chain_rpc_url_path = format!("/ext/bc/{}/rpc", chain_id_alias).to_string();

        let mut rpc_eps = Vec::new();
        let mut providers = Vec::new();
        for http_rpc in self.http_rpcs.iter() {
            let rpc_ep = format!("{http_rpc}{chain_rpc_url_path}");

            let provider = Provider::<Http>::try_from(&rpc_ep)
                .map_err(|e| {
                    Error::new(
                        ErrorKind::Other,
                        format!("failed to create provider '{}'", e),
                    )
                })?
                .interval(Duration::from_millis(2000u64));
            providers.push(provider);
            rpc_eps.push(rpc_ep);
        }

        // pick one rpc for nonce management
        let picked_http_rpc = self.pick_http_rpc();

        // TODO: make this configurable
        let escalator = GeometricGasPrice::new(5.0, 10u64, None::<u64>);
        let gas_escalator_middleware = GasEscalatorMiddleware::new(
            providers[picked_http_rpc.0].clone(),
            escalator,
            Frequency::PerBlock,
        );
        let signer_middleware = SignerMiddleware::new(
            gas_escalator_middleware,
            eth_signer.clone().with_chain_id(chain_id.as_u64()),
        );
        let nonce_middleware = NonceManagerMiddleware::new(signer_middleware, eth_signer.address());
        let picked_middleware = Arc::new(nonce_middleware);

        Ok(Evm::<'a, T, S> {
            inner: self.clone(),
            eth_signer,

            rpc_eps,
            providers,
            picked_http_rpc,
            picked_middleware,

            chain_id,
            chain_id_alias,
            chain_rpc_url_path,
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

    pub rpc_eps: Vec<String>,
    pub providers: Vec<Provider<Http>>,
    pub picked_http_rpc: (usize, String),

    /// Middleware created on the picked RPC endpoint and signer address.
    /// ref. "ethers-middleware::signer::SignerMiddleware"
    /// ref. "ethers-signers::LocalWallet"
    /// ref. "ethers-signers::wallet::Wallet"
    /// ref. "ethers-signers::wallet::Wallet::sign_transaction_sync"
    /// ref. <https://github.com/giantbrain0216/ethers_rs/blob/master/ethers-middleware/tests/nonce_manager.rs>
    pub picked_middleware: Arc<
        NonceManagerMiddleware<
            SignerMiddleware<GasEscalatorMiddleware<Provider<Http>, GeometricGasPrice>, S>,
        >,
    >,

    pub chain_id: primitive_types::U256,

    /// Either "C" or subnet_evm chain Id.
    pub chain_id_alias: String,
    pub chain_rpc_url_path: String,
}

impl<'a, T, S> Evm<'a, T, S>
where
    T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone,
    S: ethers_signers::Signer + Clone,
    S::Error: 'static,
{
    /// Fetches the current balance of the wallet owner from the specified HTTP endpoint.
    pub async fn balance_with_endpoint(&self, http_rpc: &str) -> io::Result<primitive_types::U256> {
        let cur_balance = jsonrpc_client_evm::get_balance(
            http_rpc,
            &self.chain_id_alias,
            self.inner.h160_address,
        )
        .await?;
        Ok(cur_balance)
    }

    /// Fetches the current balance of the wallet owner from all endpoints
    /// in the same order of "self.http_rpcs".
    pub async fn balances(&self) -> io::Result<Vec<primitive_types::U256>> {
        let mut balances = Vec::new();
        for http_rpc in self.inner.http_rpcs.iter() {
            let balance = self.balance_with_endpoint(http_rpc).await?;
            balances.push(balance);
        }
        Ok(balances)
    }

    /// Fetches the current balance of the wallet owner.
    pub async fn balance(&self) -> io::Result<primitive_types::U256> {
        self.balance_with_endpoint(&self.inner.pick_http_rpc().1)
            .await
    }
}
