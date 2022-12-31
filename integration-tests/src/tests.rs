use near_units::parse_near;
use serde_json::json;
use workspaces::prelude::*;
use workspaces::{network::Sandbox, Account, Contract, Worker};

mod helpers;

const FT_WASM_FILEPATH: &str = "./target/wasm32-unknown-unknown/release/fungible_token.wasm";
const IDO_WASM_FILEPATH: &str = "./target/wasm32-unknown-unknown/release/smart_contract.wasm";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initiate environemnt
    let worker = workspaces::sandbox().await?;

    // deploy contracts
    let nft_wasm = std::fs::read(FT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;
    let ido_wasm = std::fs::read(IDO_WASM_FILEPATH)?;
    let ido_contract = worker.dev_deploy(&ido_wasm).await?;

    // create accounts
    let owner = worker.root_account();
    let alice = owner
        .create_subaccount(&worker, "alice")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;
    let bob = owner
        .create_subaccount(&worker, "bob")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;
    let charlie = owner
        .create_subaccount(&worker, "charlie")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    // Initialize contracts
    nft_contract
        .call(&worker, "new_default_meta")
        .args_json(serde_json::json!({"owner_id": owner.id()}))?
        .transact()
        .await?;
    ido_contract
        .call(&worker, "new")
        .args_json(serde_json::json!({"owner_id": owner.id()}))?
        .transact()
        .await?;

    Ok(())
}
