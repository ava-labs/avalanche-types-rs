
(still in active development, please contact https://github.com/gyuho or https://github.com/hexfusion for bug reports and questions)

![Crates.io](https://img.shields.io/crates/v/avalanche-types?logo=rust&style=for-the-badge)

https://crates.io/crates/avalanche-types

`avalanche-types` crate implements Avalanche primitive types in Rust:
- Ids (e.g., [`src/ids`](./src/ids))
- Transaction types/serialization (e.g., [`src/platformvm/txs`](./src/platformvm/txs))
- Certificates (e.g., [`src/key/cert`](./src/key/cert))
- Keys and addresses (e.g., [`src/key/secp256k1`](./src/key/secp256k1))
- Peer-to-peer messages (e.g., [`src/message`](./src/message))
- RPC chain VM (e.g., [`src/rpcchainvm`](./src/rpcchainvm))
- Genesis generate helper (e.g., [`src/subnet_evm`](./src/subnet_evm))

The basic types available in this crate are used in other Avalanche Rust projects (e.g., distributed load tester [`blizzard`](https://talks.gyuho.dev/distributed-load-generator-avalanche-2022.html), [`avalanche-ops`](https://github.com/ava-labs/avalanche-ops)).
