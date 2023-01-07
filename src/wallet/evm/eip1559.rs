use std::io::{self, Error, ErrorKind};

use crate::{
    key,
    wallet::{self, evm},
};
use ethers::prelude::Eip1559TransactionRequest;
use ethers_core::types::{transaction::eip2718::TypedTransaction, RecoveryMessage, Signature};
use ethers_providers::Middleware;
use primitive_types::{H160, H256, U256};
use tokio::time::Duration;

impl<'a, T, S> evm::Evm<'a, T, S>
where
    T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone,
    S: ethers_signers::Signer + Clone,
    S::Error: 'static,
{
    #[must_use]
    pub fn eip1559(&self) -> Tx<'a, T, S> {
        Tx::new(self)
    }
}

/// Represents an EIP-1559 Ethereum transaction (dynamic fee transaction in coreth/subnet-evm).
/// ref. <https://ethereum.org/en/developers/docs/transactions>
/// ref. <https://github.com/ethereum/EIPs/blob/master/EIPS/eip-1559.md>
/// ref. "ethers-core::types::transaction::eip1559::Eip1559TransactionRequest"
/// ref. <https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_signtransaction>
/// ref. <https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendtransaction>
/// ref. <https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendrawtransaction>
/// ref. <https://pkg.go.dev/github.com/ava-labs/subnet-evm/core/types#DynamicFeeTx>
///
/// The transaction cost is "value" + "gas" * "gas_price" in coreth (ref. "types.Transaction.Cost").
/// Which is, "value" + "gas_limit" * "max_fee_per_gas".
/// The transaction cost must be smaller than the originator's balance.
/// Otherwise, fails with "insufficient funds for gas * price + value: address ... have (0) want (x)".
///
/// "max_fee_per_gas" cannot be lower than the pool's minimum fee.
/// And the pool's minimum fee is set
/// Otherwise, fails with "transaction underpriced: address ... have gas fee cap (0) < pool minimum fee cap (25000000000)".
#[derive(Clone, Debug)]
pub struct Tx<'a, T, S>
where
    T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone,
    S: ethers_signers::Signer + Clone,
    S::Error: 'static,
{
    pub inner: wallet::evm::Evm<'a, T, S>,

    /// Sequence number originated from this account to prevent message replay attack
    /// ref. <https://eips.ethereum.org/EIPS/eip-155>
    ///
    /// Must keep track of nonces when creating transactions programmatically.
    /// If two transactions were transmitted with the same nonce,
    /// only one will be confirmed and the other will be rejected.
    ///
    /// Note that nonce increments regardless whether a transaction execution succeeds or not.
    /// The nonce increments when the transaction is included in the block, but
    /// its execution can fail and still pays the gas.
    ///
    /// None for automatically fetching the next available nonce.
    pub signer_nonce: Option<U256>,

    /// Maximum transaction fee as a premium.
    /// Maps to subnet-evm DynamicFeeTx "GasTipCap".
    pub max_priority_fee_per_gas: Option<U256>,

    /// Maximum amount that the originator is willing to pay for this transaction.
    /// Maps to subnet-evm DynamicFeeTx "GasFeeCap".
    pub max_fee_per_gas: Option<U256>,

    /// "gas_limit" is the maximum amount of gas that the originator is willing
    /// to buy for this transaction (e.g., fuel tank capacity).
    /// For instance, if a transaction requires 5 gas units, the transaction can
    /// cost up to 5 * "gas_price".
    pub gas_limit: Option<U256>,

    /// If an externally-owned account, the transaction will transfer value.
    /// If a contract account, the transaction will execute the contract code.
    /// None means contract creation.
    /// The contract address is created from signer address and transaction nonce.
    pub recipient: Option<H160>,

    /// Transfer amount value.
    pub value: Option<U256>,

    /// Arbitrary data.
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
    pub fn new(ev: &wallet::evm::Evm<'a, T, S>) -> Self {
        Self {
            inner: ev.clone(),

            signer_nonce: None,

            max_priority_fee_per_gas: None,
            max_fee_per_gas: None,
            gas_limit: None,

            recipient: None,
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

    /// Same as "GasTipCap" in subnet-evm.
    #[must_use]
    pub fn max_priority_fee_per_gas(mut self, max_priority_fee_per_gas: impl Into<U256>) -> Self {
        self.max_priority_fee_per_gas = Some(max_priority_fee_per_gas.into());
        self
    }

    /// Same as "GasFeeCap" in subnet-evm.
    #[must_use]
    pub fn max_fee_per_gas(mut self, max_fee_per_gas: impl Into<U256>) -> Self {
        self.max_fee_per_gas = Some(max_fee_per_gas.into());
        self
    }

    #[must_use]
    pub fn gas_limit(mut self, gas_limit: impl Into<U256>) -> Self {
        self.gas_limit = Some(gas_limit.into());
        self
    }

    #[must_use]
    pub fn recipient(mut self, to: impl Into<H160>) -> Self {
        self.recipient = Some(to.into());
        self
    }

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

    /// Issues the transaction and returns the transaction Id.
    /// ref. "coreth,subnet-evm/internal/ethapi.SubmitTransaction"
    pub async fn submit(&self) -> io::Result<H256> {
        log::info!(
            "submitting transaction [chain Id {}, value {:?}, from {}, recipient {:?}, http rpc {}, chain RPC {}, max_priority_fee_per_gas {:?}, max_fee_per_gas {:?}, gas_limit {:?}]",
            self.inner.chain_id,
            self.value,
            self.inner.inner.h160_address,
            self.recipient,
            self.inner.picked_http_rpc.1,
            self.inner.chain_rpc_url_path,
            self.max_priority_fee_per_gas,
            self.max_fee_per_gas,
            self.gas_limit,
        );

        let signer_nonce = if let Some(signer_nonce) = self.signer_nonce {
            signer_nonce
        } else {
            log::info!("nonce not specified -- fetching latest");
            self.inner
                .picked_middleware
                .initialize_nonce(None)
                .await
                .map_err(|e| {
                    Error::new(ErrorKind::Other, format!("failed initialize_nonce '{}'", e))
                })?
        };
        log::info!("latest signer nonce {}", signer_nonce);

        // "from" itself is not RLP-encoded field
        // "from" can be simply derived from signature and transaction hash
        // when the RPC decodes the raw transaction
        // ref. <https://github.com/gakonst/ethers-rs/blob/master/ethers-core/src/types/transaction/eip1559.rs>
        // ref. <https://eips.ethereum.org/EIPS/eip-1559>
        // ref. <https://github.com/gakonst/ethers-rs/blob/master/ethers-core/src/types/transaction/eip2718.rs>
        // ref. <https://eips.ethereum.org/EIPS/eip-2718>
        let mut tx_request = Eip1559TransactionRequest::new()
            .from(ethers::prelude::H160::from(
                self.inner.inner.h160_address.as_fixed_bytes(),
            ))
            .chain_id(ethers::prelude::U64::from(self.inner.chain_id.as_u64()))
            .nonce(ethers::prelude::U256::from(signer_nonce.as_u128()));

        if let Some(to) = &self.recipient {
            tx_request = tx_request.to(ethers::prelude::H160::from(to.as_fixed_bytes()));
        }

        if let Some(value) = &self.value {
            let converted: ethers::prelude::U256 = value.into();
            tx_request = tx_request.value(converted);
        }

        if let Some(max_priority_fee_per_gas) = &self.max_priority_fee_per_gas {
            let converted: ethers::prelude::U256 = max_priority_fee_per_gas.into();
            tx_request = tx_request.max_priority_fee_per_gas(converted);
        }

        if let Some(max_fee_per_gas) = &self.max_fee_per_gas {
            let converted: ethers::prelude::U256 = max_fee_per_gas.into();
            tx_request = tx_request.max_fee_per_gas(converted);
        }

        if let Some(gas_limit) = &self.gas_limit {
            let converted: ethers::prelude::U256 = gas_limit.into();
            tx_request = tx_request.gas(converted);
        }

        if let Some(data) = &self.data {
            tx_request = tx_request.data(data.clone());
        }

        let pending_tx = self
            .inner
            .picked_middleware
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

        let tx = self
            .inner
            .picked_middleware
            .get_transaction(tx_receipt.transaction_hash)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed get_transaction '{}'", e)))?;

        // serde_json::to_string(&tx).unwrap()
        if let Some(inner) = &tx {
            assert_eq!(inner.hash(), tx_receipt.transaction_hash);
            log::info!("successfully issued transaction '{}'", inner.hash());
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

/// Transaction but without provider.
#[derive(Clone, Debug)]
pub struct Transaction {
    pub chain_id: u64,
    pub signer_nonce: Option<U256>,
    pub max_priority_fee_per_gas: Option<U256>,
    pub max_fee_per_gas: Option<U256>,
    pub gas_limit: Option<U256>,
    pub from: H160,
    pub recipient: Option<H160>,
    pub value: Option<U256>,
    pub data: Option<Vec<u8>>,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            chain_id: 0,
            signer_nonce: None,

            max_priority_fee_per_gas: None,
            max_fee_per_gas: None,
            gas_limit: None,

            from: H160::zero(),
            recipient: None,
            value: None,
            data: None,
        }
    }

    #[must_use]
    pub fn chain_id(mut self, chain_id: impl Into<u64>) -> Self {
        self.chain_id = chain_id.into();
        self
    }

    #[must_use]
    pub fn signer_nonce(mut self, signer_nonce: impl Into<U256>) -> Self {
        self.signer_nonce = Some(signer_nonce.into());
        self
    }

    #[must_use]
    pub fn max_priority_fee_per_gas(mut self, max_priority_fee_per_gas: impl Into<U256>) -> Self {
        self.max_priority_fee_per_gas = Some(max_priority_fee_per_gas.into());
        self
    }

    #[must_use]
    pub fn max_fee_per_gas(mut self, max_fee_per_gas: impl Into<U256>) -> Self {
        self.max_fee_per_gas = Some(max_fee_per_gas.into());
        self
    }

    #[must_use]
    pub fn gas_limit(mut self, gas_limit: impl Into<U256>) -> Self {
        self.gas_limit = Some(gas_limit.into());
        self
    }

    #[must_use]
    pub fn from(mut self, from: impl Into<H160>) -> Self {
        self.from = from.into();
        self
    }

    #[must_use]
    pub fn recipient(mut self, to: impl Into<H160>) -> Self {
        self.recipient = Some(to.into());
        self
    }

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

    /// Signs the transaction as "ethers_core::types::transaction::eip2718::TypedTransaction"
    /// and returns the rlp-encoded bytes that can be sent via "eth_sendRawTransaction".
    /// ref. <https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendrawtransaction>
    pub async fn sign_as_typed_transaction(
        &self,
        eth_signer: impl ethers_signers::Signer + Clone,
    ) -> io::Result<ethers_core::types::Bytes> {
        let mut tx_request = Eip1559TransactionRequest::new()
            .from(ethers::prelude::H160::from(self.from.as_fixed_bytes()))
            .chain_id(ethers::prelude::U64::from(self.chain_id));

        if let Some(signer_nonce) = self.signer_nonce {
            tx_request = tx_request.nonce(signer_nonce);
        }

        if let Some(to) = &self.recipient {
            tx_request = tx_request.to(ethers::prelude::H160::from(to.as_fixed_bytes()));
        }

        if let Some(value) = &self.value {
            let converted: ethers::prelude::U256 = value.into();
            tx_request = tx_request.value(converted);
        }

        if let Some(max_priority_fee_per_gas) = &self.max_priority_fee_per_gas {
            let converted: ethers::prelude::U256 = max_priority_fee_per_gas.into();
            tx_request = tx_request.max_priority_fee_per_gas(converted);
        }

        if let Some(max_fee_per_gas) = &self.max_fee_per_gas {
            let converted: ethers::prelude::U256 = max_fee_per_gas.into();
            tx_request = tx_request.max_fee_per_gas(converted);
        }

        if let Some(gas_limit) = &self.gas_limit {
            let converted: ethers::prelude::U256 = gas_limit.into();
            tx_request = tx_request.gas(converted);
        }

        if let Some(data) = &self.data {
            tx_request = tx_request.data(data.clone());
        }

        let tx: TypedTransaction = tx_request.into();
        let sig = eth_signer.sign_transaction(&tx).await.map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed to sign_transaction '{}'", e),
            )
        })?;

        Ok(tx.rlp_signed(&sig))
    }

    /// Decodes the RLP-encoded signed "ethers_core::types::transaction::eip2718::TypedTransaction" bytes.
    /// ref. <https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendrawtransaction>
    pub fn decode_signed_rlp(b: impl AsRef<[u8]>) -> io::Result<(TypedTransaction, Signature)> {
        let r = rlp::Rlp::new(b.as_ref());
        TypedTransaction::decode_signed(&r)
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed decode_signed '{}'", e)))
    }

    /// Decodes the RLP-encoded signed "ethers_core::types::transaction::eip2718::TypedTransaction" bytes.
    /// And verifies the decoded signature.
    /// It returns the typed transaction, transaction hash, its signer address, and the signature.
    /// ref. <https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendrawtransaction>
    pub fn decode_and_verify_signed_rlp(
        b: impl AsRef<[u8]>,
    ) -> io::Result<(TypedTransaction, H256, H160, Signature)> {
        let r = rlp::Rlp::new(b.as_ref());
        let (decoded_tx, sig) = TypedTransaction::decode_signed(&r)
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed decode_signed '{}'", e)))?;

        let tx_hash = decoded_tx.sighash();
        log::debug!("decoded signed transaction hash: 0x{:x}", tx_hash);

        let signer_addr = sig.recover(RecoveryMessage::Hash(tx_hash)).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!(
                    "failed to recover signer address from signature and signed transaction hash '{}'",
                    e
                ),
            )
        })?;

        sig.verify(RecoveryMessage::Hash(tx_hash), signer_addr)
            .map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!(
                        "failed to verify signature against the signed transaction hash '{}'",
                        e
                    ),
                )
            })?;
        log::info!(
            "verified signer address '{}' against signature and transaction hash",
            signer_addr
        );

        Ok((decoded_tx, tx_hash, signer_addr, sig))
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib --all-features -- wallet::evm::eip1559::test_transaction --exact --show-output
#[test]
fn test_transaction() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    macro_rules! ab {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    let k1 = key::secp256k1::private_key::Key::generate().unwrap();
    let key_info1 = k1.to_info(1234).unwrap();
    log::info!("created {}", key_info1.h160_address);

    let k2 = key::secp256k1::private_key::Key::generate().unwrap();
    let key_info2 = k2.to_info(1234).unwrap();
    log::info!("created {}", key_info2.h160_address);

    let chain_id = random_manager::u64() % 3000;
    let signer_nonce = U256::from(random_manager::u64() % 10);
    let gas_limit = U256::from(random_manager::u64() % 10000);
    let max_fee_per_gas = U256::from(random_manager::u64() % 10000);
    let value = U256::from(random_manager::u64() % 100000);

    let tx = Transaction::new()
        .chain_id(chain_id)
        .from(key_info1.h160_address)
        .recipient(key_info2.h160_address)
        .signer_nonce(signer_nonce)
        .max_fee_per_gas(max_fee_per_gas)
        .gas_limit(gas_limit)
        .value(value);

    let eth_signer: ethers_signers::LocalWallet = k1.signing_key().into();

    let signed_bytes = ab!(tx.sign_as_typed_transaction(eth_signer)).unwrap();
    log::info!("signed_bytes: {}", signed_bytes);

    let (decoded_tx, sig) = Transaction::decode_signed_rlp(&signed_bytes).unwrap();
    let (decoded_tx2, _tx_hash, signer_addr, sig2) =
        Transaction::decode_and_verify_signed_rlp(&signed_bytes).unwrap();

    assert_eq!(decoded_tx, decoded_tx2);
    assert_eq!(sig, sig2);
    assert_eq!(decoded_tx.chain_id().unwrap().as_u64(), chain_id);
    assert_eq!(*decoded_tx.from().unwrap(), key_info1.h160_address);
    assert_eq!(signer_addr, key_info1.h160_address);
    assert_eq!(*decoded_tx.to_addr().unwrap(), key_info2.h160_address);
    assert_eq!(decoded_tx.nonce().unwrap().as_u64(), signer_nonce.as_u64());
    assert_eq!(decoded_tx.gas().unwrap().as_u64(), gas_limit.as_u64());
    assert_eq!(decoded_tx.value().unwrap().as_u64(), value.as_u64());
}
