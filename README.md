# wehave-metabuild
Repository for the WeHave submission of the NEAR Metabuild hackathon.

WeHave is a platform for tokenizing exclusive items. You can crowdfund, own, trade and govern exclusive items.
People can own an NEP-141 (fungible) token that represents their share of the item.

We continued upon our submission from NEARCON, where we won the Future of Finance track!

This repository contains 4 smart contracts. Explained in more detail below, but a quick summary: there's one for crowdfunding, one for keeping a collection of the item tokens, then there's the token of an item itself, and also a DAO for governing the item.
A frontend is also included, which is connected to all of the smart contracts, on the testnet.

Project & Code explanation
==========================

Basic process:
Crowdfunding --> Tokenize by minting NFT --> Item Token created + item DAO created --> NFT actually minted

1. The crowdfunding smart contract code lives in the `/crowdfund` folder. It accepts USDC payments, and triggers adding an item to the collection of item tokens (collection = NFT) when a crowdfund goal has been reached.
2. The collection/NFT smart contract code lives in the `/nft` folder. It's a customized NFT which acts as an item token/DAO factory. If you mint, a new custom NEP-141 item token AND lightweight DAO gets created. (MINT = new tokenization of physical item).
3. The custom NEP-141 item token smart contract code lives in the `/ft` folder. When created, it takes the shares of the crowdfund. The supply is immediately distributed amongst the crowdfunders respectively.
4. There's a fake usdc contract in `/fake-usdc-ft` used for testing. You can ignore this.
5. The smart contract integration tests live in the `/integration-tests` directory.
6. The frontend code lives in the `/frontend` folder.

Deployments
===========

The project is live at `app-wehave-io.vercel.app`.
Crowdfunding is deployed at `crowdfunds3-wehave.testnet`. NFT collection is deployed at `items3-wehave.testnet`.

Quick Start
===========

If you haven't installed dependencies during setup:

    npm install

Build the smart contracts:

    npm run build

To integration test the smart contracts:

    npm run test:integration

For running the frontend:

    cd frontend && npm run dev
