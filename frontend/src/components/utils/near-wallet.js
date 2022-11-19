// near api js
import { providers } from 'near-api-js';

// wallet selector UI
import '@near-wallet-selector/modal-ui/styles.css';
import { setupModal } from '@near-wallet-selector/modal-ui';
import LedgerIconUrl from '@near-wallet-selector/ledger/assets/ledger-icon.png';
import MyNearIconUrl from '@near-wallet-selector/my-near-wallet/assets/my-near-wallet-icon.png';

// wallet selector options
import { setupWalletSelector } from '@near-wallet-selector/core';
import { setupLedger } from '@near-wallet-selector/ledger';
import { setupMyNearWallet } from '@near-wallet-selector/my-near-wallet';

const THIRTY_TGAS = '30000000000000';
const NO_DEPOSIT = '0';
const FAKE_USDC_CONTRACT = "usdc.fakes.testnet";

// Wallet that simplifies using the wallet selector
export class Wallet {
  walletSelector;
  wallet;
  accountId;
  crowdfundsContractId;
  itemsContractId;

  constructor({crowdfundsContractId, itemsContractId}){
    this.crowdfundsContractId = crowdfundsContractId;
    this.itemsContractId = itemsContractId;
  }

  // To be called when the website loads
  async startUp() {
    console.log("initializing");
    this.walletSelector = await setupWalletSelector({
      network: 'testnet',
      modules: [setupMyNearWallet(),
        setupLedger()],
    });

    const isSignedIn = this.walletSelector.isSignedIn();

    if (isSignedIn) {
      const { accounts } = this.walletSelector.store.getState();

      this.wallet = await this.walletSelector.wallet();
      this.accountId = accounts[0].accountId;
    }

    return isSignedIn;
  }

  walletSignedIn() {
    if (this.walletSelector == null) {
      return false;
    }

    return this.walletSelector.isSignedIn();
  }

  // Sign-in method
  signIn() {
    const description = 'Please select a wallet to sign in.';
    const modal = setupModal(this.walletSelector, { contractId: this.crowdfundsContractId, description });
    modal.show();
  }

  // Sign-out method
  signOut() {
    this.wallet.signOut();
    this.wallet = this.accountId = this.crowdfundsContractId = this.itemsContractId = null;
    window.location.replace(window.location.origin + window.location.pathname);
  }

  // --------- CROWDFUNDS ---------

  async createCrowdfund(name, accountName, description, goal, imgUrl, metadataUrl) {
    // Assumes metadata + image is already on IPFS, on a URL
    // Parse into token_metadata

    let token_metadata = {
      "title": name,
      "description": description,
      "extra": accountName,
      "media": imgUrl,
      "reference": metadataUrl
    };

    return await this.callMethod({contractId: this.crowdfundsContractId, method: 'new_item', args:{item_metadata: token_metadata, goal: goal}});
  }

  async getCurrentCrowdfunds() {
    return await this.viewMethod({contractId: this.crowdfundsContractId, method: 'get_current_items'});
  }

  async getCrowdfundProgress(itemIndex){
    return await this.viewMethod({contractId: this.crowdfundsContractId, method: 'get_crowdfund_progress', args:{item_index: itemIndex}});
  }

  async getCrowdfundGoal(itemIndex) {
    return await this.viewMethod({contractId: this.crowdfundsContractId, method: 'get_crowdfund_goal', args:{item_index: itemIndex}});
  }

  async fundUSDC(itemIndex, amount) {
    return await this.callMethod({contractId: FAKE_USDC_CONTRACT, method: 'ft_transfer_call', args:{receiver_id: this.crowdfundsContractId, amount: amount, memo: "funding", msg: itemIndex}, gas: '300000000000000', deposit: '1'})
  }

  // --------- ITEMS ---------

  async getSingleTokenfromNFT(tokenId) {
    return await this.viewMethod({contractId: this.itemsContractId, method: 'nft_token', args:{token_id: tokenId.toString()}});
  }

  // --------- ITEM TOKENS ---------

  async claimTokens(ft_account_prefix) {
    let contract = ft_account_prefix + "." + this.itemsContractId;
    return await this.callMethod({contractId: contract, method: 'storage_deposit', args:{account_id: this.accountId}, gas: '300000000000000', deposit: '1250000000000000000000'});  // 0.00125 N
  }

  async getFTTotalSupply(contract) {
    return await this.viewMethod({contractId: contract, method: 'ft_total_supply'});
  }

  async getFTUserBalance(contract, user) {
    return await this.viewMethod({contractId: contract, method: 'ft_balance_of', args:{account_id: user}});
  }

  // --------- GOVERNANCE ---------

  async getProposals(contract) {
    return await this.viewMethod({contractId: contract, method: 'get_proposals'});
  }

  async getProposalVotes(contract, proposal_index) {
    return await this.viewMethod({contractId: contract, method: 'get_proposal_votes', args:{proposal_index: proposal_index}});
  }

  async createProposal(contract, question, options) {
    return await this.callMethod({contractId: contract, method: 'new_proposal', args:{question: question, options: options}});
  }

  async voteForProposal(contract, proposalIndex, optionIndex) {
    return await this.callMethod({contractId: contract, method: 'cast_vote', args:{proposal_index: proposalIndex, answer_index: optionIndex}, gas: '300000000000000'});
  }

  // ------------------------------

  // Make a read-only call to retrieve information from the network
  async viewMethod({ contractId, method, args = {} }) {
    const { network } = this.walletSelector.options;
    const provider = new providers.JsonRpcProvider({ url: network.nodeUrl });

    let res = await provider.query({
      request_type: 'call_function',
      account_id: contractId,
      method_name: method,
      args_base64: Buffer.from(JSON.stringify(args)).toString('base64'),
      finality: 'optimistic',
    });

    console.log(res);

    return JSON.parse(Buffer.from(res.result).toString());
  }

  // Call a method that changes the contract's state
  async callMethod({ contractId, method, args = {}, gas = THIRTY_TGAS, deposit = NO_DEPOSIT }) {
    const { accountId } = this.walletSelector.store.getState().accounts[0];

    // Sign a transaction with the "FunctionCall" action
    return await this.wallet.signAndSendTransaction({
      signerId: accountId,
      receiverId: contractId,
      actions: [
        {
          type: 'FunctionCall',
          params: {
            methodName: method,
            args,
            gas,
            deposit,
          },
        },
      ],
    });
  }

  // Get transaction result from the network
  async getTransactionResult(txhash) {
    const { network } = this.walletSelector.options;
    const provider = new providers.JsonRpcProvider({ url: network.nodeUrl });

    // Retrieve transaction result from the network
    const transaction = await provider.txStatus(txhash, 'unnused');
    return providers.getTransactionLastResult(transaction);
  }
}
