use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, ext_contract, require, env, AccountId, BorshStorageKey, Balance, CryptoHash, PanicOnDefault, Promise, Gas, PromiseError, PromiseOrValue};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;

const TGAS: u64 = 1_000_000_000_000;

// Define the state of the smart contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // The NEP-141 item contract that this DAO is about
    item_account_id: AccountId,

    // The proposals that can be voted on [should we sell, should we lend, ...]
    proposals: UnorderedMap<u64, String>,

    // Per proposal the possible options [0 -> yes, no; 1 -> ok, maybe, idk]
    options: UnorderedMap<u64, Vector<String>>,

    // Votes that were cast for each proposal [0 -> ("yes", 0x5), ("no", root.near) ]
    votes: UnorderedMap<u64, Vector<(String, AccountId)>>
}

// Define storage keys for lists
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Proposals,
    Options { proposal_index_hash: CryptoHash },
    Votes { proposal_index_hash: CryptoHash}
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(item: AccountId) -> Self {
        require!(!env::state_exists(), "Already initialized");

        Self{

        }
    }

    // Add a new proposal to vote upon
    pub fn new_proposal(&mut self, question: String, answers: Vec<String>) {

    }

    // Cast a vote
    pub fn cast_vote(&mut self, ) {

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
