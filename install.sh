#!/bin/bash

# Installation script for MCP Server Rust Template dependencies

echo "Installing MCP Server dependencies..."
echo ""

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✓ Cargo found"

# Install cargo-make if not already installed
if ! command -v cargo-make &> /dev/null; then
    echo "Installing cargo-make..."
    cargo install cargo-make
else
    echo "✓ cargo-make already installed"
fi

# Install worker-build if not already installed
if ! command -v worker-build &> /dev/null; then
    echo "Installing worker-build..."
    cargo install worker-build
else
    echo "✓ worker-build already installed"
fi

# Install wrangler if not already installed
if ! command -v wrangler &> /dev/null; then
    echo "Installing wrangler..."
    cargo install wrangler
else
    echo "✓ wrangler already installed"
fi

echo ""
echo "✅ All dependencies installed successfully!"
echo ""
echo "Next steps:"
echo "1. Run 'cargo make dev' to start the development server"
echo "2. Run 'cargo make test' to run tests"
echo "3. Run 'cargo make --list-all-steps' to see all available tasks"