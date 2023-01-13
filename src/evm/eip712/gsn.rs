use std::{
    convert::TryFrom,
    io::{self, Error, ErrorKind},
};

use crate::codec::serde::hex_0x_bytes::Hex0xBytes;
use ethers_core::types::{
    transaction::eip712::{Eip712, TypedData},
    RecoveryMessage, Signature,
};
use primitive_types::{H160, H256};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use zerocopy::AsBytes;

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
    /// Can be sent via "eth_sendRawTransaction".
    pub async fn sign(
        signer: impl ethers_signers::Signer + Clone,
        forward_request: TypedData,
    ) -> io::Result<(Self, ethers_core::types::Bytes)> {
        let sig = signer
            .sign_typed_data(&forward_request)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed sign_typed_data '{}'", e)))?;

        let rr = Self {
            forward_request,
            relay_metadata: RelayMetadata {
                signature: Some(sig.to_vec()),
            },
        };
        let b = serde_json::to_vec(&rr).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed serde_json::to_vec '{}'", e),
            )
        })?;
        Ok((rr, b.into()))
    }

    /// Decodes the RLP-encoded signed "ethers_core::types::transaction::eip2718::TypedTransaction" bytes.
    /// ref. <https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendrawtransaction>
    pub fn decode_signed(b: impl AsRef<[u8]>) -> io::Result<(Self, Signature, H160)> {
        let rr: Self = serde_json::from_slice(b.as_ref()).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed serde_json::from_slice '{}'", e),
            )
        })?;

        let (sig, signer_address) = rr.recover_signature()?;
        Ok((rr, sig, signer_address))
    }

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

    let json = serde_json::json!(
        {
            "forwardRequest": {
                "types": {
                    "EIP712Domain": [
                        {
                            "name": "name",
                            "type": "string"
                        },
                        {
                            "name": "version",
                            "type": "string"
                        },
                        {
                            "name": "chainId",
                            "type": "uint256"
                        },
                        {
                            "name": "verifyingContract",
                            "type": "address"
                        }
                    ],
                    "Message": [
                        {
                            "name": "from",
                            "type": "address"
                        },
                        {
                            "name": "to",
                            "type": "address"
                        },
                        {
                            "name": "value",
                            "type": "uint256"
                        },
                        {
                            "name": "gas",
                            "type": "uint256"
                        },
                        {
                            "name": "nonce",
                            "type": "uint256"
                        },
                        {
                            "name": "data",
                            "type": "bytes"
                        },
                        {
                            "name": "nonce",
                            "type": "uint256"
                        },
                        {
                            "name": "validUntilTime",
                            "type": "uint256"
                        }
                    ]
                },
                "primaryType": "Message",
                "domain": {
                    "name": "example.metamask.io",
                    "version": "1",
                    "chainId": "1",
                    "verifyingContract": "0x0000000000000000000000000000000000000000"
                },
                "message": {
                    "from": "0xA604060890923Ff400e8c6f5290461A83AEDACec",
                    "to": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB",
                    "value": "1658645591",
                    "gas": "0",
                    "nonce": "0",
                    "data": "232cd3ec058eb935a709f093e3536ce26cc9e8e193584b0881992525f6236eef",
                    "validUntilTime": "1658645591"
                }
            },
            "relayMetadata": {
                "signature": "a3cb425eb6a835f35ec2721da37b3f0c5901bce0ff5f0a7a92deb1a122afb7503b89f2aa1e9a00365bd076d31dd09e10152759cf95f4740678c84ea262bdc19d1b"
            }
        }
    );
    let b = serde_json::to_vec(&json).unwrap();
    let parsed = RelayTransactionRequest::decode_signed(&b).unwrap();

    // println!("{:?}", relay_request);
    let sig = ab!(signer.sign_typed_data(&parsed.0.forward_request.clone())).unwrap();
    log::info!("signature: {}", sig);

    log::info!("recovered signature: {}", parsed.1);
    log::info!("recovered signer address: {}", parsed.2);
    assert_eq!(parsed.1, sig);
    assert_eq!(parsed.1.to_string(), "a3cb425eb6a835f35ec2721da37b3f0c5901bce0ff5f0a7a92deb1a122afb7503b89f2aa1e9a00365bd076d31dd09e10152759cf95f4740678c84ea262bdc19d1b");
    assert_eq!(parsed.2, key_info.h160_address);
}
