use std::io;

use crate::key;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct Signer {
    pub inner: super::Cmk,
    pub chain_id: primitive_types::U256,
    pub address: ethers_core::types::Address,
}

impl Signer {
    pub fn new(inner: super::Cmk, chain_id: primitive_types::U256) -> io::Result<Self> {
        let address: ethers_core::types::Address = inner.to_public_key().to_h160().into();
        Ok(Self {
            inner,
            chain_id,
            address,
        })
    }

    async fn sign_digest_with_eip155(
        &self,
        digest: ethers_core::types::H256,
        chain_id: u64,
    ) -> Result<ethers_core::types::Signature, aws_manager::errors::Error> {
        let sig = self.inner.sign_digest(digest.as_ref()).await?;

        let mut sig = key::secp256k1::signature::rsig_to_ethsig(&sig);
        key::secp256k1::signature::apply_eip155(&mut sig, chain_id);
        Ok(sig)
    }
}

#[async_trait]
impl<'a> ethers_signers::Signer for Signer {
    type Error = aws_manager::errors::Error;

    async fn sign_message<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<ethers_core::types::Signature, Self::Error> {
        let message = message.as_ref();
        let message_hash = ethers_core::utils::hash_message(message);

        self.sign_digest_with_eip155(message_hash, self.chain_id.as_u64())
            .await
    }

    async fn sign_transaction(
        &self,
        tx: &ethers_core::types::transaction::eip2718::TypedTransaction,
    ) -> Result<ethers_core::types::Signature, Self::Error> {
        let mut tx_with_chain = tx.clone();
        let chain_id = tx_with_chain
            .chain_id()
            .map(|id| id.as_u64())
            .unwrap_or(self.chain_id.as_u64());
        tx_with_chain.set_chain_id(chain_id);

        let sighash = tx_with_chain.sighash();
        self.sign_digest_with_eip155(sighash, chain_id).await
    }

    async fn sign_typed_data<T: ethers_core::types::transaction::eip712::Eip712 + Send + Sync>(
        &self,
        payload: &T,
    ) -> Result<ethers_core::types::Signature, Self::Error> {
        let digest = payload.encode_eip712().map_err(|e| Self::Error::Other {
            message: format!("failed encode_eip712 {}", e),
            is_retryable: false,
        })?;

        let sig = self.inner.sign_digest(digest.as_ref()).await?;
        let sig = key::secp256k1::signature::rsig_to_ethsig(&sig);
        Ok(sig)
    }

    fn address(&self) -> ethers_core::types::Address {
        self.address
    }

    fn chain_id(&self) -> u64 {
        self.chain_id.as_u64()
    }

    fn with_chain_id<T: Into<u64>>(mut self, chain_id: T) -> Self {
        let chain_id: u64 = chain_id.into();
        self.chain_id = primitive_types::U256::from(chain_id);
        self
    }
}
