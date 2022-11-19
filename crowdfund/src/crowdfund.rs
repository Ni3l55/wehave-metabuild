//! Module for representing a single crowdfund.

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, ext_contract, require, env, AccountId, BorshStorageKey, Balance, CryptoHash, PanicOnDefault, Promise, Gas, PromiseError, PromiseOrValue};
use near_sdk::collections::{UnorderedMap, UnorderedSet, Vector};
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;

use near_contract_standards::non_fungible_token::metadata::{TokenMetadata};
use near_contract_standards::non_fungible_token::{Token, TokenId};

use rust_decimal::Decimal;
use rust_decimal::prelude::*;

const TGAS: u64 = 1_000_000_000_000;
const DEFAULT_TOKEN_SUPPLY: u128 = 1_000_000;

#[ext_contract(ext_nft)]
trait NonFungibleToken {
    fn nft_mint(&mut self, token_metadata: TokenMetadata, ft_supply: U128, holders: Vec<AccountId>, shares: Vec<U128>);
}

#[ext_contract(ext_crowdfunds)]
trait Crowdfunds {
    fn nft_mint_callback(&mut self, item_index: u64);
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Crowdfund {
    // The account of the items collection used for tokenization
    nft_account_id: AccountId,

    // Identifier of this crowdfund
    identifier: u64,

    // The fee % to be paid on crowdfund
    item_fee_percentage: f64,

    // The metadata describing the item
    metadata: TokenMetadata,

    // The goal of funding
    goal: u128,

    // The fundings performed for this item (account -> USDC funded)
    fundings: UnorderedMap<AccountId, Balance>,

    // Overview of actual USDC fees paid per user (account -> USDC fees paid)
    fees_paid: UnorderedMap<AccountId, Balance>,

    // Total funding performed
    progress: u128,

    // The status of the crowdfund
    status: CrowdfundStatus
}

// Define storage keys for collections and nested collections
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Fundings { nested_hash: CryptoHash },
    FeesPaid { nested_hash: CryptoHash },
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

impl Crowdfund {
    pub fn new(nft_account_id: AccountId, identifier: u64, item_metadata: TokenMetadata, goal: u128, item_fee_percentage: f64) -> Self {
        Self {
            nft_account_id: nft_account_id,
            identifier: identifier,
            item_fee_percentage: item_fee_percentage,
            metadata: item_metadata,
            goal: goal,
            fundings: UnorderedMap::new(StorageKeys::Fundings { nested_hash: env::sha256_array(&identifier.to_be_bytes()) }),
            fees_paid: UnorderedMap::new(StorageKeys::FeesPaid { nested_hash: env::sha256_array(&identifier.to_be_bytes()) }),
            progress: 0u128,
            status: CrowdfundStatus::InProgress,
        }
    }

    pub fn get_metadata(&self) -> TokenMetadata {
        self.metadata.clone()
    }

    pub fn get_progress(&self) -> u128 {
        self.progress
    }

    pub fn get_goal(&self) -> u128 {
        self.goal
    }

    pub fn get_fee_percentage(&self) -> f64 {
        self.item_fee_percentage
    }

    pub fn set_status(&mut self, status: CrowdfundStatus) {
        self.status = status;
    }

    // Fund this crowdfund. Any leftover is returned
    pub fn fund(&mut self, sender_id: AccountId, amount: u128) -> u128 {
        require!(self.progress < self.goal, "The goal has already been reached for this item.");

        // Get the fee amount and the netto funding amount
        let (netto_amount, fee_amount) = self.split_netto_and_fee(amount);

        log!("Funding item {} for {} (Fee: {}) with progress {} and goal {}", self.identifier, netto_amount, fee_amount, self.progress, self.goal);

        // Get existing funds & fees from sender, or put on 0
        let mut funded_by_sender: Balance = self.fundings.get(&sender_id).unwrap_or_else(|| 0);
        let mut fees_paid_by_sender: Balance = self.fees_paid.get(&sender_id).unwrap_or_else(|| 0);

        // Calculate his new contributions
        let mut new_funded = funded_by_sender.checked_add(netto_amount.into()).unwrap();
        let mut new_fees_paid = fees_paid_by_sender.checked_add(fee_amount.into()).unwrap();

        if (self.progress + netto_amount) >= self.goal {
            log!("CROWFUNDING GOAL REACHED");

            // if this surpasses the limit, give back what's leftover: calculate how much fees to give back as well
            let (netto_leftover, fee_leftover) = self.calculate_leftovers(netto_amount, fee_amount);
            new_funded = new_funded - netto_leftover;
            new_fees_paid = new_fees_paid - fee_leftover;

            log!("He gave too much. Returning netto: {} and fee: {}", netto_leftover, fee_leftover);
            let leftover = netto_leftover + fee_leftover;

            // Save the funding that is performed (BEFORE! issuing the token)
            self.fundings.insert(&sender_id, &new_funded);
            self.progress = self.goal;

            // Save the fees that are paid
            self.fees_paid.insert(&sender_id, &new_fees_paid);

            log!("Initiating tokenization...");

            self.status = CrowdfundStatus::Transporting;
            // TODO don't immediately tokenize.
            // Tokenize to be called manually by us when item is acquired in warehouse

            self.tokenize_item();

            // Return leftover token
            return leftover;
        } else {
            // Save the funding that is performed
            self.fundings.insert(&sender_id, &new_funded);
            self.progress = self.progress + netto_amount;

            // Save the fees that are paid
            self.fees_paid.insert(&sender_id, &new_fees_paid);

            log!("Total for item {} is now at {}", self.identifier, self.progress);

            return 0u128;
        }
    }

    fn tokenize_item(&mut self) -> Promise {
        require!(self.progress == self.goal, "Goal not yet reached.");
        require!(!(self.status == CrowdfundStatus::Tokenized), "This item has already been tokenized."); // TODO only allow tokenization when transporting

        // TOKENIZE: call the custom NFT that creates a token
        log!("Serializing crowdfund distribution.");

        // Make crowdfund distribution serializable -> split funders (holders) & their funds (shares) into 2 Vec's
        let holders_serializable = self.fundings.keys_as_vector().to_vec();
        let shares = self.fundings.values_as_vector().to_vec();

        // Make shares serializable
        let mut shares_serializable: Vec<U128> = Vec::new();
        for share in &shares {
            shares_serializable.push(U128::from(*share));
        }

        // Call NFT mint on nft contract, pass new fungible token info
        log!("Calling nft_mint from crowdfund.");

        ext_nft::ext(self.nft_account_id.clone())
            .with_static_gas(Gas(10*TGAS))
            .nft_mint(self.metadata.clone(), U128::from(DEFAULT_TOKEN_SUPPLY), holders_serializable, shares_serializable)
            .then(
                ext_crowdfunds::ext(env::current_account_id())    // Callback to this crowdfund contract
                    .with_static_gas(Gas(1*TGAS))
                    .nft_mint_callback(self.identifier)
                )
    }

    fn split_netto_and_fee(&self, amount: u128) -> (u128, u128) {
        let item_fee_percentage_dec: Decimal = Decimal::from_f64(self.item_fee_percentage).unwrap();
        let amount_dec: Decimal = amount.into();

        let fees_dec = amount_dec * (item_fee_percentage_dec / Decimal::from(100));
        let fee_amount = fees_dec.to_u128().unwrap();
        let netto_amount: u128 = amount - fee_amount;

        (netto_amount, fee_amount)
    }

    fn calculate_leftovers(&self, netto_amount: u128, fee_amount: u128) -> (u128, u128) {
        let netto_leftover = self.progress + netto_amount - self.goal;

        let netto_leftover_dec: Decimal = netto_leftover.into();
        let netto_amount_dec: Decimal = netto_amount.into();

        let percentage_returned_dec: Decimal = netto_leftover_dec / netto_amount_dec;

        let fee_amount_dec: Decimal = fee_amount.into();
        let fee_leftover_dec: Decimal = fee_amount_dec * (percentage_returned_dec / Decimal::from(100));

        let fee_leftover: u128 = fee_leftover_dec.to_u128().unwrap();

        (netto_leftover, fee_leftover)
    }
}
