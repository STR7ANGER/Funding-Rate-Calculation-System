#!/bin/bash

set -e

NETWORK=${1:-devnet}

echo "Deploying Anchor program to $NETWORK..."

if [ "$NETWORK" = "devnet" ]; then
    solana config set --url https://api.devnet.solana.com
elif [ "$NETWORK" = "mainnet" ]; then
    solana config set --url https://api.mainnet-beta.solana.com
else
    echo "Invalid network. Use 'devnet' or 'mainnet'"
    exit 1
fi

# Check if wallet is funded
BALANCE=$(solana balance | grep -o '[0-9.]*' | head -1)
if (( $(echo "$BALANCE < 1.0" | bc -l) )); then
    echo "Warning: Wallet balance is low. You may need SOL for deployment."
    if [ "$NETWORK" = "devnet" ]; then
        echo "Requesting airdrop..."
        solana airdrop 2
    fi
fi

# Deploy
anchor deploy

echo "Deployment complete!"
echo "Update PROGRAM_ID in .env with the deployed program ID"

