#!/bin/bash
# Gamverse Kusama Testnet Deployment
# Run this on Linux/Mac with Rust + Substrate installed

echo "Building Gamverse Protocol..."
cargo build --release

echo "Generating chain spec..."
./target/release/node-template build-spec --disable-default-bootnode > kusama-plain.json

echo "Exporting genesis state (Parachain ID: 2000)..."
./target/release/node-template export-genesis-state --parachain-id 2000 > para-2000-genesis
./target/release/node-template export-genesis-wasm > para-2000-wasm

echo "Starting collator node..."
./target/release/parachain-template-collator \
  --collator \
  --force-authoring \
  --parachain-id 2000 \
  --base-path /tmp/parachain \
  --chain kusama-plain.json \
  -- \
  --chain kusama \
  --execution wasm

echo "Gamverse Parachain (ID 2000) is running on Kusama!"
echo "Get KSM from: https://kusama.network/faucet"
