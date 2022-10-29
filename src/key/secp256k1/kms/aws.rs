use std::io::{self, Error, ErrorKind};

use crate::key;
use async_trait::async_trait;
use aws_manager::kms;
use aws_sdk_kms::model::{KeySpec, KeyUsageType};

/// Represents AWS KMS asymmetric elliptic curve key pair ECC_SECG_P256K1.
/// Note that the actual private key never leaves KMS.
/// Private key signing operation must be done via AWS KMS API.
/// ref. https://docs.aws.amazon.com/kms/latest/APIReference/API_CreateKey.html
#[derive(Debug, Clone)]
pub struct PrivateKey {
    /// AWS KMS API wrapper.
    pub kms_manager: kms::Manager,

    /// CMK Id.
    pub id: String,
    /// CMK Arn.
    pub arn: String,

    /// Public key.
    pub public_key: key::secp256k1::public_key::Key,
}

impl PrivateKey {
    /// Generates a private key from random bytes.
    pub async fn create(kms_manager: kms::Manager, name: &str) -> io::Result<Self> {
        let cmk = kms_manager
            .create_key(name, KeySpec::EccSecgP256K1, KeyUsageType::SignVerify)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed kms.create_key {}", e)))?;

        // derives the public key from its private key
        let pubkey = kms_manager
            .client()
            .get_public_key()
            .key_id(&cmk.id)
            .send()
            .await
            .map_err(|e| {
                Error::new(ErrorKind::Other, format!("failed kms.get_public_key {}", e))
            })?;

        if let Some(blob) = pubkey.public_key() {
            let public_key = key::secp256k1::public_key::Key::from_public_key_der(blob.as_ref())?;
            log::info!("created key with ETH address {}", public_key.eth_address());

            return Ok(Self {
                kms_manager,
                public_key,
                id: cmk.id,
                arn: cmk.arn,
            });
        }

        return Err(Error::new(ErrorKind::Other, "public key blob not found"));
    }

    pub async fn sign_digest(&self, digest: &[u8]) -> io::Result<key::secp256k1::signature::Sig> {
        // ref. "crypto/sha256.Size"
        assert_eq!(digest.len(), ring::digest::SHA256_OUTPUT_LEN);

        // DER-encoded >65-byte signature, need convert to 65-byte recoverable signature
        // ref. https://docs.aws.amazon.com/kms/latest/APIReference/API_Sign.html
        // ref. https://docs.aws.amazon.com/kms/latest/APIReference/API_Sign.html#KMS-Sign-response-Signature
        let raw = self
            .kms_manager
            .secp256k1_sign_digest(&self.id, digest)
            .await
            .map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!(
                        "failed secp256k1_sign_digest '{}' (retriable {})",
                        e.message(),
                        e.is_retryable()
                    ),
                )
            })?;

        // converts to recoverable signature of 65-byte
        key::secp256k1::signature::Sig::from_der(&raw, digest, &self.public_key.into())
    }

    /// Schedules to delete the KMS CMK, with 7-day grace period.
    pub async fn delete(&self) -> io::Result<()> {
        self.kms_manager
            .schedule_to_delete(&self.id)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed schedule_to_delete {}", e)))
    }
}

#[async_trait]
impl key::secp256k1::SignOnly for PrivateKey {
    fn signing_key(&self) -> io::Result<k256::ecdsa::SigningKey> {
        Err(Error::new(ErrorKind::Other, "not implemented"))
    }

    async fn sign_digest(&self, msg: &[u8]) -> io::Result<[u8; 65]> {
        let sig = self.sign_digest(msg).await?;
        Ok(sig.to_bytes())
    }
}
