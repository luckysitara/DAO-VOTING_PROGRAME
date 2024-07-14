# DAO-VOTING_PROGRAME
This repository contains a decentralized autonomous organization (DAO) voting program built using Solana and Anchor. The program allows users to vote on proposals and display results. Key features include:

- **Voting System**: Users can vote on proposals with a simple yes/no interface.
- **Privacy Voting**: Implemented using Zero-Knowledge (ZK) proofs to ensure voter privacy.
- **Participation Rewards**: Users receive reward points for participating in votes.

## Features

- **Smart Contracts**: Solana programs written in Rust.
- **Zero-Knowledge Proofs**: Circom and SnarkJS integration for privacy.
- **Frontend**: React-based UI for interacting with the voting system.

## Getting Started

### Prerequisites

- Node.js and npm
- Solana CLI
- Anchor CLI

### Installation

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/luckysitara/dao-voting_program.git
   cd dao-voting_program
   ```

2. **Install Dependencies**:
   ```bash
   npm install
   ```

3. **Compile Circuits**:
   ```bash
   chmod +x install-depend.sh

   ```

4. **Deploy Solana Program**:
   ```bash
   anchor build
   anchor deploy
   ```

5. **Run Frontend**:
   ```bash
   npm start
   ```



## Contributions

Feel free to contribute by opening issues or submitting pull requests.

## License

This project is licensed under the MIT License.
