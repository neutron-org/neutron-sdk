#!/bin/bash
set -e

WASMS=$(find artifacts -type f -name "*.wasm")

BIN="utils/check_contract/target/release/check_contract"
cargo build --release --manifest-path=utils/check_contract/Cargo.toml

for W in $WASMS; do
  echo -n "Checking $(basename "$W")... "
  $BIN $W
done