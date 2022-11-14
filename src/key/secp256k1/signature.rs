use std::io::{self, Error, ErrorKind};

use hmac::digest::generic_array::GenericArray;

/// The length of recoverable ECDSA signature.
/// "github.com/decred/dcrd/dcrec/secp256k1/v3/ecdsa.SignCompact" outputs
/// 65-byte signature -- see "compactSigSize"
/// ref. "avalanchego/utils/crypto.PrivateKeySECP256K1R.SignHash"
/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/utils/crypto#SECP256K1RSigLen
/// ref. "secp256k1::constants::SCHNORR_SIGNATURE_SIZE" + 1
pub const LEN: usize = 65;

/// Represents Ethereum-style "recoverable signatures".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sig(pub k256::ecdsa::recoverable::Signature);

impl Sig {
    /// Loads the recoverable signature from the bytes.
    pub fn from_bytes(b: &[u8]) -> io::Result<Self> {
        if b.len() != LEN {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "invalid signature length",
            ));
        }

        let sig = k256::ecdsa::recoverable::Signature::try_from(b).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed to load recoverable signature {}", e),
            )
        })?;
        Ok(Self(sig))
    }

    /// Loads the recoverable signature from the DER-encoded bytes,
    /// as defined by ANS X9.62–2005 and RFC 3279 Section 2.2.3.
    /// ref. https://docs.aws.amazon.com/kms/latest/APIReference/API_Sign.html#KMS-Sign-response-Signature
    pub fn from_der(
        raw_sig: &[u8],
        digest: &[u8],
        vkey: &k256::ecdsa::VerifyingKey,
    ) -> io::Result<Self> {
        // decode DER-encoded signature to "k256::ecdsa::Signature" object
        let sig = k256::ecdsa::Signature::from_der(raw_sig).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed to load recoverable signature from DER ({})", e),
            )
        })?;
        let normalized_sig = sig.normalize_s().unwrap_or(sig);

        let rsig = rsig_from_normalized_sig_and_digest_bytes(&normalized_sig, digest, vkey)?;
        return Ok(Self::from(rsig));
    }

    /// Converts the signature to bytes.
    pub fn to_bytes(&self) -> [u8; LEN] {
        let mut b = [0u8; LEN];
        b.copy_from_slice(self.0.as_ref());
        b
    }

    /// Recovers the public key from the 32-byte SHA256 output message using its signature.
    pub fn recover_public_key(
        &self,
        digest: &[u8],
    ) -> io::Result<(
        crate::key::secp256k1::public_key::Key,
        k256::ecdsa::VerifyingKey,
    )> {
        recover_pubkeys(&self.0, digest)
    }

    pub fn r(&self) -> primitive_types::U256 {
        let b = self.0.as_ref();
        primitive_types::U256::from_big_endian(&b[0..32])
    }

    pub fn s(&self) -> primitive_types::U256 {
        let b = self.0.as_ref();
        primitive_types::U256::from_big_endian(&b[32..64])
    }

    /// Returns the recovery Id.
    pub fn v(&self) -> u64 {
        let v: u8 = self.0.recovery_id().into();
        v as u64
    }
}

fn recover_pubkeys(
    rsig: &k256::ecdsa::recoverable::Signature,
    digest: &[u8],
) -> io::Result<(
    crate::key::secp256k1::public_key::Key,
    k256::ecdsa::VerifyingKey,
)> {
    let prehash = GenericArray::clone_from_slice(&digest[..]);
    let vkey = rsig
        .recover_verifying_key_from_digest_bytes(&prehash)
        .map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed recover_verifying_key_from_digest_bytes {}", e),
            )
        })?;

    // [optional]
    // use ecdsa::signature::hazmat::PrehashVerifier;
    // assert!(vkey.verify_prehash(digest, &self.0).is_ok());

    Ok((vkey.into(), vkey))
}

/// Checks whether the specified recoverable signature can derive
/// the expected verifying key from the digest bytes.
fn check_recoverable_signature(
    rsig: &k256::ecdsa::recoverable::Signature,
    digest: &[u8],
    vkey: &k256::ecdsa::VerifyingKey,
) -> bool {
    match recover_pubkeys(rsig, digest) {
        Ok(rs) => {
            let recovered_vkey = rs.1;
            recovered_vkey == *vkey
        }
        Err(e) => {
            log::debug!("failed recover_pubkeys {}", e);
            false
        }
    }
}

/// ref. "ethers-signers::aws::util::rsig_from_digest_bytes_trial_recovery"
pub fn rsig_from_normalized_sig_and_digest_bytes(
    normalized_sig: &k256::ecdsa::Signature,
    digest: &[u8],
    vkey: &k256::ecdsa::VerifyingKey,
) -> io::Result<k256::ecdsa::recoverable::Signature> {
    let rid0 = k256::ecdsa::recoverable::Id::new(0).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed to create recoverable::Id 0 {}", e),
        )
    })?;
    let rsig0 = k256::ecdsa::recoverable::Signature::new(normalized_sig, rid0).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed to create recoverable::Signature 0 {}", e),
        )
    })?;

    if check_recoverable_signature(&rsig0, digest, vkey) {
        return Ok(rsig0);
    }

    let rid1 = k256::ecdsa::recoverable::Id::new(1).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed to create recoverable::Id 1 {}", e),
        )
    })?;
    let rsig1 = k256::ecdsa::recoverable::Signature::new(normalized_sig, rid1).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed to create recoverable::Signature 1 {}", e),
        )
    })?;

    if check_recoverable_signature(&rsig1, digest, vkey) {
        return Ok(rsig1);
    }

    return Err(Error::new(
        ErrorKind::Other,
        "failed to recover recoverable signature",
    ));
}

impl From<k256::ecdsa::recoverable::Signature> for Sig {
    fn from(sig: k256::ecdsa::recoverable::Signature) -> Self {
        Self(sig)
    }
}

impl From<Sig> for k256::ecdsa::recoverable::Signature {
    fn from(sig: Sig) -> Self {
        sig.0
    }
}

impl From<Sig> for [u8; LEN] {
    fn from(sig: Sig) -> Self {
        sig.to_bytes()
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- key::secp256k1::signature::test_signature --exact --show-output
#[test]
fn test_signature() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init();

    let pk = crate::key::secp256k1::private_key::Key::generate().unwrap();
    let pubkey = pk.to_public_key();

    let msg: Vec<u8> = random_manager::bytes(100).unwrap();
    let hashed = crate::hash::sha256(&msg);

    let sig = pk.sign_digest(&hashed).unwrap();
    assert_eq!(sig.to_bytes().len(), crate::key::secp256k1::signature::LEN);

    let (recovered_pubkey, _) = sig.recover_public_key(&hashed).unwrap();
    assert_eq!(pubkey.eth_address(), recovered_pubkey.eth_address());
    assert_eq!(pubkey, recovered_pubkey);
}
