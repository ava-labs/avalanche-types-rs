pub mod eth_signer;

use std::{
    collections::HashMap,
    io::{self, Error, ErrorKind},
};

use crate::{hash, ids::short, key};
use async_trait::async_trait;
use aws_manager::kms;
use aws_sdk_kms::model::{KeySpec, KeyUsageType};
use ethers_core::k256::ecdsa::recoverable::Signature as RSig;
use tokio::time::{sleep, Duration, Instant};

/// Represents AWS KMS asymmetric elliptic curve key pair ECC_SECG_P256K1.
/// Note that the actual private key never leaves KMS.
/// Private key signing operation must be done via AWS KMS API.
/// ref. <https://docs.aws.amazon.com/kms/latest/APIReference/API_CreateKey.html>
#[derive(Debug, Clone)]
pub struct Cmk {
    /// AWS KMS API wrapper.
    pub kms_manager: kms::Manager,

    /// CMK Id.
    pub id: String,
    /// CMK Arn.
    pub arn: String,

    /// Public key.
    pub public_key: key::secp256k1::public_key::Key,

    /// Total duration for retries.
    pub retry_timeout: Duration,
    /// Interval between retries.
    pub retry_interval: Duration,
}

impl Cmk {
    /// Generates a new CMK.
    pub async fn create(
        kms_manager: kms::Manager,
        tags: HashMap<String, String>,
    ) -> io::Result<Self> {
        let cmk = kms_manager
            .create_key(KeySpec::EccSecgP256K1, KeyUsageType::SignVerify, Some(tags))
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed kms.create_key {}", e)))?;

        Self::from_arn(kms_manager, &cmk.arn).await
    }

    /// Loads the Cmk from its Arn or Id.
    pub async fn from_arn(kms_manager: kms::Manager, arn: &str) -> io::Result<Self> {
        let (id, _desc) = kms_manager.describe_key(arn).await.map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!(
                    "failed kms.describe_key {} (retryable {})",
                    e.message(),
                    e.is_retryable()
                ),
            )
        })?;
        log::info!("described key Id '{id}' from '{arn}'");

        // derives the public key from its private key
        let pubkey = kms_manager.get_public_key(arn).await.map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!(
                    "failed kms.get_public_key {} (retryable {})",
                    e.message(),
                    e.is_retryable()
                ),
            )
        })?;

        if let Some(blob) = pubkey.public_key() {
            // same as "key::secp256k1::public_key::Key::from_public_key_der(blob.as_ref())"
            // ref. <https://github.com/gakonst/ethers-rs/tree/master/ethers-signers/src/aws>
            let verifying_key =
                key::secp256k1::public_key::load_ecdsa_verifying_key_from_public_key(
                    blob.as_ref(),
                )?;
            let public_key = key::secp256k1::public_key::Key::from_verifying_key(&verifying_key);
            log::info!(
                "fetched CMK public key with ETH address '{}'",
                public_key.to_eth_address(),
            );

            return Ok(Self {
                kms_manager,
                public_key,
                id,
                arn: arn.to_string(),
                retry_timeout: Duration::from_secs(90),
                retry_interval: Duration::from_secs(10),
            });
        }

        return Err(Error::new(ErrorKind::Other, "public key not found"));
    }

    /// Schedules to delete the KMS CMK.
    pub async fn delete(&self, pending_window_in_days: i32) -> io::Result<()> {
        self.kms_manager
            .schedule_to_delete(&self.arn, pending_window_in_days)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed schedule_to_delete {}", e)))
    }

    pub fn to_public_key(&self) -> key::secp256k1::public_key::Key {
        self.public_key
    }

    /// Converts to Info.
    pub fn to_info(&self, network_id: u32) -> io::Result<key::secp256k1::Info> {
        let short_addr = self.public_key.to_short_id()?;
        let eth_addr = self.public_key.to_eth_address();
        let h160_addr = self.public_key.to_h160();

        let mut addresses = HashMap::new();
        addresses.insert(
            network_id,
            key::secp256k1::ChainAddresses {
                x: self.public_key.to_hrp_address(network_id, "X")?,
                p: self.public_key.to_hrp_address(network_id, "P")?,
            },
        );

        Ok(key::secp256k1::Info {
            id: Some(self.arn.clone()),
            key_type: key::secp256k1::KeyType::AwsKms,

            addresses,

            short_address: short_addr,
            eth_address: eth_addr,
            h160_address: h160_addr,

            ..Default::default()
        })
    }

    pub async fn sign_digest(&self, digest: &[u8]) -> Result<RSig, aws_manager::errors::Error> {
        // ref. "crypto/sha256.Size"
        assert_eq!(digest.len(), hash::SHA256_OUTPUT_LEN);

        let (start, mut success) = (Instant::now(), false);
        let mut round = 0_u32;

        // DER-encoded >65-byte signature, need convert to 65-byte recoverable signature
        // ref. <https://docs.aws.amazon.com/kms/latest/APIReference/API_Sign.html#KMS-Sign-response-Signature>
        let mut raw_der = Vec::new();
        loop {
            round = round + 1;
            let elapsed = start.elapsed();
            if elapsed.gt(&self.retry_timeout) {
                break;
            }

            raw_der = match self
                .kms_manager
                .sign_digest_secp256k1_ecdsa_sha256(&self.id, digest)
                .await
            {
                Ok(raw) => {
                    success = true;
                    raw
                }
                Err(aerr) => {
                    log::warn!(
                        "[round {round}] failed sign {} (retriable {})",
                        aerr,
                        aerr.is_retryable()
                    );
                    if !aerr.is_retryable() {
                        return Err(aerr);
                    }

                    sleep(self.retry_interval).await;
                    continue;
                }
            };
            break;
        }
        if !success {
            return Err(aws_manager::errors::Error::API {
                message: "failed sign after retries".to_string(),
                is_retryable: false,
            });
        }

        let sig = key::secp256k1::signature::decode_signature(&raw_der).map_err(|e| {
            aws_manager::errors::Error::Other {
                message: format!("failed decode_signature {}", e),
                is_retryable: false,
            }
        })?;

        let mut fixed_digest = [0u8; hash::SHA256_OUTPUT_LEN];
        fixed_digest.copy_from_slice(digest);
        Ok(
            key::secp256k1::signature::rsig_from_digest_bytes_trial_recovery(
                &sig,
                fixed_digest,
                &self.public_key.to_verifying_key(),
            ),
        )
    }
}

#[async_trait]
impl key::secp256k1::SignOnly for Cmk {
    type Error = aws_manager::errors::Error;

    fn signing_key(&self) -> io::Result<k256::ecdsa::SigningKey> {
        Err(Error::new(ErrorKind::Other, "not implemented"))
    }

    async fn sign_digest(&self, msg: &[u8]) -> Result<[u8; 65], aws_manager::errors::Error> {
        let sig = self.sign_digest(msg).await?;

        let mut b = [0u8; key::secp256k1::signature::LEN];
        b.copy_from_slice(&sig.as_ref());

        Ok(b)
    }
}

/// ref. <https://doc.rust-lang.org/book/ch10-02-traits.html>
impl key::secp256k1::ReadOnly for Cmk {
    fn key_type(&self) -> key::secp256k1::KeyType {
        key::secp256k1::KeyType::AwsKms
    }

    fn hrp_address(&self, network_id: u32, chain_id_alias: &str) -> io::Result<String> {
        self.to_public_key()
            .to_hrp_address(network_id, chain_id_alias)
    }

    fn short_address(&self) -> io::Result<short::Id> {
        self.to_public_key().to_short_id()
    }

    fn short_address_bytes(&self) -> io::Result<Vec<u8>> {
        self.to_public_key().to_short_bytes()
    }

    fn eth_address(&self) -> String {
        self.to_public_key().to_eth_address()
    }

    fn h160_address(&self) -> primitive_types::H160 {
        self.to_public_key().to_h160()
    }
}
