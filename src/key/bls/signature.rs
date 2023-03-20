use std::io::{self, Error, ErrorKind};

use crate::key::bls::{self, public_key::Key as PublicKey};
use blst::{
    min_pk::{AggregateSignature, Signature},
    BLST_ERROR,
};

#[derive(Debug, Clone)]
pub struct Sig(pub Signature);

pub const LEN: usize = 96;

impl Sig {
    /// Converts the public key to compressed bytes.
    /// ref. "avalanchego/utils/crypto/bls.SignatureToBytes"
    pub fn to_compressed_bytes(&self) -> [u8; LEN] {
        self.0.compress()
    }

    /// Loads the signature from the compressed raw scalar bytes (in big endian).
    pub fn from_bytes(compressed: &[u8]) -> io::Result<Self> {
        let sig = Signature::uncompress(compressed).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed blst::min_pk::Signature::uncompress {:?}", e),
            )
        })?;
        sig.validate(false).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed blst::min_pk::Signature::validate {:?}", e),
            )
        })?;

        Ok(Self(sig))
    }

    /// Verifies the message and the validity of its signature.
    /// Invariant: [pubkey] and [self.0] have both been validated.
    /// ref. "avalanchego/utils/crypto/bls.Verify"
    pub fn verify(&self, msg: &[u8], pubkey: &PublicKey) -> bool {
        self.0.verify(
            false,
            msg,
            &bls::private_key::CIPHER_SUITE_SIGNATURE,
            &[],
            &pubkey.0,
            false,
        ) == BLST_ERROR::BLST_SUCCESS
    }

    /// Verifies the message and the validity of its signature.
    /// Invariant: [pubkey] and [self.0] have both been validated.
    /// ref. "avalanchego/utils/crypto/bls.VerifyProofOfPossession"
    pub fn verify_proof_of_possession(&self, msg: &[u8], pubkey: &PublicKey) -> bool {
        self.0.verify(
            false,
            msg,
            &bls::private_key::CIPHER_SUITE_PROOF_OF_POSSESSION,
            &[],
            &pubkey.0,
            false,
        ) == BLST_ERROR::BLST_SUCCESS
    }
}

impl From<Signature> for Sig {
    fn from(s: Signature) -> Self {
        Self(s)
    }
}

impl From<Sig> for Signature {
    fn from(s: Sig) -> Self {
        s.0
    }
}

pub fn aggregate(sigs: &[Sig]) -> io::Result<Sig> {
    let mut ss = Vec::with_capacity(sigs.len());
    for s in sigs.iter() {
        ss.push(&s.0);
    }

    let agg_sig = AggregateSignature::aggregate(&ss, false).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("failed AggregateSignature::aggregate {:?}", e),
        )
    })?;
    Ok(Sig(agg_sig.to_signature()))
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- key::bls::signature::test_signature --exact --show-output
#[test]
fn test_signature() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init();

    let sk = crate::key::bls::private_key::Key::generate().unwrap();
    let pubkey = sk.to_public_key();

    let msg = random_manager::secure_bytes(50).unwrap();
    let sig = sk.sign(&msg);
    let sig_bytes = sig.to_compressed_bytes();

    assert!(sig.verify(&msg, &pubkey));
    assert!(!sig.verify_proof_of_possession(&msg, &pubkey));

    let sig_pos = sk.sign_proof_of_possession(&msg);
    assert!(!sig_pos.verify(&msg, &pubkey));
    assert!(sig_pos.verify_proof_of_possession(&msg, &pubkey));

    let agg_sig = aggregate(&[sig]).unwrap();
    let agg_sig_bytes = agg_sig.to_compressed_bytes();

    assert_eq!(sig_bytes, agg_sig_bytes);
}
