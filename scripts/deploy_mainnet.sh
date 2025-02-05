#!/bin/bash

# DankNet Mainnet Deployment Script
set -e

# Load environment
source .env.production

# Deploy Programs
echo "🚀 Deploying Core Contracts..."
anchor deploy --provider.cluster mainnet \
    --program-name dank_token \
    --program-id DANKm3m3m3m3m3m3m3m3m3m3m3m3m3m3m3m3m3m3m3m3

anchor deploy --provider.cluster mainnet \
    --program-name dank_governance \
    --program-id GOVERm3m3m3m3m3m3m3m3m3m3m3m3m3m3m3m3m3m3

# Verify Bytecode
echo "🔍 Verifying Deployments..."
solana program dump DANKm3m3m3m3m3m3m3m3m3m3m3m3m3m3m3m3m3m3m3m3 dank_token.so
solana program dump GOVERm3m3m3m3m3m3m3m3m3m3m3m3m3m3m3m3m3m3 governance.so

shasum -a 256 dank_token.so governance.so > .deployment_checksums

# Initialize Liquidity Pools
echo "💧 Setting Up Liquidity..."
ts-node scripts/init_pools.ts \
    --keypair ~/.config/solana/mainnet-keypair.json \
    --rpc $SOLANA_RPC \
    --amount 1000000

echo "✅ Deployment Complete! Stay DANK 🤪"
