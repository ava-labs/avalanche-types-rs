use std::{convert::TryFrom, env::args, io};

use avalanche_types::{evm::eip712::gsn::RelayTransactionRequest, key};
use ethers_core::types::transaction::eip712::TypedData;
use ethers_providers::{Http, Middleware, Provider};

/// cargo run --example evm_send_eip712_relay_request --features="jsonrpc_client evm" -- [HTTP RPC ENDPOINT]
/// cargo run --example evm_send_eip712_relay_request --features="jsonrpc_client evm" -- http://localhost:9876/rpc
#[tokio::main]
async fn main() -> io::Result<()> {
    // ref. https://github.com/env-logger-rs/env_logger/issues/47
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let k = key::secp256k1::TEST_KEYS[0].clone();
    let key_info = k.to_info(1).unwrap();
    log::info!("created hot key:\n\n{}\n", key_info);
    let signer: ethers_signers::LocalWallet = k.signing_key().into();

    let json = serde_json::json!(
        {
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
        }
    );
    let forward_request: TypedData = serde_json::from_value(json).unwrap();

    let (relay_tx_request, signed_bytes) = RelayTransactionRequest::sign(signer, forward_request)
        .await
        .unwrap();
    println!("relay_tx_request: {:?}", relay_tx_request);
    println!("signed_bytes: {}", signed_bytes);

    // let forwarder = ForwardRequest {
    //     from: Address::random(),
    //     to: Address::random(),
    //     value: 100.into(),
    //     gas: 0.into(),
    //     nonce: 0.into(),
    //     data: vec![0, 1, 2],
    //     valid_until_time: U256::MAX,
    // };
    // let forwarder_hash = forwarder.encode_eip712().unwrap();
    // println!("hash {:?}", forwarder_hash);

    let url = args().nth(1).expect("no url given");
    log::info!("running against {url}");

    let provider =
        Provider::<Http>::try_from(url.clone()).expect("could not instantiate HTTP Provider");
    log::info!("created provider for {url}");

    let pending = provider.send_raw_transaction(signed_bytes).await.unwrap();
    log::info!("pending tx hash {}", pending.tx_hash());

    Ok(())
}
