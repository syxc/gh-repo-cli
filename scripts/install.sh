#!/bin/bash
# Installation script for gh-repo-cli

set -e

echo "🚀 Installing gh-repo-cli..."

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Error: Node.js is not installed"
    echo "Please install Node.js from https://nodejs.org/"
    exit 1
fi

# Install dependencies
echo "📦 Installing dependencies..."
npm install

# Link globally
echo "🔗 Linking globally..."
npm link

# Verify installation
echo "✅ Verifying installation..."
if command -v gh &> /dev/null; then
    echo "✅ Installation successful!"
    echo ""
    echo "Run 'gh --version' to verify"
    echo "Run 'gh --help' to see all commands"
else
    echo "❌ Installation failed"
    exit 1
fi
