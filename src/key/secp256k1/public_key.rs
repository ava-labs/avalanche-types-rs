use std::io::{self, Error, ErrorKind};

use crate::{
    constants, formatting, hash,
    ids::short,
    key::{
        self,
        secp256k1::{address, signature::Sig},
    },
};
use ecdsa::signature::hazmat::PrehashVerifier;
use k256::{elliptic_curve::sec1::ToEncodedPoint, pkcs8::DecodePublicKey};

/// The size (in bytes) of a public key.
/// ref. "secp256k1::constants::PUBLIC_KEY_SIZE"
pub const LEN: usize = 33;

/// The size (in bytes) of an serialized uncompressed public key.
/// ref. "secp256k1::constants::UNCOMPRESSED_PUBLIC_KEY_SIZE"
pub const UNCOMPRESSED_LEN: usize = 65;

/// Represents "k256::PublicKey" and "k256::ecdsa::VerifyingKey".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Key(pub k256::PublicKey);

impl Key {
    /// Decodes compressed or uncompressed public key bytes with Elliptic-Curve-Point-to-Octet-String
    /// encoding described in SEC 1: Elliptic Curve Cryptography (Version 2.0) section 2.3.3 (page 10).
    /// ref. <http://www.secg.org/sec1-v2.pdf>
    pub fn from_sec1_bytes(b: &[u8]) -> io::Result<Self> {
        let pubkey = k256::PublicKey::from_sec1_bytes(b).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed k256::PublicKey::from_sec1_bytes {}", e),
            )
        })?;
        Ok(Self(pubkey))
    }

    /// Decodes ASN.1 DER-encoded public key bytes.
    pub fn from_public_key_der(b: &[u8]) -> io::Result<Self> {
        let pubkey = k256::PublicKey::from_public_key_der(b).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed k256::PublicKey::from_public_key_der {}", e),
            )
        })?;
        Ok(Self(pubkey))
    }

    /// Loads the public key from a message and its recoverable signature.
    /// ref. "fx.SECPFactory.RecoverHashPublicKey"
    pub fn from_signature(digest: &[u8], sig: &[u8]) -> io::Result<Self> {
        let sig = Sig::from_bytes(sig)?;
        let (pubkey, _) = sig.recover_public_key(digest)?;
        Ok(pubkey)
    }

    pub fn to_verifying_key(&self) -> k256::ecdsa::VerifyingKey {
        self.0.into()
    }

    /// Verifies the message and the validity of its signature with recoverable code.
    pub fn verify(&self, digest: &[u8], sig: &[u8]) -> io::Result<bool> {
        let sig = Sig::from_bytes(sig)?;

        let (recovered_pubkey, verifying_key) = sig.recover_public_key(digest)?;
        let rsig = k256::ecdsa::recoverable::Signature::from(sig);
        if verifying_key.verify_prehash(digest, &rsig).is_err() {
            return Ok(false);
        }

        Ok(*self == recovered_pubkey)
    }

    /// Converts the public key to compressed bytes.
    pub fn to_compressed_bytes(&self) -> [u8; LEN] {
        let vkey: k256::ecdsa::VerifyingKey = self.0.into();
        let bb = vkey.to_bytes();

        let mut b = [0u8; LEN];
        b.copy_from_slice(&bb);
        b
    }

    /// Converts the public key to uncompressed bytes.
    pub fn to_uncompressed_bytes(&self) -> [u8; UNCOMPRESSED_LEN] {
        let vkey: k256::ecdsa::VerifyingKey = self.0.into();
        let p = vkey.to_encoded_point(false);

        let mut b = [0u8; UNCOMPRESSED_LEN];
        b.copy_from_slice(&p.to_bytes());
        b
    }

    /// "hashing.PubkeyBytesToAddress"
    ///
    /// ref. "pk.PublicKey().Address().Bytes()"
    ///
    /// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/utils/hashing#PubkeyBytesToAddress>
    pub fn to_short_id(&self) -> io::Result<crate::ids::short::Id> {
        let compressed = self.to_compressed_bytes();
        short::Id::from_public_key_bytes(&compressed)
    }

    /// "hashing.PubkeyBytesToAddress" and "ids.ToShortID"
    ///
    /// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/utils/hashing#PubkeyBytesToAddress>
    pub fn to_short_bytes(&self) -> io::Result<Vec<u8>> {
        let compressed = self.to_compressed_bytes();
        hash::sha256_ripemd160(&compressed)
    }

    pub fn to_h160(&self) -> primitive_types::H160 {
        let uncompressed = self.to_uncompressed_bytes();

        // ref. "Keccak256(pubBytes[1:])[12:]"
        let digest_h256 = hash::keccak256(&uncompressed[1..]);
        let digest_h256 = &digest_h256.0[12..];

        primitive_types::H160::from_slice(digest_h256)
    }

    pub fn hrp_address(&self, network_id: u32, chain_id_alias: &str) -> io::Result<String> {
        let hrp = match constants::NETWORK_ID_TO_HRP.get(&network_id) {
            Some(v) => v,
            None => constants::FALLBACK_HRP,
        };

        // ref. "pk.PublicKey().Address().Bytes()"
        let short_address_bytes = self.to_short_bytes()?;

        // ref. "formatting.FormatAddress(chainIDAlias, hrp, pubBytes)"
        formatting::address(chain_id_alias, hrp, &short_address_bytes)
    }

    /// Encodes the public key in ETH address format.
    ///
    /// ref. <https://pkg.go.dev/github.com/ethereum/go-ethereum/crypto#PubkeyToAddress>
    ///
    /// ref. <https://pkg.go.dev/github.com/ethereum/go-ethereum/common#Address.Hex>
    pub fn eth_address(&self) -> String {
        address::h160_to_eth_address(self.to_h160())
    }
}

impl From<k256::PublicKey> for Key {
    fn from(pubkey: k256::PublicKey) -> Self {
        Self(pubkey)
    }
}

impl From<Key> for k256::PublicKey {
    fn from(pubkey: Key) -> Self {
        pubkey.0
    }
}

impl From<k256::ecdsa::VerifyingKey> for Key {
    fn from(vkey: k256::ecdsa::VerifyingKey) -> Self {
        Self(vkey.into())
    }
}

impl From<Key> for k256::ecdsa::VerifyingKey {
    fn from(pubkey: Key) -> Self {
        pubkey.0.into()
    }
}

/// ref. <https://doc.rust-lang.org/std/string/trait.ToString.html>
///
/// ref. <https://doc.rust-lang.org/std/fmt/trait.Display.html>
///
/// Use "Self.to_string()" to directly invoke this
impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.to_compressed_bytes()))
    }
}

/// ref. <https://doc.rust-lang.org/book/ch10-02-traits.html>
impl key::secp256k1::ReadOnly for Key {
    fn key_type(&self) -> key::secp256k1::KeyType {
        key::secp256k1::KeyType::Hot
    }

    fn hrp_address(&self, network_id: u32, chain_id_alias: &str) -> io::Result<String> {
        self.hrp_address(network_id, chain_id_alias)
    }

    fn short_address(&self) -> io::Result<short::Id> {
        self.to_short_id()
    }

    fn short_address_bytes(&self) -> io::Result<Vec<u8>> {
        self.to_short_bytes()
    }

    fn eth_address(&self) -> String {
        self.eth_address()
    }

    fn h160_address(&self) -> primitive_types::H160 {
        self.to_h160()
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- key::secp256k1::public_key::test_public_key --exact --show-output
#[test]
fn test_public_key() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init();

    let pk1 = crate::key::secp256k1::private_key::Key::generate().unwrap();
    let pubkey1 = pk1.to_public_key();

    let b = pubkey1.to_compressed_bytes();
    let pubkey2 = Key::from_sec1_bytes(&b).unwrap();

    let b = pubkey1.to_uncompressed_bytes();
    let pubkey3 = Key::from_sec1_bytes(&b).unwrap();

    assert_eq!(pubkey1, pubkey2);
    assert_eq!(pubkey2, pubkey3);

    let msg: Vec<u8> = random_manager::bytes(100).unwrap();
    let hashed = hash::sha256(&msg);

    let sig1 = pk1.sign_digest(&hashed).unwrap();
    assert_eq!(sig1.to_bytes().len(), crate::key::secp256k1::signature::LEN);

    let pubkey4 = Key::from_signature(&hashed, &sig1.to_bytes()).unwrap();
    assert_eq!(pubkey3, pubkey4);

    assert!(pubkey1.verify(&hashed, &sig1.to_bytes()).unwrap());
    assert!(pubkey2.verify(&hashed, &sig1.to_bytes()).unwrap());
    assert!(pubkey3.verify(&hashed, &sig1.to_bytes()).unwrap());
    assert!(pubkey4.verify(&hashed, &sig1.to_bytes()).unwrap());

    log::info!("public key: {}", pubkey1);
    log::info!("to_short_id: {}", pubkey1.to_short_id().unwrap());
    log::info!("to_h160: {}", pubkey1.to_h160());
    log::info!("eth_address: {}", pubkey1.eth_address());

    let x_avax_addr = pubkey1.hrp_address(1, "X").unwrap();
    let p_avax_addr = pubkey1.hrp_address(1, "P").unwrap();
    let c_avax_addr = pubkey1.hrp_address(1, "C").unwrap();
    log::info!("AVAX X address: {}", x_avax_addr);
    log::info!("AVAX P address: {}", p_avax_addr);
    log::info!("AVAX C address: {}", c_avax_addr);
}
