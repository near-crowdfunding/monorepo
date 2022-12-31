# Near IDO

```bash
cargo build --all --target wasm32-unknown-unknown --release
```

```bash
near dev-deploy --wasmFile ../target/wasm32-unknown-unknown/release/fungible_token.wasm  --force --initFunction new_default_meta --initArgs '{"owner_id": "trungtin2001.testnet", "total_supply": "100000000000000000"}' 
near dev-deploy --wasmFile ../target/wasm32-unknown-unknown/release/fungible_token.wasm --initFunction new_default_meta --initArgs '{"owner_id": "trungtin2001.testnet", "total_supply": "100000000000000000"}'
```

```bash
near call dev-1672458740713-61913261715971 storage_deposit '{"account_id": "dev-1672460865556-70769880095535", "registration_only": true}' --accountId trungtin2001.testnet --amount 0.1
```

```bash
near call dev-1672458740713-61913261715971 ft_transfer_call '{"receiver_id": "dev-1672460865556-70769880095535", "amount": "10", "msg": "take my mone zy"}' --depositYocto 1 --accountId trungtin2001.testnet --gas 40000000000000
```
