# Predix - Cross-Chain Betting Protocol

## Overview
Predix is a decentralized cross-chain betting protocol built on the Solana blockchain with Wormhole interoperability. It enables seamless cross-chain betting and secure tokenized rewards using advanced smart contract mechanisms.

Predix leverages Anchor, Wormhole, and the Solana blockchain to create a trustless, efficient, and transparent betting ecosystem.

---

## Features

### ✅ Cross-Chain Betting with Wormhole
- Users can place bets across different chains through verified messages from Wormhole.
- The betting process ensures message integrity and authenticity using Wormhole's `Posted VAA`.

### ✅ On-Chain Betting System
- Secure betting directly on Solana with SPL token support.
- Market-based betting where users bet on predefined outcomes.

### ✅ Secure Token Transfers
- Uses Anchor’s `Associated Token Accounts` and `Token2022` for managing bets securely.
- Implements secure token transfers between users and market vaults with transaction fee handling.

### ✅ Reward Distribution
- Winners receive automated payouts based on betting pool distribution.
- Reward APRs are calculated based on market conditions and distributed accordingly.

---

## How Predix Uses Solana & Wormhole

### 🔹 Cross-Chain Interoperability with Wormhole
- Messages from external chains are verified through Wormhole’s `Posted VAA`.
- Uses the `ForeignEmitter` account to validate messages from trusted chains.

### 🔹 Trustless Betting System
- Smart contracts ensure the integrity of each bet.
- Betting is only allowed on approved and existing markets.

### 🔹 Secure & Efficient Token Management
- Users deposit SPL tokens into betting vaults.
- Fees and rewards are managed transparently using Anchor’s token program.

### 🔹 Decentralized Reward Claiming
- Winning bets are verified based on market outcomes.
- Users can claim rewards without intermediaries.

---

## Key Technologies
- **Programming Language**: Rust
- **Blockchain**: Solana
- **Smart Contract Framework**: [Anchor](https://project-serum.github.io/anchor/)
- **Interoperability**: [Wormhole](https://wormhole.com/)
- **Token Standards**:
  - SPL Tokens (`Token2022`)
  - `Associated Token Accounts`

---

## Environment Setup
Ensure the following environment variables are configured:
```env
RPC_URL=<your_sol_rpc_url>
WORMHOLE_PROGRAM=<wormhole_program_address>
```

---

## Installation

### Clone the Repository
```bash
git clone <repository-url>
cd <repository-directory>
```

### Install Dependencies
```bash
anchor build
```

### Deploy Smart Contracts
```bash
anchor deploy
```

---

## Usage

### 📌 Placing a Cross-Chain Bet
```rust
let result = bet_cross_chain(ctx, answer_key, vaa_hash);
```

### 📌 Placing an On-Chain Bet
```rust
let result = bet(ctx, answer_key, amount);
```

### 📌 Claiming Rewards
```rust
let result = claim_token(ctx);
```

---

## Contribution
Contributions are welcome! Feel free to open issues and submit pull requests.

---

## License
This project is licensed under the MIT License.

