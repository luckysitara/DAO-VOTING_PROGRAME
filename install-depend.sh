#!/usr/bin/env bash

echo "Installing additional dependencies..."
sudo apt-get install -y libssl-dev libudev-dev pkg-config zlib1g-dev llvm clang make

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "Rust is already installed."
fi

# Install Solana CLI
echo "Installing Solana CLI..."
sh -c "$(curl -sSfL https://release.solana.com/v1.9.0/install)"

# Install Anchor CLI
echo "Installing Anchor CLI..."
cargo install --git https://github.com/project-serum/anchor --tag v0.18.0 anchor-cli --locked

echo "Dependencies installation complete."
