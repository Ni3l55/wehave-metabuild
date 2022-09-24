use std::{env, fs};
use near_units::parse_near;
use serde_json::json;
use workspaces::prelude::*;
use workspaces::{network::Sandbox, Account, Contract, Worker};
use near_sdk::json_types::U128;
use near_sdk::AccountId;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ---------------- ARRANGE ----------------

    // Read crowdfund WASM from cmd line
    let wasm_arg: &str = &(env::args().nth(1).unwrap());
    let wasm_filepath = fs::canonicalize(env::current_dir()?.join(wasm_arg))?;
    let crowdfund_wasm = std::fs::read(wasm_filepath)?;

    // Read NFT WASM from cmd line
    let wasm_arg_nft: &str = &(env::args().nth(2).unwrap());
    let wasm_filepath_nft = fs::canonicalize(env::current_dir()?.join(wasm_arg_nft))?;
    let nft_wasm = std::fs::read(wasm_filepath_nft)?;

    // Read fusdc WASM from cmd line
    let wasm_arg_fusdc: &str = &(env::args().nth(3).unwrap());
    let wasm_filepath_fusdc = fs::canonicalize(env::current_dir()?.join(wasm_arg_fusdc))?;
    let fusdc_wasm = std::fs::read(wasm_filepath_fusdc)?;

    // Create a sandbox (workspace)
    let worker = workspaces::sandbox().await?;

    // Create root test account (test.near)
    let account = worker.root_account()?;

    // Create main wehave account (wehave.test.near)
    let wehave_account = account
        .create_subaccount(&worker, "wehave")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    // Create crowdfund account and deploy contract (crowdfund.test.near)
    let crowdfund_account = account
        .create_subaccount(&worker, "crowdfund")
        .initial_balance(parse_near!("100 N"))
        .transact()
        .await?
        .into_result()?;

    let crowdfund_contract = crowdfund_account.deploy(&worker, &crowdfund_wasm)
        .await?
        .into_result()?;

    // Initialize crowdfund contract
    let nft_account_id = "nft.test.near";
    wehave_account.call(&worker, crowdfund_contract.id(), "new")
        .args_json(json!({
            "nft_account_id": nft_account_id
        }))?
        .transact()
        .await?;

    // Create NFT account and deploy contract (nft.test.near)
    let nft_account = account
        .create_subaccount(&worker, "nft")
        .initial_balance(parse_near!("200 N"))
        .transact()
        .await?
        .into_result()?;

    let nft_contract = nft_account.deploy(&worker, &nft_wasm)
        .await?
        .into_result()?;

    println!("Initializing NFT contract.");

    // Initialize NFT contract
    let crowdfund_id: AccountId = "crowdfund.test.near".parse().unwrap();
    wehave_account.call(&worker, nft_contract.id(), "new_default_meta")
        .args_json(serde_json::json!({
            "owner_id": crowdfund_id    // Owner of NFT contract should be crowdfund; he is the minter
        }))?
        .transact()
        .await?;

    // Create fake usdc account in sandbox and deploy WASM (fusdc.test.near)
    let fusdc_account = account
        .create_subaccount(&worker, "fusdc")
        .initial_balance(parse_near!("100 N"))
        .transact()
        .await?
        .into_result()?;

    let fusdc_contract = fusdc_account.deploy(&worker, &fusdc_wasm)
        .await?
        .into_result()?;

    println!("Initializing fake usdc contract.");

    // Initialize fake usdc contract for testing
    let wehave_id: AccountId = "wehave.test.near".parse().unwrap();
    wehave_account.call(&worker, fusdc_contract.id(), "new_default_meta")
        .args_json(serde_json::json!({
            "owner_id": wehave_id,      // WeHave.test.near gets all the supply
            "total_supply": U128::from(1000000)
        }))?
        .transact()
        .await?;

    // Create a user account (alice.test.near)
    let alice = account
        .create_subaccount(&worker, "alice")
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await?
        .into_result()?;

    // Create a user account (bob.test.near)
    let bob = account
        .create_subaccount(&worker, "bob")
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await?
        .into_result()?;

    // ---------------- ACT ----------------

    println!("Distributing some USDC from WeHave to Alice and Bob");

    // Distribute some fake usdc to alice.test.near and bob.test.near
    let alice_id: AccountId = "alice.test.near".parse().unwrap();
    distribute_fusdc(&worker, &fusdc_contract, &wehave_account, &alice_id).await?;
    let bob_id: AccountId = "bob.test.near".parse().unwrap();
    distribute_fusdc(&worker, &fusdc_contract, &wehave_account, &bob_id).await?;

    println!("Alice creates a ferrari crowdfund for $1000");
    // Alice creates a ferrari to crowdfund
    crowdfund_new_item(&worker, &crowdfund_contract, &alice, String::from("ferrari"), 1000).await?;
    println!("Alice funds the ferrari for 400 usdc");
    // Alice funds the item
    fund_item(&worker, &fusdc_contract, &crowdfund_contract, &alice, String::from("0"), String::from("400")).await?;

    println!("Bob funds the ferrari for 700 usdc");
    // Bob funds the item
    fund_item(&worker, &fusdc_contract, &crowdfund_contract, &bob, String::from("0"), String::from("700")).await?;


    println!("Alice creates a rolex crowdfund for $500");
    // Alice creates a ferrari to crowdfund
    crowdfund_new_item(&worker, &crowdfund_contract, &alice, String::from("rolex"), 500).await?;
    println!("Alice funds the rolex for 200 usdc");
    // Alice funds the item
    fund_item(&worker, &fusdc_contract, &crowdfund_contract, &alice, String::from("1"), String::from("200")).await?;
    println!("Bob funds the rolex for 100 usdc");
    // Bob funds the item
    fund_item(&worker, &fusdc_contract, &crowdfund_contract, &bob, String::from("1"), String::from("100")).await?;
    println!("Bob funds the rolex for 200 usdc");
    // Bob funds the item
    fund_item(&worker, &fusdc_contract, &crowdfund_contract, &bob, String::from("1"), String::from("200")).await?;

    Ok(())
}

async fn distribute_fusdc(worker: &Worker<Sandbox>, contract: &Contract, user: &Account, to: &AccountId) -> anyhow::Result<()> {
    // Register the user by storage deposit
    let result = user.call(&worker, contract.id(), "storage_deposit")
        .args_json(json!({"account_id": to}))?
        .max_gas()
        .deposit(parse_near!("1 N"))
        .transact()
        .await?;

    let result = user.call(&worker, contract.id(), "ft_transfer")
        .args_json(json!({"receiver_id": to, "amount": U128::from(1000)}))?
        .max_gas()
        .deposit(parse_near!("1 yN"))
        .transact()
        .await?;

    println!("{:?}", result.logs());

    Ok(())
}

async fn crowdfund_new_item(worker: &Worker<Sandbox>, contract: &Contract, user: &Account, item_name: String, goal: u128) -> anyhow::Result<()> {
    let result = user.call(&worker, contract.id(), "new_item")
        .args_json(json!({"item_name": item_name, "goal": goal}))?
        .transact()
        .await?;

    Ok(())
}

async fn fund_item(worker: &Worker<Sandbox>, fusdc_contract: &Contract, crowdfund_contract: &Contract, user: &Account, item_index: String, amount: String) -> anyhow::Result<()> {
    let crowdfund_id: AccountId = "crowdfund.test.near".parse().unwrap();

    // Register the user by storage deposit
    let result = user.call(&worker, fusdc_contract.id(), "storage_deposit")
        .args_json(json!({"account_id": crowdfund_id}))?
        .max_gas()
        .deposit(parse_near!("1 N"))
        .transact()
        .await?;

    let result = user.call(&worker, fusdc_contract.id(), "ft_transfer_call")
        .args_json(json!({
            "receiver_id": crowdfund_id,
            "amount": amount,
            "memo": "funding the ferrari",
            "msg": item_index
        }))?
        .max_gas()
        .deposit(parse_near!("1 yN"))
        .transact()
        .await?;

    println!("Result: {:?}", result.logs());

    Ok(())
}
