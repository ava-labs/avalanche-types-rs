use std::io;

use crate::{hash, key};
use primitive_types::{H160, U256};
use rlp::RlpStream;

/// NOT WORKING...
/// TODO: fix signature...
///
/// NOTE: The default coreth and subnet-evm will fail this transaction with
/// "only replay-protected (EIP-155) transactions allowed over RPC".
///
/// Represents a legacy Ethereum transaction.
/// ref. https://ethereum.org/en/developers/docs/transactions
/// ref. "ethers-core::types::transaction::TransactionRequest"
/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_signtransaction
/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendtransaction
/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendrawtransaction
/// ref. https://pkg.go.dev/github.com/ava-labs/subnet-evm/core/types#LegacyTx
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tx {
    /// Sequence number originated from this account to prevent message replay attack
    /// ref. https://eips.ethereum.org/EIPS/eip-155
    ///
    /// Must keep track of nonces when creating transactions programmatically.
    /// If two transactions were transmitted with the same nonce,
    /// only one will be confirmed and the other will be rejected.
    ///
    /// None for next available nonce.
    pub signer_nonce: Option<U256>,
    /// Gas controls the amount of resources that this transaction can use.
    /// "gas_price" is what the originator is willing to pay for the gas.
    /// "gas_price" is measured in wei per gas unit.
    /// If the "gas_price" is 5 gwei, the account is willing to pay
    /// 5 billion wei for the gas. The higher "gas_price", the faster
    /// the transaction is likely to be confirmed.
    pub gas_price: Option<U256>,
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

    pub chain_id: U256,
}

impl Default for Tx {
    fn default() -> Self {
        Self::default()
    }
}

impl Tx {
    pub fn default() -> Self {
        Self {
            signer_nonce: None,
            gas_price: None,
            gas_limit: None,
            to: None,
            value: None,
            data: None,

            chain_id: U256::zero(),
        }
    }

    #[must_use]
    pub fn signer_nonce<T: Into<U256>>(mut self, signer_nonce: T) -> Self {
        self.signer_nonce = Some(signer_nonce.into());
        self
    }

    #[must_use]
    pub fn gas_price<T: Into<U256>>(mut self, gas_price: T) -> Self {
        self.gas_price = Some(gas_price.into());
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
    pub fn chain_id<T: Into<U256>>(mut self, chain_id: T) -> Self {
        self.chain_id = chain_id.into();
        self
    }

    /// RLP-encodes the base fields.
    /// ref. "ethers-core::types::transaction::TransactionRequest::rlp"
    /// ref. https://github.com/onbjerg/ethers-flashbots/issues/11
    fn rlp_base(&self, rlp: &mut RlpStream) {
        super::rlp_opt(rlp, &self.signer_nonce); // #1
        super::rlp_opt(rlp, &self.gas_price); // #2
        super::rlp_opt(rlp, &self.gas_limit); // #3
        super::rlp_opt(rlp, &self.to); // #4
        super::rlp_opt(rlp, &self.value); // #5
        super::rlp_opt(rlp, &self.data); // #6
    }

    /// ref. "ethers-core::types::transaction::TransactionRequest::rlp"
    fn rlp_with_no_signature(&self) -> Vec<u8> {
        let mut rlp = RlpStream::new();
        rlp.begin_list(9);
        self.rlp_base(&mut rlp);

        // #7 ~ #9
        // first encode transaction nine fields: ..., chain_id, 0, 0
        // ref. "ethers-core::types::transaction::TransactionRequest::rlp"
        // ref. "ethers-core::types::transaction::TransactionRequest::sighash"
        rlp.append(&self.chain_id);
        rlp.append(&0u8);
        rlp.append(&0u8);

        rlp.out().freeze().into()
    }

    pub async fn sign<T: key::secp256k1::SignOnly + Clone>(
        &self,
        signer: T,
    ) -> io::Result<Vec<u8>> {
        // produce an RLP-encoded serialized message and Keccak-256 hash it
        // ref. "ethers-core::types::transaction::TransactionRequest::sighash"
        let tx_bytes_hash = hash::keccak256(&self.rlp_with_no_signature());

        // compute the ECDSA signature with private key
        // ref. "ethers-core::types::Signature::try_from(bytes: &'a [u8])"
        // ref. "ethers-signers::wallet::Wallet::sign_transaction_sync"
        let sighash = signer.sign_digest(tx_bytes_hash.as_ref()).await?;
        let sig = key::secp256k1::signature::Sig::from_bytes(&sighash)?;

        Ok(self.rlp_with_signature(sig))
    }

    /// appends three components of an ECDSA signature of the originating key
    /// ref. "ethers-core::types::transaction::TransactionRequest::rlp_signed"
    /// ref. "ethers-middleware::signer::SignerMiddleware::sign_transaction"
    /// ref. "ethers-signers::wallet::Wallet::sign_transaction_sync"
    /// ref. "ethers-core::types::transaction::TransactionRequest::sighash"
    /// ref. "ethers-signers::wallet::Wallet::sign_hash"
    fn rlp_with_signature(&self, sig: key::secp256k1::signature::Sig) -> Vec<u8> {
        let mut rlp = RlpStream::new();
        rlp.begin_list(9);
        self.rlp_base(&mut rlp);

        rlp.append(&sig.v());
        rlp.append(&sig.r());
        rlp.append(&sig.s());

        rlp.out().freeze().into()
    }
}
