use std::io::{self, Error, ErrorKind};

use crate::{
    client::{self, evm as client_evm},
    evm, hash, key,
};
use ethers_providers::Middleware;
use primitive_types::{H160, H256, U256};
use tokio::time::{sleep, Duration, Instant};

pub const DEFAULT_GAS: u64 = 21000;

/// Represents an Ethereum transaction.
/// ref. https://ethereum.org/en/developers/docs/transactions/
///
/// NOTE: The default coreth and subnet-evm will fail this transaction with
/// "only replay-protected (EIP-155) transactions allowed over RPC".
#[derive(Clone, Debug)]
pub struct Tx<'a, T, S>
where
    T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone,
    S: ethers_signers::Signer + Clone,
    S::Error: 'static,
{
    pub inner: client::wallet::evm::Evm<'a, T, S>,

    /// Sequence number originated from this account to prevent message replay attack
    /// ref. https://eips.ethereum.org/EIPS/eip-155
    pub signer_nonce: Option<U256>,

    /// "gas_price" is what the originator is willing to pay for the gas.
    /// "gas_price" is measured in wei per gas unit.
    pub gas_price: Option<U256>,
    /// "gas_limit" is the maximum amount of gas that the originator is willing
    /// to buy for this transaction (e.g., fuel tank capacity).
    pub gas_limit: Option<U256>,

    pub to: Option<H160>,
    pub value: Option<U256>,
    pub data: Option<Vec<u8>>,

    /// Set "true" to poll transfer status after issuance for its acceptance.
    pub check_acceptance: bool,

    /// Initial wait duration before polling for acceptance.
    pub poll_initial_wait: Duration,
    /// Wait between each poll intervals for acceptance.
    pub poll_interval: Duration,
    /// Maximum duration for polling.
    pub poll_timeout: Duration,

    /// Set to true to return transaction Id for "issue" in dry mode.
    pub dry_mode: bool,
}

impl<'a, T, S> Tx<'a, T, S>
where
    T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone,
    S: ethers_signers::Signer + Clone,
    S::Error: 'static,
{
    pub fn new(ev: &client::wallet::evm::Evm<'a, T, S>) -> Self {
        Self {
            inner: ev.clone(),

            signer_nonce: None,
            gas_price: None,
            gas_limit: Some(U256::from(DEFAULT_GAS)),
            to: None,
            value: None,
            data: None,

            check_acceptance: false,

            poll_initial_wait: Duration::from_millis(500),
            poll_interval: Duration::from_millis(700),
            poll_timeout: Duration::from_secs(300),

            dry_mode: false,
        }
    }

    #[must_use]
    pub fn signer_nonce(mut self, signer_nonce: impl Into<U256>) -> Self {
        self.signer_nonce = Some(signer_nonce.into());
        self
    }

    #[must_use]
    pub fn gas_price(mut self, gas_price: impl Into<U256>) -> Self {
        self.gas_price = Some(gas_price.into());
        self
    }

    #[must_use]
    pub fn gas_limit(mut self, gas_limit: impl Into<U256>) -> Self {
        self.gas_limit = Some(gas_limit.into());
        self
    }

    /// Sets the transfer fund receiver address.
    #[must_use]
    pub fn to(mut self, to: impl Into<H160>) -> Self {
        self.to = Some(to.into());
        self
    }

    /// Sets the transfer amount.
    #[must_use]
    pub fn value(mut self, value: impl Into<U256>) -> Self {
        self.value = Some(value.into());
        self
    }

    #[must_use]
    pub fn data(mut self, data: impl Into<Vec<u8>>) -> Self {
        self.data = Some(data.into());
        self
    }

    /// Sets the check acceptance boolean flag.
    #[must_use]
    pub fn check_acceptance(mut self, check_acceptance: bool) -> Self {
        self.check_acceptance = check_acceptance;
        self
    }

    /// Sets the initial poll wait time.
    #[must_use]
    pub fn poll_initial_wait(mut self, poll_initial_wait: Duration) -> Self {
        self.poll_initial_wait = poll_initial_wait;
        self
    }

    /// Sets the poll wait time between intervals.
    #[must_use]
    pub fn poll_interval(mut self, poll_interval: Duration) -> Self {
        self.poll_interval = poll_interval;
        self
    }

    /// Sets the poll timeout.
    #[must_use]
    pub fn poll_timeout(mut self, poll_timeout: Duration) -> Self {
        self.poll_timeout = poll_timeout;
        self
    }

    /// Sets the dry mode boolean flag.
    #[must_use]
    pub fn dry_mode(mut self, dry_mode: bool) -> Self {
        self.dry_mode = dry_mode;
        self
    }

    /// Issues the transaction with "ethers" and returns the transaction Id.
    /// ref. "coreth,subnet-evm/internal/ethapi.SubmitTransaction"
    pub async fn submit(&self) -> io::Result<H256> {
        let picked_http_rpc = self.inner.inner.pick_http_rpc();
        log::info!(
            "issuing ethers transaction [chain Id {}, value {:?}, from {}, to {:?}, http rpc {}, chain RPC {}, gas_price {:?}, gas_limit {:?}]",
            self.inner.chain_id,
            self.value,
            self.inner.inner.h160_address,
            self.to,
            picked_http_rpc.1,
            self.inner.chain_rpc_url_path,
            self.gas_price,
            self.gas_limit,
        );

        let signer_nonce = if let Some(signer_nonce) = self.signer_nonce {
            signer_nonce
        } else {
            self.inner.latest_nonce().await?
        };
        log::info!("latest signer nonce {}", signer_nonce);

        let mut tx_request = ethers::prelude::TransactionRequest::new()
            .from(ethers::prelude::H160::from(
                self.inner.inner.h160_address.as_fixed_bytes(),
            ))
            .chain_id(ethers::prelude::U64::from(self.inner.chain_id.as_u64()))
            .nonce(ethers::prelude::U256::from(signer_nonce.as_u128()));

        if let Some(to) = &self.to {
            tx_request = tx_request.to(ethers::prelude::H160::from(to.as_fixed_bytes()));
        }
        if let Some(value) = &self.value {
            tx_request = tx_request.value(ethers::prelude::U256::from(value.as_u128()));
        }
        if let Some(gas_price) = &self.gas_price {
            tx_request = tx_request.gas_price(ethers::prelude::U256::from(gas_price.as_u128()));
        }
        if let Some(gas_limit) = &self.gas_limit {
            tx_request = tx_request.gas(ethers::prelude::U256::from(gas_limit.as_u128()));
        }
        if let Some(data) = &self.data {
            tx_request = tx_request.data(data.clone());
        }

        // ref. "ethers-middleware::signer::SignerMiddleware"
        // ref. "ethers-signers::LocalWallet"
        // ref. "ethers-signers::wallet::Wallet"
        let signer = ethers::prelude::SignerMiddleware::new(
            self.inner.providers[picked_http_rpc.0].clone(),
            self.inner
                .eth_signer
                .clone()
                .with_chain_id(self.inner.chain_id.as_u64()),
        );

        let pending_tx = signer
            .send_transaction(tx_request, None)
            .await
            .map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!("failed to send_transaction '{}'", e),
                )
            })?;

        let tx_receipt = pending_tx.await.map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed to wait for pending tx '{}'", e),
            )
        })?;
        if tx_receipt.is_none() {
            return Err(Error::new(ErrorKind::Other, "tx dropped from mempool"));
        }
        let tx_receipt = tx_receipt.unwrap();
        let tx_hash = H256(tx_receipt.transaction_hash.0);

        let tx = signer
            .get_transaction(tx_receipt.transaction_hash)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed get_transaction '{}'", e)))?;

        // serde_json::to_string(&tx).unwrap()
        if let Some(inner) = &tx {
            assert_eq!(inner.hash(), tx_receipt.transaction_hash);
            log::info!("{} successfully issued", inner.hash());
        } else {
            log::warn!("transaction not found in get_transaction");
        }

        if !self.check_acceptance {
            log::debug!("skipping checking acceptance...");
            return Ok(tx_hash);
        }

        Ok(tx_hash)
    }
}

impl<'a, T, S> Tx<'a, T, S>
where
    T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone,
    S: ethers_signers::Signer + Clone,
    S::Error: 'static,
{
    /// Issues the transaction and returns the transaction Id.
    #[deprecated(note = "not working... TODO: fix")]
    pub async fn submit0(&self) -> io::Result<H256> {
        let picked_http_rpc = self.inner.inner.pick_http_rpc();
        log::info!(
            "issuing transaction [chain Id {}, value {:?} AVAX, from {}, to {:?}, http rpc {}, chain RPC {}, gas price {:?}, gas limit {:?}]",
            self.inner.chain_id,
            self.value,
            self.inner.inner.h160_address,
            self.to,
            picked_http_rpc.1,
            self.inner.chain_rpc_url_path,
            self.gas_price,
            self.gas_limit,
        );

        let signer_nonce = if let Some(signer_nonce) = self.signer_nonce {
            signer_nonce
        } else {
            self.inner.latest_nonce().await?
        };
        log::info!("latest signer nonce {}", signer_nonce);

        let mut tx = evm::txs::legacy::Tx::default().signer_nonce(signer_nonce);

        if let Some(to) = self.to {
            tx = tx.to(to);
        }
        if let Some(value) = self.value {
            tx = tx.value(value);
        }
        if let Some(gas_price) = self.gas_price {
            tx = tx.gas_price(gas_price);
        }
        if let Some(gas_limit) = self.gas_limit {
            tx = tx.gas_limit(gas_limit);
        }
        if let Some(data) = &self.data {
            tx = tx.data(data.clone());
        }

        let tx_bytes_signed = tx.sign(self.inner.inner.keychain.keys[0].clone()).await?;
        let tx_bytes_signed_hex = format!("0x{}", hex::encode(&tx_bytes_signed));
        let tx_hash = hash::keccak256(&tx_bytes_signed);

        if self.dry_mode {
            log::debug!("dry mode... returning...");
            return Ok(tx_hash);
        }

        let resp = client_evm::send_raw_transaction(
            &picked_http_rpc.1,
            &self.inner.chain_id_alias,
            &tx_bytes_signed_hex,
        )
        .await?;

        if let Some(r) = &resp.result {
            assert_eq!(tx_hash, *r);
        } else {
            // "coreth,subnet-evm/eth.EthAPIBackend.SendTx" adds this transaction to its transaction pool
            // which means, this transaction was sent but still pending, thus no result
            log::info!("no result found for eth_sendRawTransaction, use precomputed hash");
        };
        log::info!("transaction hash {}", tx_hash);

        if !self.check_acceptance {
            log::debug!("skipping checking acceptance...");
            return Ok(H256::zero());
        }

        // enough time for txs processing
        log::info!("initial waiting {:?}", self.poll_initial_wait);
        sleep(self.poll_initial_wait).await;

        log::info!("polling to confirm transaction");
        let (start, mut success) = (Instant::now(), false);
        loop {
            let elapsed = start.elapsed();
            if elapsed.gt(&self.poll_timeout) {
                break;
            }

            let resp = client_evm::get_transaction_receipt(
                &picked_http_rpc.1,
                &self.inner.chain_id_alias,
                &format!("0x{:x}", tx_hash),
            )
            .await?;

            if let Some(r) = &resp.result {
                let status = r.status.as_u64();
                log::info!("tx {} status {}", tx_hash, status);
                success = status == 1;
                if success {
                    break;
                }
            }

            sleep(self.poll_interval).await;
        }
        if !success {
            return Err(Error::new(
                ErrorKind::Other,
                "failed to check acceptance in time",
            ));
        }

        Ok(tx_hash)
    }
}
