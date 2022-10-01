use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, ext_contract, require, env, AccountId, BorshStorageKey, Balance, CryptoHash, PanicOnDefault, Promise, Gas, PromiseError, PromiseOrValue};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;

use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{Token, TokenId};

const TGAS: u64 = 1_000_000_000_000;
const DEFAULT_TOKEN_SUPPLY: u128 = 1_000_000;

// Define the state of the smart contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // The account id of the nft used for tokenization
    nft_account_id: AccountId,

    // Different crowdfunded items (index -> name)
    items: UnorderedMap<u128, String>,

    // Crowdfund goal per item (index -> balance)
    goals: UnorderedMap<u128, u128>,

    // Overview of fundings per item (index -> account -> USDC funded)
    fundings: UnorderedMap<u128, UnorderedMap<AccountId, Balance>>,

    // Summary of total funding per item for (index -> balance)
    total_fundings: UnorderedMap<u128, u128>,

    // List of item indices that have been tokenized (token deployed, different from just goal reached)
    tokenized: UnorderedSet<u128>
}

// Define storage keys (also for nested collection)
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Items,
    Goals,
    Fundings,
    Subfunding { item_index_hash: CryptoHash },
    TotalFundings,
    Tokenized,
}

#[ext_contract(ext_nft)]
trait NonFungibleToken {
    fn nft_mint(&mut self, token_id: TokenId, token_metadata: TokenMetadata, ft_name: String, ft_supply: U128, holders: Vec<AccountId>, shares: Vec<U128>);
}

pub trait FungibleTokenReceiver {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(nft_account_id: AccountId) -> Self {
        require!(!env::state_exists(), "Already initialized");

        Self{
            nft_account_id: nft_account_id,
            items: UnorderedMap::new(StorageKeys::Items),
            goals: UnorderedMap::new(StorageKeys::Goals),
            fundings: UnorderedMap::new(StorageKeys::Fundings),
            total_fundings: UnorderedMap::new(StorageKeys::TotalFundings),
            tokenized: UnorderedSet::new(StorageKeys::Tokenized),
        }
    }

    pub fn new_item(&mut self, item_name: String, goal: u128) {
        require!(goal > 0, "Goal is smaller than zero.");

        // TODO: only allow new crowdfund proposals from certain address(es)

        // Potentially change this index in the future. For now just auto increment
        let amt = u128::from(self.items.len());

        self.items.insert(&amt, &item_name);

        // Create goal for item
        self.goals.insert(&amt, &goal);

        // Instantiate the funding map for this item
        self.fundings.insert(&amt, &UnorderedMap::new(StorageKeys::Subfunding {
                                    item_index_hash: env::sha256_array(&amt.to_be_bytes()),
                                })
                            );

        // Instantiate the total funding for the item
        let start: u128 = 0;
        self.total_fundings.insert(&amt, &start);
    }

    pub fn get_crowdfund_progress(&self, item_index: u128) -> u128 {
        self.total_fundings.get(&item_index).expect("Incorrect item index!")
    }

    pub fn get_crowdfund_goal(&self, item_index: u128) -> u128 {
        self.goals.get(&item_index).expect("Incorrect item index!")
    }

    pub fn tokenize_item(&mut self, item_index: u128) -> Promise {
        let crowdfund_progress = self.get_crowdfund_progress(item_index);
        let crowdfund_goal = self.get_crowdfund_goal(item_index);

        require!(crowdfund_progress == crowdfund_goal, "Goal not yet reached.");
        require!(!self.tokenized.contains(&item_index), "This item has already been tokenized.");

        // TOKENIZE: call the custom NFT that creates a token
        log!("Serializing crowdfund distribution.");

        let item_fundings = self.fundings.get(&item_index).expect("Could not get fundings for this item.");

        // Make crowdfund distribution serializable -> split funders (holders) & their funds (shares) into 2 Vec's
        let holders_serializable = item_fundings.keys_as_vector().to_vec();
        let shares = item_fundings.values_as_vector().to_vec();

        // Make shares serializable
        let mut shares_serializable: Vec<U128> = Vec::new();
        for share in &shares {
            shares_serializable.push(U128::from(*share));
        }

        // Call NFT mint on nft contract, pass new fungible token info
        log!("Calling nft_mint from crowdfund.");

        let token_metadata = TokenMetadata {
            title: None,
            description: None,
            media: None,
            media_hash: None,
            copies: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        };

        ext_nft::ext(self.nft_account_id.clone())
            .with_static_gas(Gas(10*TGAS))   // TODO token metadata!
            .nft_mint(item_index.to_string(), token_metadata, self.items.get(&item_index).expect("Incorrect item index!"), U128::from(DEFAULT_TOKEN_SUPPLY), holders_serializable, shares_serializable)
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(1*TGAS))
                    .nft_mint_callback(item_index)
                )
    }

    #[handle_result]
    #[private]
    #[payable]
    pub fn nft_mint_callback(&mut self, item_index: u128, #[callback_result] call_result: Result<(), PromiseError>) {
        if call_result.is_err() {
            log!("Something went wrong during nft_mint.");
            // Decide what to do here
            // Potentially give back fundings
            // Or try again
        } else {
            log!("nft_mint was successful!");
            self.tokenized.insert(&item_index);
        }
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    // When an account sends usdc to the crowdfund
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128> {
        // TODO: only accept code from a certain stablecoin when decided

        log!("{:?} tokens transferred from {} with msg: {}", amount, sender_id, msg);

        // Check the item_index which is in the message
        let item_index: u128 = msg.parse().unwrap();

        // Check crowdfund information
        let item_progress = self.get_crowdfund_progress(item_index);
        let goal = self.goals.get(&item_index).expect("Unable to retrieve goal of item.");

        assert!(item_progress < goal, "The goal has already been reached.");

        log!("Funding item {} with progress {} and goal {}", item_index, item_progress, goal);

        // Register the funding for the item
        let mut item_fundings = self.fundings.get(&item_index).expect("Incorrect item index!");
        let mut funded_by_sender: Balance = item_fundings.get(&sender_id).unwrap_or_else(|| 0);

        if let Some(mut new_balance) = funded_by_sender.checked_add(amount.into()) {
            if (item_progress + u128::from(amount)) >= goal {
                log!("CROWFUNDING GOAL REACHED");

                // if this surpasses the limit, give back what's leftover
                let leftover = item_progress + u128::from(amount) - goal;
                new_balance = new_balance - leftover;

                // Save the funding which was performed (BEFORE! minting the nft / token issue)
                item_fundings.insert(&sender_id, &new_balance);
                self.fundings.insert(&item_index, &item_fundings);
                self.total_fundings.insert(&item_index, &goal);

                log!("Initiating tokenization...");

                // TODO in the future: just flip a variable here
                // Tokenize to be called manually by us when item is acquired in warehouse

                self.tokenize_item(item_index.clone());

                // Return leftover token
                return PromiseOrValue::Value(U128::from(leftover));
            } else {
                let mut item_total_funding = self.total_fundings.get(&item_index).expect("Total funding not found.");
                item_total_funding = item_progress + u128::from(amount);

                // Save the funding which was performed
                item_fundings.insert(&sender_id, &new_balance);
                self.fundings.insert(&item_index, &item_fundings);
                self.total_fundings.insert(&item_index, &item_total_funding);

                log!("Total for item {} is now at {}", item_index, item_total_funding);

                return PromiseOrValue::Value(U128::from(0));
            }
        }

        log!("Unable to fund item. Returning tokens.");
        PromiseOrValue::Value(amount)
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new();
        testing_env!(context.is_view(true).build());
    }

    #[test]
    fn test_new_item() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new();
        let sample_item_name = String::from("rolex");
        contract.new_item(sample_item_name.clone());
        assert_eq!(contract.items.get(&0), Some(sample_item_name));
    }
}
