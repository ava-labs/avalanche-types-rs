
[<img alt="crates.io" src="https://img.shields.io/crates/v/avalanche-types.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/avalanche-types)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-avalanche_types-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/avalanche-types)
![Github Actions](https://github.com/ava-labs/avalanche-types-rs/actions/workflows/test-and-release.yml/badge.svg)

## AvalancheGo Compatibility
| Crate Version(s) | AvalancheGo Version(s) | Protocol Version |
| --- | ----------- | --- |
| v0.0.134-155 | v1.9.2,v1.9.3 | 19
| v0.0.156 | v1.9.4 | 20

## Introduction
The `avalanche-types` crate implements Avalanche primitive types in Rust. `avalanche-types` is the canonical representation of Avalanche types in Rust. Avalanche types are separated by modules and are all under the `src` directory.

This crate also provides an SDK library for developing subnets in Rust. For the SDK functionality, see `src/subnet` which contains everything required to build a subnet VM in Rust.
The following VMs were built with the SDK:
* Simple Rust VM: [TimestampVM](https://github.com/ava-labs/timestampvm-rs)
* Complex Rust VM: [SpacesVM](https://github.com/ava-labs/spacesvm-rs)

## Getting Started

Examples can be found in [`examples`] and is a good first step to getting an understanding of general usage.

### Tutorials

- [How to Build a Simple Rust VM](https://docs.avax.network/subnets/create-a-simple-rust-vm) tutorial provides
a basic example of using the Rust SDK.

### Rust Version

`avalanche-types` currently works on Rust `1.65` and above as it requires support for the 2021 edition. This project uses the stable toolchain.

## Getting Help

First please try find the answer to your question in the code documentation.
If more clarification is required, try opening an [issue] with the question.

[issue]: https://github.com/ava-labs/avalanche-types-rs/issues/new

## Features

- Ids (e.g., [`src/ids`](./src/ids))
- Transaction types/serialization (e.g., [`src/platformvm/txs`](./src/platformvm/txs))
- Certificates (e.g., [`src/key/cert`](./src/key/cert))
- Keys and addresses (e.g., [`src/key/secp256k1`](./src/key/secp256k1))
- Peer-to-peer messages (e.g., [`src/message`](./src/message))
- RPC chain VM (e.g., [`src/subnet/rpc`](./src/subnet/rpc))
- Genesis generate helper (e.g., [`src/subnet_evm`](./src/subnet_evm))
- Protobuf generated stubs and helpers (e.g., [`src/proto`](./src/proto))

The basic types available in this crate are used in other Avalanche Rust projects (e.g., distributed load tester [`blizzard`](https://talks.gyuho.dev/distributed-load-generator-avalanche-2022.html), [`avalanche-ops`](https://github.com/ava-labs/avalanche-ops)).

## License

This project is licensed under the [BSD 3](LICENSE).
