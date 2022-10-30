use std::io;

use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct Signer {
    pub inner: super::Signer,
    pub chain_id: primitive_types::U256,
    pub address: ethers_core::types::Address,
}

impl Signer {
    pub fn new(inner: super::Signer, chain_id: primitive_types::U256) -> io::Result<Self> {
        let short_bytes = inner.public_key().to_short_bytes()?;
        let address = ethers_core::types::Address::from_slice(&short_bytes);
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

        let mut sig = rsig_to_ethsig(&sig.into());
        apply_eip155(&mut sig, chain_id);

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

        let sighash = tx.sighash();
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
        let sig = rsig_to_ethsig(&sig.into());

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

/// Converts a recoverable signature to an ethers signature
/// ref. "ethers-signers::aws::utils::rsig_to_ethsig"
fn rsig_to_ethsig(
    sig: &ethers_core::k256::ecdsa::recoverable::Signature,
) -> ethers_core::types::Signature {
    let v: u8 = sig.recovery_id().into();
    let v = (v + 27) as u64;
    let r_bytes: ethers_core::k256::FieldBytes = sig.r().into();
    let s_bytes: ethers_core::k256::FieldBytes = sig.s().into();
    let r = ethers_core::types::U256::from_big_endian(r_bytes.as_slice());
    let s = ethers_core::types::U256::from_big_endian(s_bytes.as_slice());
    ethers_core::types::Signature { r, s, v }
}

/// Modify the v value of a signature to conform to eip155
/// ref. "ethers-signers::aws::utils::apply_eip155"
fn apply_eip155(sig: &mut ethers_core::types::Signature, chain_id: u64) {
    let v = (chain_id * 2 + 35) + ((sig.v - 1) % 2);
    sig.v = v;
}
