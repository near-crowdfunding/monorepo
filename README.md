# Near IDO

```bash
cargo build --all --target wasm32-unknown-unknown --release
```

```bash
export FT_TOKEN_ID=dev-1672478985876-45180344800441;
export IDO_TOKEN_ID=dev-1672479104776-29293708941738;
```

```bash
near dev-deploy --wasmFile ../target/wasm32-unknown-unknown/release/fungible_token.wasm --initFunction new_default_meta --initArgs '{"owner_id": "trungtin2001.testnet", "total_supply": "100000000000000000"}'
```

```bash
near call dev-1672478985876-45180344800441 storage_deposit '{"account_id": "dev-1672479104776-29293708941738", "registration_only": true}' --accountId trungtin2001.testnet --amount 0.1
```

```bash
near call dev-1672458740713-61913261715971 ft_transfer_call '{"receiver_id": "dev-1672477004835-11284451339123", "amount": "10", "msg": "take my mone zy"}' --depositYocto 1 --accountId trungtin2001.testnet --gas 40000000000000
```

```bash
near call dev-1672479950182-11629533174275 create_campaign '{"name": "Demo campaigin 1", "goal": "10000", "incoming_token_id": "dev-1672478985876-45180344800441", "outgoing_token_id": "dev-1672458740713-61913261715971", "amount": "10", "start_time": "1709508541000000000", "end_time": "1809508541000000000"}' --depositYocto 1 --accountId trungtin2001.testnet --gas 40000000000000
```
