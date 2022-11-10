use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, ext_contract, require, env, AccountId, BorshStorageKey, Balance, CryptoHash, PanicOnDefault, Promise, Gas, PromiseError, PromiseOrValue};
use near_sdk::collections::{UnorderedMap, UnorderedSet, Vector};
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;

use near_contract_standards::non_fungible_token::metadata::{TokenMetadata};
use near_contract_standards::non_fungible_token::{Token, TokenId};

use percentage::Percentage;

const TGAS: u64 = 1_000_000_000_000;
const DEFAULT_TOKEN_SUPPLY: u128 = 1_000_000;

// Define the state of the smart contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // TODO Should have implemented this by creating Crowdfund struct, then create Vector of crowdfunds :(

    // The base uri to find more info about the crowdfund item
    base_uri: String,

    // The stablecoin accepted as payment
    accepted_coin: AccountId,

    // The account id of the items collection used for tokenization
    nft_account_id: AccountId,

    // The default fee % taken on crowdfunds
    default_fee_percentage: f64,

    // The fee % to be paid per item
    item_fee_percentage: Vector<f64>,

    // Different crowdfunded items, kept as metadata for minting later
    items: Vector<TokenMetadata>,

    // Crowdfund goal per item (indexed balance)
    goals: Vector<u128>,

    // Overview of fundings per item (index -> account -> USDC funded)
    fundings: Vector<UnorderedMap<AccountId, Balance>>,

    // Overview of actual USDC fees paid per item (index -> account -> USDC fees paid)
    fees_paid: Vector<UnorderedMap<AccountId, Balance>>,

    // Summary of total funding per item for (indexed balance)
    total_fundings: Vector<u128>,

    // Per crowdfund, the status it's in
    status: Vector<CrowdfundStatus>,

    // List of accounts allowed to create, delete a crowdfund
    crowdfund_operators: Vector<AccountId>
}

// Define storage keys for collections and nested collections
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    ItemFees,
    Items,
    Goals,
    Fundings,
    Subfunding { item_index_hash: CryptoHash },
    FeesPaid,
    SubFeesPaid { item_index_hash: CryptoHash},
    TotalFundings,
    Status,
    CrowdfundOperators
}

// See smart contract documentation for the meaning of all these
#[derive(BorshStorageKey, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum CrowdfundStatus {
    Created,
    Rejected,
    InProgress,
    OutOfTime,
    Buying,
    FailedBuying,
    Transporting,
    FailedTransporting,
    Tokenized
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
            accepted_coin: accepted_coin,
            nft_account_id: nft_account_id,
            default_fee_percentage: 4.0,
            item_fee_percentage: Vector::new(StorageKeys::ItemFees),
            items: Vector::new(StorageKeys::Items),
            goals: Vector::new(StorageKeys::Goals),
            fundings: Vector::new(StorageKeys::Fundings),
            fees_paid: Vector::new(StorageKeys::FeesPaid),
            total_fundings: Vector::new(StorageKeys::TotalFundings),
            status: Vector::new(StorageKeys::Status),
            crowdfund_operators: Vector::new(StorageKeys::CrowdfundOperators),
        }
    }

    pub fn new_item(&mut self, item_metadata: TokenMetadata, goal: u128) {
        require!(self.caller_is_operator(), "Caller is not allowed to create a crowdfund.");
        require!(goal > 0, "Goal is smaller than zero.");

        self.items.push(&item_metadata);    // TODO Check / set extra field in metadata as UID

        // Create goal for item
        self.goals.push(&goal);

        // Create fees for item
        self.item_fee_percentage.push(&self.default_fee_percentage);

        // Instantiate the funding map for this item
        let amt = u128::from(self.fundings.len());
        self.fundings.push(&UnorderedMap::new(StorageKeys::Subfunding {
                                    item_index_hash: env::sha256_array(&amt.to_be_bytes()),
                                })
                            );

        // Instantiate the fees map for this item
        self.fees_paid.push(&UnorderedMap::new(StorageKeys::SubFeesPaid {
                                    item_index_hash: env::sha256_array(&amt.to_be_bytes()),
                                })
                            );

        // Instantiate the total funding for this item
        let start: u128 = 0;
        self.total_fundings.push(&start);

        // Instantiate the status for this item TODO put in created first
        let crowdfund_status = CrowdfundStatus::InProgress;
        self.status.push(&crowdfund_status);
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
        self.items.to_vec()
    }

    pub fn get_crowdfund_progress(&self, item_index: u64) -> u128 {
        self.total_fundings.get(item_index).expect("Incorrect item index!")
    }

    pub fn get_crowdfund_fee_percentage(&self, item_index: u64) -> f64 {
        self.item_fee_percentage.get(item_index).expect("Incorrect item index!")
    }

    pub fn get_crowdfund_goal(&self, item_index: u64) -> u128 {
        self.goals.get(item_index).expect("Incorrect item index!")
    }

    pub fn tokenize_item(&mut self, item_index: u64) -> Promise {
        let crowdfund_progress = self.get_crowdfund_progress(item_index);
        let crowdfund_goal = self.get_crowdfund_goal(item_index);

        require!(crowdfund_progress == crowdfund_goal, "Goal not yet reached.");
        require!(!(self.status.get(item_index).unwrap() == CrowdfundStatus::Tokenized), "This item has already been tokenized."); // TODO only allow tokenization when transporting

        // TOKENIZE: call the custom NFT that creates a token
        log!("Serializing crowdfund distribution.");

        let item_fundings = self.fundings.get(item_index).unwrap();

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

        let token_metadata = self.items.get(item_index).unwrap();

        ext_nft::ext(self.nft_account_id.clone())
            .with_static_gas(Gas(10*TGAS))
            .nft_mint(token_metadata.clone(), U128::from(DEFAULT_TOKEN_SUPPLY), holders_serializable, shares_serializable)
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(1*TGAS))
                    .nft_mint_callback(item_index)
                )
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
            self.status.replace(item_index, &CrowdfundStatus::Tokenized);
        }
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128> {
        require!(env::predecessor_account_id() == self.accepted_coin, "This coin is not accepted as payment.");
        let item_index: u64 = msg.parse().unwrap();

        // Check crowdfund information
        let item_progress = self.get_crowdfund_progress(item_index);
        let goal = self.goals.get(item_index).unwrap();
        require!(item_progress < goal, "The goal has already been reached for this item.");

        // Substract the fee from the amount
        let fee_percentage = Percentage::from(self.item_fee_percentage);
        let fee_amount = fee_percentage.apply_to(amount);
        let netto_amount = amount - fee_amount;

        log!("Funding item {} for {} (Fee: {}) with progress {} and goal {}", item_index, netto_amount, fee_amount, item_progress, goal);

        // Get existing funds & fees from sender, or put on 0
        let mut item_fundings = self.fundings.get(item_index).unwrap();
        let mut item_fees_paid = self.fees_paid.get(item_index).unwrap();
        let mut funded_by_sender: Balance = item_fundings.get(&sender_id).unwrap_or_else(|| 0);
        let mut fees_paid_by_sender: Balance = item_fees_paid.get(&sender_id).unwrap_or_else(|| 0);

        // Calculate his new contributions
        let mut new_funded = funded_by_sender.checked_add(netto_amount.into()).unwrap();
        let mut new_fees_paid = fees_paid_by_sender.checked_add(fee_amount.into()).unwrap();

        if (item_progress + u128::from(netto_amount)) >= goal {
            log!("CROWFUNDING GOAL REACHED");

            // if this surpasses the limit, give back what's leftover: calculate how much fees to give back as well
            let netto_leftover = item_progress + u128::from(netto_amount) - goal;
            new_funded = new_funded - netto_leftover;

            let fee_leftover = Percentage::from(netto_leftover/netto_amount).apply_to(fee_amount);
            new_fees_paid = new_fees_paid - fee_leftover;

            let leftover = netto_leftover + fee_leftover;

            // Save the funding that is performed (BEFORE! issuing the token)
            item_fundings.insert(&sender_id, &new_funded);
            self.fundings.replace(item_index, &item_fundings);
            self.total_fundings.replace(item_index, &goal);

            // Save the fees that are paid
            item_fees_paid.insert(&sender_id, &new_fees_paid);
            self.fees_paid.replace(item_index, &item_fees_paid);

            log!("Initiating tokenization...");

            self.status.replace(item_index, &CrowdfundStatus::Transporting);
            // TODO don't immediately tokenize.
            // Tokenize to be called manually by us when item is acquired in warehouse

            self.tokenize_item(item_index.clone());

            // Return leftover token
            return PromiseOrValue::Value(U128::from(leftover));
        } else {
            let mut item_total_funding = self.total_fundings.get(item_index).unwrap();
            item_total_funding = item_progress + u128::from(netto_amount);

            // Save the funding that is performed
            item_fundings.insert(&sender_id, &new_funded);
            self.fundings.replace(item_index, &item_fundings);
            self.total_fundings.replace(item_index, &item_total_funding);

            // Save the fees that are paid
            fees_paid.insert(&sender_id, &new_fees_paid);
            self.fees_paid.replace(item_index, &fees_paid);

            log!("Total for item {} is now at {}", item_index, item_total_funding);

            return PromiseOrValue::Value(U128::from(0));
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
