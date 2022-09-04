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

    // Create a sandbox (workspace)
    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(wasm_filepath)?;

    let account = worker.root_account()?;

    // Create nft account in sandbox and deploy WASM (nft.test.near)
    let sc_account = account
        .create_subaccount(&worker, "nft")
        .initial_balance(parse_near!("200 N"))
        .transact()
        .await?
        .into_result()?;

    let contract = sc_account.deploy(&worker, &wasm)
        .await?
        .into_result()?;

    // Create account in sandbox (crowdfund.test.near)
    let crowdfund = account
        .create_subaccount(&worker, "crowdfund")
        .initial_balance(parse_near!("100 N"))
        .transact()
        .await?
        .into_result()?;

    // Create account in sandbox (niels.test.near) (acts as funder)
    let niels = account
        .create_subaccount(&worker, "niels")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    // ---------------- ACT ----------------

    // Mint 1 nft --> create new ft in background
    test_mint_nft(&crowdfund, &contract, &worker).await?;   // crowdfund mints "1"

    // ---------------- ASSERT ----------------

    check_if_nft_minted(&crowdfund, &contract, &worker).await?;

    check_fungible_token_balance(&niels, &contract, &worker).await?;

    check_fungible_token_balance(&sc_account, &contract, &worker).await?;

    //check_all_accounts(&niels, &contract, &worker).await?;

    Ok(())
}

async fn test_mint_nft(
    user: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<()> {

    println!("user = {:?}", user);
    println!("contract = {:?}", contract);

    let owner_id: AccountId = "crowdfund.test.near".parse().unwrap();

    println!("Initializing contract at {:?}", contract);

    user.call(&worker, contract.id(), "new_default_meta")
        .args_json(serde_json::json!({
            "owner_id": owner_id,
        }))?
        .transact()
        .await?;

    println!("Initialized.");

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

    println!("Minting as user {:?} for contract with id {:?}, for token {:?}", user, contract.id(), token_id);

    let result = user.call(&worker, contract.id(), "nft_mint")
        .args_json(json!({"token_id": token_id, "token_metadata": token_metadata}))?
        .max_gas()
        .deposit(parse_near!("50 N"))
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

    Ok(())
}

async fn check_fungible_token_balance(user: &Account, contract: &Contract, worker: &Worker<Sandbox>) -> anyhow::Result<()> {
    let result = user.call(&worker, contract.id(), "check_ft_balance")
        .args_json(json!({"token_id": "1", "account_id": user.id()}))?
        .max_gas()
        .transact()
        .await?;

    println!("TOKEN_BALANCE {:?}: {:#?}", user, result);

    Ok(())
}

async fn check_all_accounts(user: &Account, contract: &Contract, worker: &Worker<Sandbox>) -> anyhow::Result<()> {
    let result = user.call(&worker, contract.id(), "get_ft_accounts")
        .args_json(json!({"token_id": "1"}))?
        .max_gas()
        .transact()
        .await?;

    println!("ALL ACCOUNTS {:?}: {:#?}", user, result);

    Ok(())
}
