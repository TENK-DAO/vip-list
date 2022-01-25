use near_sdk::{
    json_types::U128,
    serde_json::{self, json},
};
use tokio::fs;
use workspaces::prelude::*;

#[tokio::test]
async fn workspaces_test() -> anyhow::Result<()> {
    let wasm = fs::read("target/wasm32-unknown-unknown/release/vip_list.wasm").await?;

    let worker = workspaces::sandbox();
    let root = worker.root_account();

    let args = json!({
        "owner_id": root.id(),
    })
    .to_string();

    let contract = worker.dev_deploy(wasm).await?;
    // No failures
    let res = contract
        .call(&worker, "new")
        .args(args.into_bytes())
        .transact()
        .await?;

    // res.json()?;

    let res = contract
        .call(&worker, "add_list")
        .args_json(json!({
            "account_id": root.id(),
        }))?
        .transact()
        .await?;

    println!("{:#?}", res);

    let res = contract
        .call(&worker, "add_to_list")
        .args_json(json!({
            "account_id": root.id(),
            "allowance": U128::from(42)
        }))?
        .transact()
        .await?;

    println!("{:#?}", res);

    let res = contract
        .call(&worker, "vip_allowance")
        .args_json(json!({
            "account_id": root.id(),
        }))?
        .transact()
        .await?;
    println!("{:#?}", res.json()?);

    Ok(())
}
