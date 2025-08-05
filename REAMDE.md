# DeFi NFT Crowdfunding Protocol

## Description

The **DeFi NFT Crowdfunding Protocol** is a decentralized fundraising platform built on the **Solana** blockchain using the **Anchor** framework. It enables project creators to raise capital by issuing non‑fungible tokens (NFTs) that represent contributions to their campaigns. Investors or backers can fund projects by purchasing these NFTs and may receive access rights, perks or revenue‑sharing benefits tied to the success of the project. Funds are held in program‑derived escrow vaults until campaign milestones are met.

## Table of Contents

- [Features](#features)
- [Architecture Overview](#architecture-overview)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Running the Project](#running-the-project)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Features

This platform provides a rich set of capabilities designed to make on‑chain crowdfunding safe, transparent and accessible:

- **Campaign Creation:** Project creators can initialize campaigns with a funding goal, deadline, NFT supply and reward metadata.
- **Contribution & NFT Minting:** Contributors can fund campaigns using SOL/USDC and receive a unique NFT representing their stake.
- **Escrow Vaults:** Funds are held securely in escrow vaults until campaign goals are achieved.
- **Withdrawal & Refunds:** Logic enforced by smart contracts enables fund withdrawal after success or refunds upon failure.
- **Reward Distribution (Optional):** NFTs may grant holders access to future revenue or governance rights.
- **Modular Architecture:** Programs have well-separated responsibilities and communicate via CPIs.

## Architecture Overview

### Campaign Program

Manages campaigns, stores metadata, and controls fund withdrawals.

### Contribution Program

Handles user contributions, updates campaign totals, and invokes NFT minting.

### NFT Minting Program

Mints NFTs using Metaplex standards and stores campaign-specific metadata.

### Reward/Revenue Program

Distributes post-campaign revenues to NFT holders.

## Prerequisites

- Rust & Cargo
- Solana CLI (v1.16+)
- Anchor CLI (v0.29+)
- Node.js (v18+)
- Git

## Installation

```bash
git clone https://github.com/hamza-Hb17/capstone-nft-crowdfund.git
cd capstone-nft-crowdfunding
# Install dependencies
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
npm install
```

## Running the Project

```bash
solana-test-validator -r --reset
anchor build
anchor deploy
cd web && npm run dev
```

## Usage

- **Create Campaign:** Fill form and initialize campaign on-chain.
- **Contribute:** Select campaign, contribute funds, and receive NFT.
- **Withdraw/Refund:** Claim funds or receive refunds based on campaign result.

## Contributing

1. Fork the repo and create a new branch.
2. Submit a pull request with clear commit messages.

## License

MIT License

## Acknowledgments

Thanks to the Solana and Anchor developer communities for documentation, tools, and inspiration.
