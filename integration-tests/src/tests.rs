use std::{env, fs};
use near_units::parse_near;
use serde_json::json;
use workspaces::prelude::*;
use workspaces::{network::Sandbox, Account, Contract, Worker};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_sdk::AccountId;
use near_sdk::log;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ---------------- ARRANGE ----------------

    // Read WASM from cmd line
    let wasm_arg: &str = &(env::args().nth(1).unwrap());
    let wasm_filepath = fs::canonicalize(env::current_dir()?.join(wasm_arg))?;

    // Create a sandbox (workspace) and deploy WASM
    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(wasm_filepath)?;
    let contract = worker.dev_deploy(&wasm).await?;

    // Create account in sandbox (alice.test.near)
    let account = worker.root_account()?;
    let alice = account
        .create_subaccount(&worker, "alice")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    // ---------------- ACT ----------------

    // begin tests
    test_mint_nft(&alice, &contract, &worker).await?;   // Alice mints "1" for herself

    // ---------------- ASSERT ----------------

    check_if_nft_minted(&alice, &contract, &worker).await?;
    //check_fungible_token_balance().await?;

    Ok(())
}

async fn test_mint_nft(
    user: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<()> {

    println!("test");

    let owner_id: AccountId = "alice.test.near".parse().unwrap();

    println!("test2");

    user.call(&worker, contract.id(), "new_default_meta")
        .args_json(serde_json::json!({
            "owner_id": owner_id,
        }))?
        .transact()
        .await?;

    println!("test3");

    let token_id: TokenId = String::from("1");
    let token_metadata: TokenMetadata = TokenMetadata {
        title: None, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
        description: None, // free-form description
        media: None, // URL to associated media, preferably to decentralized, content-addressed storage
        media_hash: None, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
        copies: None, // number of copies of this set of metadata in existence when token was minted.
        issued_at: None, // ISO 8601 datetime when token was issued or minted
        expires_at: None, // ISO 8601 datetime when token expires
        starts_at: None, // ISO 8601 datetime when token starts being valid
        updated_at: None, // ISO 8601 datetime when token was last updated
        extra: None, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
        reference: None, // URL to an off-chain JSON file with more info.
        reference_hash: None, //
    };

    let result = user.call(&worker, contract.id(), "nft_mint")
        .args_json(json!({"token_id": token_id, "token_metadata": token_metadata}))?
        .max_gas()
        .transact()
        .await?;

    println!("{:?}", result.logs());

    Ok(())
}

async fn check_if_nft_minted(user: &Account, contract: &Contract, worker: &Worker<Sandbox>) -> anyhow::Result<()> {
    let token_id: TokenId = String::from("1");

    let result = user.call(&worker, contract.id(), "nft_token")
        .args_json(json!({"token_id": token_id}))?
        .transact()
        .await?;

    let token: Option<Token> = result.json()?;

    println!("{:?}", result.logs());
    println!("{:#?}", token);

    Ok(())
}
