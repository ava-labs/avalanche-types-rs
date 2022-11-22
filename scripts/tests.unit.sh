#!/usr/bin/env bash
set -xue

if ! [[ "$0" =~ scripts/tests.unit.sh ]]; then
  echo "must be run from repository root"
  exit 255
fi

# TODO: better way to skip proto tests?
RUST_LOG=debug cargo test --workspace \
--features avalanche-types/avalanchego \
--features avalanche-types/cert \
--features avalanche-types/client \
--features avalanche-types/codec_base64 \
--features avalanche-types/codec_big_int \
--features avalanche-types/evm \
--features avalanche-types/kms_aws \
--features avalanche-types/libsecp256k1 \
--features avalanche-types/mnemonic \
--features avalanche-types/subnet_evm \
-- --show-output

echo "ALL SUCCESS!"
