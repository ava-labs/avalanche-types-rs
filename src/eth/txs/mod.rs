use std::io;

use crate::{hash, key};
use primitive_types::{H160, U256};
use rlp::RlpStream;

/// ref. "ethers-core::types::transaction::TransactionRequest"
/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_signtransaction
/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendtransaction
/// ref. https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendrawtransaction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tx {
    pub chain_id: U256,

    /// Transfer fund sender address.
    pub sender: H160,
    /// Transfer fund receiver address.
    pub receiver: H160,

    /// Transfer amount.
    pub amount: U256,

    /// Gas controls the amount of resources that this transaction can use.
    /// "gas_price" is what the originator is willing to pay for the gas.
    /// "gas_price" is measured in wei per gas unit.
    /// If the "gas_price" is 5 gwei, the account is willing to pay
    /// 5 billion wei for the gas. The higher "gas_price", the faster
    /// the transaction is likely to be confirmed.
    /// "Max fee(GWEI)" in Metamask maps to "GasPrice" in subnet-evm.
    /// "Max priority fee (GWEI)" in Metamask maps to "GasTipCap".
    pub gas_price: Option<U256>,
    /// "gas_limit" is the maximum amount of gas that the originator is willing
    /// to buy for this transaction (e.g., fuel tank capacity).
    /// For instance, if a transaction requires 5 gas units, the transaction can
    /// cost up to 5 * "gas_price".
    pub gas_limit: Option<U256>,

    /// Sequence number originated from this account to prevent message replay attack
    /// ref. https://eips.ethereum.org/EIPS/eip-155
    ///
    /// Must keep track of nonces when creating transactions programmatically.
    /// If two transactions were transmitted with the same nonce,
    /// only one will be confirmed and the other will be rejected.
    ///
    /// None for next available nonce.
    pub nonce: Option<U256>,

    /// Binary data payload.
    /// This can be compiled code of a contract OR the hash of the invoked
    /// method signature and encoded parameters.
    pub data: Option<Vec<u8>>,
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

            sender: H160::from(&[0_u8; 20]),
            receiver: H160::from(&[0_u8; 20]),

            amount: U256::zero(),

            gas_price: None,
            gas_limit: None,

            nonce: None,

            data: None,
        }
    }

    pub async fn sign<T: key::secp256k1::SignOnly + Clone>(
        &self,
        signer: T,
    ) -> io::Result<Vec<u8>> {
        let mut rlp = RlpStream::new();
        rlp.begin_list(9);
        self.rlp_base(&mut rlp);

        // #7 ~ #9
        // first encode transaction nine fields: ..., chain_id, 0, 0
        // ref. "ethers-core::types::transaction::TransactionRequest::rlp"
        rlp.append(&self.chain_id);
        rlp.append(&0u8);
        rlp.append(&0u8);

        // produce an RLP-encoded serialized message and Keccak-256 hash it
        // ref. "ethers-core::types::transaction::TransactionRequest::rlp_signed"
        // ref. "ethers-core::types::transaction::TransactionRequest::sighash"
        let tx_bytes_hash = hash::keccak256(rlp.out().as_ref());

        // compute the ECDSA signature with private key
        let sig_raw = signer.sign_digest(tx_bytes_hash.as_ref()).await?;
        // ref. "ethers-core::types::Signature::try_from(bytes: &'a [u8])"
        let sig = key::secp256k1::signature::Sig::from_bytes(&sig_raw)?;

        let mut rlp = RlpStream::new();
        rlp.begin_list(9);
        self.rlp_base(&mut rlp);

        // three components of an ECDSA signature of the originating key, append the signature
        // ref. "ethers-signers::wallet::Wallet::sign_transaction_sync"
        // ref. "ethers-core::types::transaction::TransactionRequest::sighash"
        // ref. "ethers-signers::wallet::Wallet::sign_hash"
        rlp.append(&sig.v());
        rlp.append(&sig.r());
        rlp.append(&sig.s());

        Ok(rlp.out().freeze().into())
    }

    fn rlp_base(&self, rlp: &mut RlpStream) {
        // #1
        if let Some(nonce) = &self.nonce {
            // "impl_rlp::impl_uint_rlp!(U256, 4)"
            // ref. "primitive-types/lib.rs" with "impl-rlp" feature
            rlp.append(nonce);
        } else {
            rlp.append(&"");
        }

        // #2
        if let Some(gas_price) = &self.gas_price {
            rlp.append(gas_price);
        } else {
            rlp.append(&"");
        }

        // #3
        if let Some(gas) = &self.gas_limit {
            rlp.append(gas);
        } else {
            rlp.append(&"");
        }

        // #4
        rlp.append(&self.receiver);

        // #5
        rlp.append(&self.amount);

        // #6
        if let Some(data) = &self.data {
            rlp.append(data);
        } else {
            rlp.append(&"");
        }
    }
}
