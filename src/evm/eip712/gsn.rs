use std::{
    collections::BTreeMap,
    convert::TryFrom,
    io::{self, Error, ErrorKind},
};

use crate::codec::serde::hex_0x_bytes::Hex0xBytes;
use ethers_core::types::{
    transaction::eip712::{EIP712Domain, Eip712, Eip712DomainType, TypedData, Types},
    RecoveryMessage, Signature, H160, H256, U256,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use zerocopy::AsBytes;

/// ref. <https://eips.ethereum.org/EIPS/eip-712>
/// ref. <https://eips.ethereum.org/EIPS/eip-2770>
/// ref. <https://github.com/opengsn/gsn/blob/master/packages/contracts/src/forwarder/IForwarder.sol>
pub struct RelayTransactionRequestBuilder {
    pub domain_name: String,
    pub domain_version: String,
    pub domain_chain_id: U256,
    pub domain_verifying_contract: H160,
    pub from: H160,
    pub to: H160,
    pub value: U256,
    pub gas: U256,
    pub nonce: U256,
    pub data: Vec<u8>,
    pub valid_until_time: U256,
}

impl RelayTransactionRequestBuilder {
    pub fn new() -> Self {
        Self {
            domain_name: String::new(),
            domain_version: String::new(),
            domain_chain_id: U256::zero(),
            domain_verifying_contract: H160::zero(),
            from: H160::zero(),
            to: H160::zero(),
            value: U256::zero(),
            gas: U256::zero(),
            nonce: U256::zero(),
            data: Vec::new(),
            valid_until_time: U256::zero(),
        }
    }

    #[must_use]
    pub fn domain_name(mut self, domain_name: impl Into<String>) -> Self {
        self.domain_name = domain_name.into();
        self
    }

    #[must_use]
    pub fn domain_version(mut self, domain_version: impl Into<String>) -> Self {
        self.domain_version = domain_version.into();
        self
    }

    #[must_use]
    pub fn domain_chain_id(mut self, domain_chain_id: impl Into<U256>) -> Self {
        self.domain_chain_id = domain_chain_id.into();
        self
    }

    #[must_use]
    pub fn domain_verifying_contract(mut self, domain_verifying_contract: impl Into<H160>) -> Self {
        self.domain_verifying_contract = domain_verifying_contract.into();
        self
    }

    #[must_use]
    pub fn from(mut self, from: impl Into<H160>) -> Self {
        self.from = from.into();
        self
    }

    #[must_use]
    pub fn to(mut self, to: impl Into<H160>) -> Self {
        self.to = to.into();
        self
    }

    #[must_use]
    pub fn value(mut self, value: impl Into<U256>) -> Self {
        self.value = value.into();
        self
    }

    #[must_use]
    pub fn gas(mut self, gas: impl Into<U256>) -> Self {
        self.gas = gas.into();
        self
    }

    #[must_use]
    pub fn nonce(mut self, nonce: impl Into<U256>) -> Self {
        self.nonce = nonce.into();
        self
    }

    #[must_use]
    pub fn data(mut self, data: impl Into<Vec<u8>>) -> Self {
        self.data = data.into();
        self
    }

    #[must_use]
    pub fn valid_until_time(mut self, valid_until_time: impl Into<U256>) -> Self {
        self.valid_until_time = valid_until_time.into();
        self
    }

    pub fn build_typed_data(&self) -> TypedData {
        let mut message = BTreeMap::new();
        message.insert(
            String::from("from"),
            serde_json::to_value(self.from).unwrap(),
        );
        message.insert(String::from("to"), serde_json::to_value(self.to).unwrap());
        message.insert(
            String::from("value"),
            serde_json::to_value(self.value).unwrap(),
        );
        message.insert(String::from("gas"), serde_json::to_value(self.gas).unwrap());
        message.insert(
            String::from("nonce"),
            serde_json::to_value(self.nonce).unwrap(),
        );
        message.insert(
            String::from("data"),
            serde_json::to_value(hex::encode(&self.data)).unwrap(),
        );
        message.insert(
            String::from("validUntilTime"),
            serde_json::to_value(self.valid_until_time).unwrap(),
        );

        TypedData {
            domain: EIP712Domain {
                name: Some(self.domain_name.clone()),
                version: Some(self.domain_version.clone()),
                chain_id: Some(self.domain_chain_id),
                verifying_contract: Some(self.domain_verifying_contract),
                salt: None,
            },
            types: foward_request_types(),
            primary_type: "Message".to_string(),
            message,
        }
    }

    /// Builds and signs the typed data with the signer and returns the
    /// "RelayTransactionRequest" with the signature attached in the relay metadata.
    /// Use "serde_json::to_vec" to encode to "ethers_core::types::Bytes"
    /// and send the request via "eth_sendRawTransaction".
    pub async fn build_and_sign(
        &self,
        eth_signer: impl ethers_signers::Signer + Clone,
    ) -> io::Result<RelayTransactionRequest> {
        let forward_request = self.build_typed_data();
        RelayTransactionRequest::sign(forward_request, eth_signer).await
    }
}

/// ref. <https://eips.ethereum.org/EIPS/eip-2770>
/// ref. <https://github.com/opengsn/gsn/blob/master/packages/contracts/src/forwarder/IForwarder.sol>
pub fn foward_request_types() -> Types {
    let mut types = BTreeMap::new();
    types.insert(
        "EIP712Domain".to_string(),
        vec![
            Eip712DomainType {
                name: String::from("name"),
                r#type: String::from("string"),
            },
            Eip712DomainType {
                name: String::from("version"),
                r#type: String::from("string"),
            },
            Eip712DomainType {
                name: String::from("chainId"),
                r#type: String::from("uint256"),
            },
            Eip712DomainType {
                name: String::from("verifyingContract"),
                r#type: String::from("address"),
            },
        ],
    );
    types.insert(
        "Message".to_string(),
        vec![
            Eip712DomainType {
                name: String::from("from"),
                r#type: String::from("address"),
            },
            Eip712DomainType {
                name: String::from("to"),
                r#type: String::from("address"),
            },
            Eip712DomainType {
                name: String::from("value"),
                r#type: String::from("uint256"),
            },
            Eip712DomainType {
                name: String::from("gas"),
                r#type: String::from("uint256"),
            },
            Eip712DomainType {
                name: String::from("nonce"),
                r#type: String::from("uint256"),
            },
            Eip712DomainType {
                name: String::from("data"),
                r#type: String::from("bytes"),
            },
            Eip712DomainType {
                name: String::from("validUntilTime"),
                r#type: String::from("uint256"),
            },
        ],
    );
    return types;
}

/// ref. <https://github.com/opengsn/gsn/blob/master/packages/common/src/types/RelayTransactionRequest.ts>
/// ref. <https://github.com/opengsn/gsn/blob/master/packages/common/src/EIP712/RelayRequest.ts>
/// ref. <https://github.com/opengsn/gsn/blob/master/packages/common/src/EIP712/ForwardRequest.ts>
/// ref. <https://github.com/opengsn/gsn/blob/master/packages/contracts/src/forwarder/IForwarder.sol>
/// ref. <https://github.com/opengsn/gsn/blob/master/packages/common/src/EIP712/RelayData.ts>
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelayTransactionRequest {
    pub forward_request: TypedData,
    pub relay_metadata: RelayMetadata,
}

/// ref. <https://github.com/opengsn/gsn/blob/master/packages/common/src/types/RelayTransactionRequest.ts>
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelayMetadata {
    #[serde_as(as = "Option<Hex0xBytes>")]
    pub signature: Option<Vec<u8>>,
}

impl RelayTransactionRequest {
    /// Signs the typed data with the signer and returns the "RelayTransactionRequest"
    /// with the signature attached in the relay metadata.
    /// Use "serde_json::to_vec" to encode to "ethers_core::types::Bytes"
    /// and send the request via "eth_sendRawTransaction".
    pub async fn sign(
        forward_request: TypedData,
        signer: impl ethers_signers::Signer + Clone,
    ) -> io::Result<Self> {
        let sig = signer
            .sign_typed_data(&forward_request)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed sign_typed_data '{}'", e)))?;

        Ok(Self {
            forward_request,
            relay_metadata: RelayMetadata {
                signature: Some(sig.to_vec()),
            },
        })
    }

    /// Decodes the EIP-712 encoded typed data and signature in the relay metadata.
    /// ref. <https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendrawtransaction>
    pub fn decode_signed(b: impl AsRef<[u8]>) -> io::Result<Self> {
        serde_json::from_slice(b.as_ref()).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed serde_json::from_slice '{}'", e),
            )
        })
    }

    /// Recovers the signature and signer address from its relay metadata signature field.
    pub fn recover_signature(&self) -> io::Result<(Signature, H160)> {
        if let Some(sig) = &self.relay_metadata.signature {
            let sig = Signature::try_from(sig.to_owned().as_bytes()).map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!("failed Signature::try_from '{}'", e),
                )
            })?;

            let fwd_req_hash = self.forward_request.encode_eip712().map_err(|e| {
                Error::new(ErrorKind::Other, format!("failed encode_eip712 '{}'", e))
            })?;
            let fwd_req_hash = H256::from_slice(&fwd_req_hash.to_vec());

            let signer_addr = sig.recover(RecoveryMessage::Hash(fwd_req_hash)).map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!(
                        "failed to recover signer address from signature and forward request hash '{}'",
                        e
                    ),
                )
            })?;
            Ok((sig, signer_addr))
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "relay_metadata.signature not found",
            ));
        }
    }
}

/// RUST_LOG=debug cargo test --all-features --package avalanche-types --lib -- evm::eip712::gsn::test_relay_request --exact --show-output
#[test]
fn test_relay_request() {
    use ethers_signers::{LocalWallet, Signer};

    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    macro_rules! ab {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    let k = crate::key::secp256k1::TEST_KEYS[0].clone();
    let key_info = k.to_info(1).unwrap();
    log::info!("{:?}", key_info);
    let signer: LocalWallet = k.signing_key().into();

    let rr = ab!(RelayTransactionRequestBuilder::new()
        .domain_name("hello")
        .domain_version("1")
        .domain_chain_id(U256::from(1))
        .domain_verifying_contract(H160::random())
        .from(H160::random())
        .to(H160::random())
        .value(U256::zero())
        .nonce(U256::from(1))
        .data(vec![1, 2, 3])
        .valid_until_time(U256::MAX)
        .build_and_sign(signer.clone()))
    .unwrap();
    let s = serde_json::to_string_pretty(&rr).unwrap();
    log::info!("typed data: {s}");
    let (sig1, signer_addr) = rr.recover_signature().unwrap();
    assert_eq!(key_info.h160_address, signer_addr);

    let sig2 = ab!(signer.sign_typed_data(&rr.forward_request)).unwrap();
    assert_eq!(sig1, sig2);
}
