mod crowdfund;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, ext_contract, require, env, AccountId, BorshStorageKey, Balance, CryptoHash, PanicOnDefault, Promise, Gas, PromiseError, PromiseOrValue};
use near_sdk::collections::{UnorderedMap, UnorderedSet, Vector};
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;

use near_contract_standards::non_fungible_token::metadata::{TokenMetadata};
use near_contract_standards::non_fungible_token::{Token, TokenId};

use crowdfund::Crowdfund;
use crowdfund::CrowdfundStatus;

const DEFAULT_TOKEN_DECIMALS: u8 = 6;

// Define the state of the smart contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // The base uri to find more info about crowdfunded items
    base_uri: String,

    // The number of decimals for interpreting the balance amounts
    decimals: u8,

    // The stablecoin accepted as payment for crowdfunds
    accepted_coin: AccountId,

    // The account id of the items collection used for tokenization
    nft_account_id: AccountId,

    // The default fee % taken on crowdfunds
    default_fee_percentage: f64,

    // The list of crowdfunds
    crowdfunds: Vector<Crowdfund>,

    // List of accounts allowed to create or delete a crowdfund
    crowdfund_operators: Vector<AccountId>
}

// Define storage keys for collections and nested collections
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Crowdfunds,
    CrowdfundOperators
}

#[ext_contract(ext_nft)]
trait NonFungibleToken {
    fn nft_mint(&mut self, token_metadata: TokenMetadata, ft_supply: U128, holders: Vec<AccountId>, shares: Vec<U128>);
}

pub trait FungibleTokenReceiver {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(nft_account_id: AccountId, accepted_coin: AccountId) -> Self {
        require!(!env::state_exists(), "Already initialized");

        Self{
            base_uri: String::from("test"),
            decimals: DEFAULT_TOKEN_DECIMALS,
            accepted_coin: accepted_coin,
            nft_account_id: nft_account_id,
            default_fee_percentage: 4.0,
            crowdfunds: Vector::new(StorageKeys::Crowdfunds),
            crowdfund_operators: Vector::new(StorageKeys::CrowdfundOperators),
        }
    }

    pub fn new_item(&mut self, item_metadata: TokenMetadata, goal: u128) {
        require!(self.caller_is_operator(), "Caller is not allowed to create a crowdfund.");
        require!(goal > 0, "Goal is smaller than zero.");

        let amt = u64::from(self.crowdfunds.len());
        let new_crowdfund = Crowdfund::new(self.nft_account_id.clone(), amt, item_metadata, goal, self.default_fee_percentage);

        self.crowdfunds.push(&new_crowdfund);
    }

    pub fn add_operator(&mut self, operator: AccountId) {
        require!(env::predecessor_account_id() == env::current_account_id(), "Only this contract itself can add an operator.");
        self.crowdfund_operators.push(&operator);
    }

    fn caller_is_operator(&self) -> bool {
        for operator in self.crowdfund_operators.iter() {
            if env::predecessor_account_id() == operator {
                return true;
            }
        }

        return false;
    }

    pub fn get_current_items(&self) -> Vec<TokenMetadata> {
        let mut metadata_list = vec!();
        let crowdfund_vec = self.crowdfunds.to_vec();

        for crowdfund in &crowdfund_vec {
            metadata_list.push(crowdfund.get_metadata());
        }

        metadata_list
    }

    pub fn get_crowdfund_progress(&self, item_index: u64) -> u128 {
        self.crowdfunds.get(item_index).expect("Incorrect item index!").get_progress()
    }

    pub fn get_crowdfund_fee_percentage(&self, item_index: u64) -> f64 {
        self.crowdfunds.get(item_index).expect("Incorrect item index!").get_fee_percentage()
    }

    pub fn get_crowdfund_goal(&self, item_index: u64) -> u128 {
        self.crowdfunds.get(item_index).expect("Incorrect item index!").get_goal()
    }

    #[handle_result]
    #[private]
    #[payable]
    pub fn nft_mint_callback(&mut self, item_index: u64, #[callback_result] call_result: Result<(), PromiseError>) {
        if call_result.is_err() {
            log!("Something went wrong during nft_mint.");
            // Decide what to do here
            // Potentially give back fundings
            // Or try again
        } else {
            log!("nft_mint was successful!");
            self.crowdfunds.get(item_index).unwrap().set_status(CrowdfundStatus::Tokenized);
        }
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128> {
        require!(env::predecessor_account_id() == self.accepted_coin, "This coin is not accepted as payment.");
        // TODO return the coin?

        // Check if the crowdfund is ilegible for funding TODO use status
        let item_index: u64 = msg.parse().unwrap();

        let mut crowdfund = self.crowdfunds.get(item_index).expect("Incorrect item index!");
        let leftover = crowdfund.fund(sender_id, u128::from(amount));

        self.crowdfunds.replace(item_index, &crowdfund);

        PromiseOrValue::Value(U128::from(leftover))
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, Balance};

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
        let contract = Contract::new("test.near".parse().unwrap());
        testing_env!(context.is_view(true).build());
    }

    #[test]
    fn test_new_item() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new("test.near".parse().unwrap());
        let sample_item_name = String::from("rolex");
        contract.new_item(sample_item_name.clone(), 1000);
        assert_eq!(contract.items.get(&0), Some(sample_item_name));
        let the_vec: Vec<String> = vec!(String::from("rolex"));
        assert_eq!(contract.get_current_items(), the_vec);
    }
}
