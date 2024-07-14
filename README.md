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
   git clone https://github.com/luckysitara/dao-voting-program.git
   cd dao-voting-program
   ```

2. **Install Dependencies**:
   ```bash
   npm install
   ```

3. **Compile Circuits**:
   ```bash
   circom circuits/vote.circom --r1cs --wasm --sym --c
   snarkjs setup --protocol groth
   snarkjs groth16 setup circuits/vote.r1cs circuits/vote_0000.zkey circuits/vote_final.zkey
   snarkjs zkey export verificationkey circuits/vote_final.zkey circuits/verification_key.json
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

## File Structure

- **app/**: Solana programs and Anchor configuration.
- **circuits/**: ZK proof circuits and keys.
- **src/**: Frontend React application.
  - **components/**: React components for voting and tallying results.
  - **zk_utils.js**: Utilities for generating and verifying ZK proofs.

## Contributions

Feel free to contribute by opening issues or submitting pull requests.

## License

This project is licensed under the MIT License.
