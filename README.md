
(still in active development, please contact https://github.com/gyuho or https://github.com/hexfusion for bug reports and questions)

![Crates.io](https://img.shields.io/crates/v/avalanche-types?logo=rust&style=for-the-badge)

https://crates.io/crates/avalanche-types

`avalanche-types` crate implements Avalanche primitive types in Rust:
- Ids (e.g., [`src/ids`](./src/ids))
- Transaction types/serialization (e.g., [`src/platformvm/txs`](./src/platformvm/txs))
- Keys and addresses (e.g., [`src/key/secp256k1`](./src/key/secp256k1))
- Peer-to-peer messages (e.g., [`src/messages`](./src/messages))

The basic types available in this crate are use in other Avalanche Rust projects (e.g., distributed load tester [`blizzard`](https://talks.gyuho.dev/distributed-load-generator-avalanche-2022.html), [`avalanche-ops`](https://github.com/ava-labs/avalanche-ops)).
