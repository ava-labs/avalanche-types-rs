use std::io::{self, Error, ErrorKind};

use crate::{evm::AccessList, hash, key};
use primitive_types::{H160, U256};
use rlp::RlpStream;

/// NOT WORKING...
/// TODO: fix signature...
///
/// Represents an EIP-1559 Ethereum transaction (dynamic fee transaction in coreth/subnet-evm).
///
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
///
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tx {
    pub chain_id: U256,

    /// Sequence number originated from this account to prevent message replay attack
    /// ref. <https://eips.ethereum.org/EIPS/eip-155>
    ///
    /// Must keep track of nonces when creating transactions programmatically.
    /// If two transactions were transmitted with the same nonce,
    /// only one will be confirmed and the other will be rejected.
    ///
    /// None for next available nonce.
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

    /// Transfer fund receiver address.
    /// None means contract creation.
    pub to: Option<H160>,
    /// Transfer amount value.
    pub value: Option<U256>,

    /// Binary data payload.
    /// This can be compiled code of a contract OR the hash of the invoked
    /// method signature and encoded parameters.
    pub data: Option<Vec<u8>>,

    pub access_list: AccessList,
}

impl Default for Tx {
    fn default() -> Self {
        Self::default()
    }
}

pub const DEFAULT_GAS_FEE_CAP: u64 = 25000000000;
pub const DEFAULT_GAS_LIMIT: u64 = 21000;

impl Tx {
    pub fn default() -> Self {
        Self {
            chain_id: U256::zero(),
            signer_nonce: None,

            max_priority_fee_per_gas: None,
            max_fee_per_gas: Some(primitive_types::U256::from(DEFAULT_GAS_FEE_CAP)),
            gas_limit: Some(primitive_types::U256::from(DEFAULT_GAS_LIMIT)),

            to: None,
            value: None,
            data: None,
            access_list: AccessList::default(),
        }
    }

    #[must_use]
    pub fn chain_id<T: Into<U256>>(mut self, chain_id: T) -> Self {
        self.chain_id = chain_id.into();
        self
    }

    #[must_use]
    pub fn signer_nonce<T: Into<U256>>(mut self, signer_nonce: T) -> Self {
        self.signer_nonce = Some(signer_nonce.into());
        self
    }

    #[must_use]
    pub fn max_priority_fee_per_gas<T: Into<U256>>(mut self, max_priority_fee_per_gas: T) -> Self {
        self.max_priority_fee_per_gas = Some(max_priority_fee_per_gas.into());
        self
    }

    #[must_use]
    pub fn max_fee_per_gas<T: Into<U256>>(mut self, max_fee_per_gas: T) -> Self {
        self.max_fee_per_gas = Some(max_fee_per_gas.into());
        self
    }

    #[must_use]
    pub fn gas_limit<T: Into<U256>>(mut self, gas_limit: T) -> Self {
        self.gas_limit = Some(gas_limit.into());
        self
    }

    #[must_use]
    pub fn to<T: Into<H160>>(mut self, to: T) -> Self {
        self.to = Some(to.into());
        self
    }

    #[must_use]
    pub fn value<T: Into<U256>>(mut self, value: T) -> Self {
        self.value = Some(value.into());
        self
    }

    #[must_use]
    pub fn data<T: Into<Vec<u8>>>(mut self, data: T) -> Self {
        self.data = Some(data.into());
        self
    }

    #[must_use]
    pub fn access_list<T: Into<AccessList>>(mut self, access_list: T) -> Self {
        self.access_list = access_list.into();
        self
    }

    /// RLP-encodes the base fields.
    /// ref. "ethers-core::types::transaction::eip2718::TypedTransaction::rlp"
    /// ref. "ethers-core::types::transaction::eip1559::Eip1559TransactionRequest::rlp"
    /// ref. <https://github.com/onbjerg/ethers-flashbots/issues/11>
    fn rlp_base(&self, rlp: &mut RlpStream) {
        rlp.append(&self.chain_id); // #1
        super::rlp_opt(rlp, &self.signer_nonce); // #2
        super::rlp_opt(rlp, &self.max_priority_fee_per_gas); // #3
        super::rlp_opt(rlp, &self.max_fee_per_gas); // #4
        super::rlp_opt(rlp, &self.gas_limit); // #5
        super::rlp_opt(rlp, &self.to); // #6
        super::rlp_opt(rlp, &self.value); // #7
        super::rlp_opt(rlp, &self.data); // #8
        rlp.append(&self.access_list); // #9
    }

    /// ref. "ethers-core::types::transaction::eip2718::TypedTransaction::rlp"
    /// ref. "ethers-core::types::transaction::eip1559::Eip1559TransactionRequest::rlp"
    fn rlp_with_no_signature(&self) -> Vec<u8> {
        let mut rlp = RlpStream::new();
        rlp.begin_list(9);
        self.rlp_base(&mut rlp);

        rlp.out().freeze().into()
    }

    pub async fn sign<T: key::secp256k1::SignOnly + Clone>(
        &self,
        signer: T,
    ) -> io::Result<Vec<u8>> {
        // produce an RLP-encoded serialized message and Keccak-256 hash it
        // ref. "ethers-core::types::transaction::eip2718::TypedTransaction::sighash"
        let tx_bytes_hash = hash::keccak256(&self.rlp_with_no_signature());

        // compute the ECDSA signature with private key
        // ref. "ethers-core::types::Signature::try_from(bytes: &'a [u8])"
        // ref. "ethers-signers::wallet::Wallet::sign_transaction_sync"
        let sighash = signer
            .sign_digest(tx_bytes_hash.as_ref())
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed sign_digest {}", e)))?;

        let sig = key::secp256k1::signature::Sig::from_bytes(&sighash)?;

        // ref. "ethers-signers::wallet::Wallet::sign_transaction_sync"
        // ref. "ethers-signers::wallet::Wallet::sign_hash"
        let v = sig.v() + 27;

        // ref. "ethers-signers::wallet::Wallet::sign_transaction_sync"
        let v = to_eip155_v(v - 27, self.chain_id);

        Ok(self.rlp_with_signature(sig, v))
    }

    /// appends three components of an ECDSA signature of the originating key
    /// ref. "ethers-core::types::transaction::eip1559::Eip1559TransactionRequest::rlp_signed"
    /// ref. "ethers-middleware::signer::SignerMiddleware::sign_transaction"
    /// ref. "ethers-signers::wallet::Wallet::sign_transaction_sync"
    /// ref. "ethers-core::types::transaction::TransactionRequest::sighash"
    /// ref. "ethers-signers::wallet::Wallet::sign_hash"
    fn rlp_with_signature(&self, sig: key::secp256k1::signature::Sig, sig_v: u64) -> Vec<u8> {
        let mut rlp = RlpStream::new();
        rlp.begin_unbounded_list();
        self.rlp_base(&mut rlp);

        // ref. "ethers-core::types::transaction::eip1559::Eip1559TransactionRequest::rlp_signed"
        let v = normalize_v(sig_v, self.chain_id);
        rlp.append(&v);
        rlp.append(&sig.r());
        rlp.append(&sig.s());

        rlp.finalize_unbounded_list();

        // EIP-1559 (0x02), coreth dynamic fee tx (0x00)
        // ref. "ethers-core::types::transaction::response::Transaction::rlp"
        // ref. "ethers-core::types::transaction::eip2718::TypedTransaction::rlp_signed"
        let mut encoded = vec![];
        encoded.extend_from_slice(&[0x2]);
        encoded.extend_from_slice(rlp.out().freeze().as_ref());
        encoded
    }
}

/// normalizes the signature back to 0/1
/// ref. "ethers-core::types::transaction::normalize_v"
pub fn normalize_v(v: u64, chain_id: U256) -> u64 {
    if v > 1 {
        v - chain_id.as_u64() * 2 - 35
    } else {
        v
    }
}

/// ref. "ethers-signers::to_eip155_v"
/// ref. "ethers-signers::wallet::Wallet::sign_transaction_sync"
pub fn to_eip155_v(recovery_id: u64, chain_id: U256) -> u64 {
    recovery_id + 35 + chain_id.as_u64() * 2
}
