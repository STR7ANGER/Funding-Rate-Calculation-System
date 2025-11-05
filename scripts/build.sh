#!/bin/bash

set -e

echo "Building Funding Rate Calculation System..."

# Build Anchor program
if command -v anchor &> /dev/null; then
    echo "Building Anchor program..."
    anchor build || echo "Anchor build skipped (Anchor not installed)"
else
    echo "Anchor not found, skipping Anchor build"
fi

# Build backend
echo "Building backend..."
cargo build --release

echo "Build complete!"

