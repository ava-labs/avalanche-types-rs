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
        let desc = kms_manager
            .client()
            .describe_key()
            .key_id(arn)
            .send()
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed kms.describe_key {}", e)))?;
        let id = desc.key_metadata().unwrap().key_id().unwrap().to_string();
        log::info!("described key Id '{id}' from '{arn}'");

        // derives the public key from its private key
        let pubkey = kms_manager
            .client()
            .get_public_key()
            .key_id(arn) // or use Cmk Id
            .send()
            .await
            .map_err(|e| {
                Error::new(ErrorKind::Other, format!("failed kms.get_public_key {}", e))
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
        let x_address = self.public_key.to_hrp_address(network_id, "X")?;
        let p_address = self.public_key.to_hrp_address(network_id, "P")?;
        let c_address = self.public_key.to_hrp_address(network_id, "C")?;
        addresses.insert(
            network_id,
            key::secp256k1::ChainAddresses {
                x_address,
                p_address,
                c_address,
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

        // DER-encoded >65-byte signature, need convert to 65-byte recoverable signature
        // ref. <https://docs.aws.amazon.com/kms/latest/APIReference/API_Sign.html#KMS-Sign-response-Signature>
        let raw_der = self
            .kms_manager
            .sign_digest_secp256k1_ecdsa_sha256(&self.id, digest)
            .await?;

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
