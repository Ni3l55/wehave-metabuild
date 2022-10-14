/*!
Non-Fungible Token implementation with JSON serialization.
NOTES:
  - The maximum balance value is limited by U128 (2**128 - 1).
  - JSON calls should pass U128 as a base-10 string. E.g. "100".
  - The contract optimizes the inner trie structure by hashing account IDs. It will prevent some
    abuse of deep tries. Shouldn't be an issue, once NEAR clients implement full hashing of keys.
  - The contract tracks the change in storage before and after the call. If the storage increases,
    the contract requires the caller of the contract to attach enough deposit to the function call
    to cover the storage cost.
    This is done to prevent a denial of service attack on the contract by taking all available storage.
    If the storage decreases, the contract will issue a refund for the cost of the released storage.
    The unused tokens from the attached deposit are also refunded, so it's safe to
    attach more deposit than required.
  - To prevent the deployed contract from being modified or deleted, it should not have any access
    keys on its account.
*/
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::storage_management::{StorageBalance, StorageBalanceBounds, StorageManagement};
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};
use near_sdk::collections::LookupMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{
    env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue, PromiseError, ext_contract, log, Gas, Balance
};
use near_sdk::serde_json::json;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

const TGAS: u64 = 1000000000000;
const DEFAULT_TOKEN_STORAGE: u128 = 10_000_000_000_000_000_000_000_000; // 10 N, for fungible token
const DEFAULT_FT_DECIMALS: u8 = 8;
const DEFAULT_DAO_STORAGE: u128 = 10_000_000_000_000_000_000_000_000; // 10 N, for dao
const MINT_STORAGE_COST: u128 = 1_000_000_000_000_000_000_000_000; // 1 N

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[ext_contract(ext_ft)]
trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
    fn ft_balance_of(&mut self, account_id: AccountId);
    fn storage_balance_of(&self, account_id: AccountId);
    fn get_accounts(&mut self);
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "WeHave NFT".to_string(),
                symbol: "WEHAVE".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        require!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        }
    }

    /// Mint a new token with ID=`token_id` belonging to `token_owner_id`.
    ///
    /// Since this example implements metadata, it also requires per-token metadata to be provided
    /// in this call. `self.tokens.mint` will also require it to be Some, since
    /// `StorageKey::TokenMetadata` was provided at initialization.
    ///
    /// `self.tokens.mint` will enforce `predecessor_account_id` to equal the `owner_id` given in
    /// initialization call to `new`.
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        token_metadata: TokenMetadata,
        ft_name: String,
        ft_supply: U128,
        holders: Vec<AccountId>,
        shares: Vec<U128>
    ) -> Promise {
        log!("Arrived at nft_mint");
        // Only contract owner is allowed to mint (caller = owner)
        assert_eq!(env::predecessor_account_id(), self.tokens.owner_id, "Account unauthorized to mint.");

        // TOKENIZE: Create a new fungible token
        const FT_CODE: &[u8] = include_bytes!("../../ft/target/wasm32-unknown-unknown/release/wehave_ft.wasm");

        let ft_account_id: AccountId = AccountId::new_unchecked(
          format!("{}.{}", ft_name, env::current_account_id())  // TODO use token namings // TODO extract ft acct id from metadata instead of this extra param
        );

        log!("Creating account & deploying contract for new fungible token: {}", ft_account_id);

        let ft_title = token_metadata.title.as_ref().expect("Title of fungible token is missing...");
        let ft_symbol = token_metadata.title.as_ref().expect("Title of fungible token is missing...").chars().take(4).collect::<String>().to_uppercase();

        let ft_metadata = FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: format!("{} Token", ft_title),    // Ferrari F40 Token
            symbol: ft_symbol, // FERR
            icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),   // TODO: a nice WHV svg icon?
            reference: None,
            reference_hash: None,
            decimals: DEFAULT_FT_DECIMALS,
        };

        Promise::new(ft_account_id.clone())
            .create_account()
            .add_full_access_key(env::signer_account_pk()) // Crowdfund becomes owner.. ??
            .deploy_contract(FT_CODE.to_vec())
            .transfer(DEFAULT_TOKEN_STORAGE) // Transfer some NEAR for storage from the NFT contract itself
            .function_call(
                String::from("new"),
                json!({"owner_id": env::current_account_id(), "total_supply": self.calculate_total_supply(ft_supply, DEFAULT_FT_DECIMALS), "holders": holders, "shares": shares, "metadata": ft_metadata})
                    .to_string()
                    .as_bytes()
                    .to_vec(),
                0,
                Gas(20*TGAS),
            ).then(
                Self::ext(env::current_account_id())
                .with_static_gas(Gas(7*TGAS))
                .with_attached_deposit(MINT_STORAGE_COST)   // Transfer some NEAR for minting cost
                .ft_deploy_callback(token_id, ft_account_id, token_metadata)
            )
    }

    #[handle_result]
    #[private]
    #[payable]
    pub fn ft_deploy_callback(&mut self, token_id: TokenId, owner_id: AccountId, token_metadata: TokenMetadata, #[callback_result] call_result: Result<(), PromiseError>) {
        if call_result.is_err() {
            log!("Could not deploy {:?}", owner_id);
            // Potentially give back fundings here...
        } else {
            log!("Minting item {} for ft account {}", token_id, owner_id);

            // Add to collection: Mint new item owned by fungible token
            self.tokens.internal_mint(token_id, owner_id.clone(), Some(token_metadata));

            // TODO: mint only after dao?

            // Deploy dao for token
            let dao_account_id: AccountId = AccountId::new_unchecked(
                format!("dao-{}", owner_id) // dao-ferrarif40.nft.test.near? Can't use a '.' since not in that contract
             );

            log!("Creating account & deploying DAO: {}", dao_account_id);

            // TOKENIZE: Create a new fungible token
            const DAO_CODE: &[u8] = include_bytes!("../../item-dao/target/wasm32-unknown-unknown/release/wehave_item_dao.wasm");

            Promise::new(dao_account_id.clone())
                .create_account()
                .add_full_access_key(env::signer_account_pk()) // Crowdfund --> NFT becomes owner.. ??
                .transfer(DEFAULT_DAO_STORAGE)
                .deploy_contract(DAO_CODE.to_vec())
                .function_call(
                    String::from("new"),
                    json!({"item_ft": owner_id})
                        .to_string()
                        .as_bytes()
                        .to_vec(),
                    0,
                    Gas(5*TGAS),    // TODO measure gas stuff
                ).then(
                    Self::ext(env::current_account_id())
                    .with_static_gas(Gas(1*TGAS))
                    .item_dao_deploy_callback()
                );
        }
    }

    #[handle_result]
    #[private]
    pub fn item_dao_deploy_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>) {
        if call_result.is_err() {
            log!("Could not deploy DAO");
        } else {
            log!("Dao deployed successfully!");
        }
    }

    fn calculate_total_supply(&self, ft_supply: U128, decimals: u8) -> U128 {
        let multiplier: u128 = 10;
        let mut ft_supply_u128: u128 = ft_supply.into();
        ft_supply_u128 = ft_supply_u128 * (multiplier.pow(u32::from(decimals)));
        U128::from(ft_supply_u128)
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use std::collections::HashMap;

    use super::*;

    const MINT_STORAGE_COST: u128 = 5870000000000000000000 * 10;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn sample_token_metadata() -> TokenMetadata {
        TokenMetadata {
            title: Some("Olympus Mons".into()),
            description: Some("The tallest mountain in the charted solar system".into()),
            media: None,
            media_hash: None,
            copies: Some(1u64),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.nft_token("1".to_string()), None);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_mint() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());

        let total_supply: U128 = U128::from(1000000);
        let decimals: u8 = 8;
        assert_eq!(contract.calculate_total_supply(total_supply, decimals), U128::from(100000000000000));

        let ft_name = String::from("test");
        let ft_supply = U128::from(1000000);
        let holders: Vec<AccountId> = vec!("alice.test.near".parse().unwrap());
        let shares = vec!(1000000.into());

        let token_id = "0".to_string();
        let token = contract.nft_mint(token_id.clone(), sample_token_metadata(), ft_name, ft_supply, holders, shares);
    }
}
