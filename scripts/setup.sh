#!/bin/bash

set -e

echo "Setting up Funding Rate Calculation System..."

# Check for Rust
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Check for Solana CLI
if ! command -v solana &> /dev/null; then
    echo "Installing Solana CLI..."
    sh -c "$(curl -sSfL https://release.solana.com/v2.0.20/install)"
    export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
fi

# Check for Anchor
if ! command -v anchor &> /dev/null; then
    echo "Installing Anchor..."
    cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
    avm install 0.30.1
    avm use 0.30.1
fi

# Install sqlx-cli
if ! command -v sqlx &> /dev/null; then
    echo "Installing sqlx-cli..."
    cargo install sqlx-cli --no-default-features --features postgres
fi

# Create .env if it doesn't exist
if [ ! -f .env ]; then
    echo "Creating .env file..."
    cp .env.example .env || echo "Please create .env file manually"
fi

echo "Setup complete!"
echo "Next steps:"
echo "1. Edit .env file with your configuration"
echo "2. Run 'make docker-up' to start services"
echo "3. Run 'make migrate' to apply database migrations"
echo "4. Run 'make build' to build the project"

