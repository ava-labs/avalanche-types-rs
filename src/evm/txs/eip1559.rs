use std::io;

use crate::{evm::AccessList, hash, key};
use primitive_types::{H160, U256};
use rlp::RlpStream;

/// Represents an EIP-1559 Ethereum transaction (dynamic fee transaction in subnet-evm).
/// ref. https://ethereum.org/en/developers/docs/transactions
/// ref. https://github.com/ethereum/EIPs/blob/master/EIPS/eip-1559.md
/// ref. "ethers-core::types::transaction::eip1559::Eip1559TransactionRequest"
/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_signtransaction
/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendtransaction
/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendrawtransaction
/// ref. https://pkg.go.dev/github.com/ava-labs/subnet-evm/core/types#DynamicFeeTx
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tx {
    pub chain_id: U256,
    /// Sequence number originated from this account to prevent message replay attack
    /// ref. https://eips.ethereum.org/EIPS/eip-155
    ///
    /// Must keep track of nonces when creating transactions programmatically.
    /// If two transactions were transmitted with the same nonce,
    /// only one will be confirmed and the other will be rejected.
    ///
    /// None for next available nonce.
    pub signer_nonce: Option<U256>,

    /// Maximum transaction fee as a premium.
    /// Maps to subnet-evm DynamicFeeTx GasTipCap.
    pub max_priority_fee_per_gas: Option<U256>,
    /// Maximum amount that the originator is willing to pay for this transaction.
    /// Maps to subnet-evm DynamicFeeTx GasFeeCap.
    pub max_fee_per_gas: Option<U256>,

    /// "gas_limit" is the maximum amount of gas that the originator is willing
    /// to buy for this transaction (e.g., fuel tank capacity).
    /// For instance, if a transaction requires 5 gas units, the transaction can
    /// cost up to 5 * "gas_price".
    pub gas_limit: Option<U256>,

    /// Transfer fund receiver address.
    pub to: H160,
    /// Transfer amount value.
    pub value: U256,
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

impl Tx {
    pub fn default() -> Self {
        Self {
            chain_id: U256::zero(),
            signer_nonce: None,
            max_priority_fee_per_gas: None,
            max_fee_per_gas: None,
            gas_limit: None,
            to: H160::from(&[0_u8; 20]),
            value: U256::zero(),
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
        self.to = to.into();
        self
    }

    #[must_use]
    pub fn value<T: Into<U256>>(mut self, value: T) -> Self {
        self.value = value.into();
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
    fn rlp_base(&self, rlp: &mut RlpStream) {
        rlp.append(&self.chain_id); // #1
        super::rlp_opt(rlp, &self.signer_nonce); // #2
        super::rlp_opt(rlp, &self.max_priority_fee_per_gas); // #3
        super::rlp_opt(rlp, &self.max_fee_per_gas); // #4
        super::rlp_opt(rlp, &self.gas_limit); // #5
        rlp.append(&self.to); // #6
        rlp.append(&self.value); // #7
        super::rlp_opt(rlp, &self.data); // #8
        rlp.append(&self.access_list); // #9
    }

    pub async fn sign<T: key::secp256k1::SignOnly + Clone>(
        &self,
        signer: T,
    ) -> io::Result<Vec<u8>> {
        // ref.  "ethers-core::types::transaction::eip1559::Eip1559TransactionRequest::rlp"
        let mut rlp = RlpStream::new();
        rlp.begin_list(9);
        self.rlp_base(&mut rlp);

        let tx_bytes_hash = hash::keccak256(rlp.out().freeze().as_ref());
        let sig = key::secp256k1::signature::Sig::from_bytes(
            &signer.sign_digest(tx_bytes_hash.as_ref()).await?,
        )?;

        // ref.  "ethers-core::types::transaction::eip1559::Eip1559TransactionRequest::rlp_signed"
        let mut rlp = RlpStream::new();
        rlp.begin_unbounded_list();
        self.rlp_base(&mut rlp);

        rlp.append(&normalize_v(sig.v(), self.chain_id));
        rlp.append(&sig.r());
        rlp.append(&sig.s());

        rlp.finalize_unbounded_list();
        Ok(rlp.out().freeze().into())
    }
}

/// normalizes the signature back to 0/1
/// ref.  "ethers-core::types::transaction::normalize_v"
fn normalize_v(v: u64, chain_id: U256) -> u64 {
    if v > 1 {
        v - chain_id.as_u64() * 2 - 35
    } else {
        v
    }
}
