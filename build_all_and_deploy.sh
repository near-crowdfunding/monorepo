#!/bin/sh

echo ">> Building contract"

rustup target add wasm32-unknown-unknown
cargo build --all --target wasm32-unknown-unknown --release

echo ">> Deploying FT contract"
near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/fungible_token.wasm --initFunction new_default_meta --initArgs '{"owner_id": "trungtin2001.testnet", "total_supply": "100000000000000000"}' --projectKeyDirectory ft-neardev -f &

echo ">> Deploying IDO contract"
near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/smart_contract.wasm --initFunction new --initArgs '{}' -f --projectKeyDirectory ido-neardev &

wait
