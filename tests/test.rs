use near_sdk::{json_types::U128, serde_json::json};
use tokio::fs;
use workspaces::prelude::*;

#[tokio::test]
async fn workspaces_test() -> anyhow::Result<()> {
    let wasm = fs::read("target/wasm32-unknown-unknown/release/vip_list.wasm").await?;

    let worker = workspaces::sandbox();
    let root = worker.root_account();
    let alice = root
        .create_subaccount(&worker, "alice")
        .transact()
        .await?
        .into_result()?;

    let contract = worker.dev_deploy(&wasm).await?;
    // No failures
    contract
        .call(&worker, "new")
        .args_json(json!({
            "owner_id": root.id(),
        }))?
        .transact()
        .await?;

    root
        .call(&worker, contract.id(), "add_list")
        .args_json(json!({
            "account_id": root.id(),
        }))?
        .transact()
        .await?;

    root
        .call(&worker, contract.id(), "add_to_list")
        .args_json(json!({
            "account_id": alice.id(),
            "allowance": U128::from(42)
        }))?
        .transact()
        .await?;

    let res = contract
        .call(&worker, "vip_allowance")
        .args_json(json!({
            "account_id": alice.id(),
            "list_id": root.id(),
        }))?
        .transact()
        .await?;
    assert_eq!("42", res.json::<String>()?);

    Ok(())
}
