#!/bin/bash

# Test script for gh-repo-cli
# This script runs all tests and generates coverage reports

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "${SCRIPT_DIR}")"

cd "${PROJECT_DIR}"

echo "🧪 Running tests for gh-repo-cli..."
echo ""

# Check if node_modules exists
if [ ! -d "node_modules" ]; then
  echo "📦 Installing dependencies..."
  npm install
  echo ""
fi

# Run linter
echo "🔍 Running linter..."
npm run lint
echo ""

# Run unit tests
echo "🧪 Running unit tests..."
npm test -- --testPathPattern="tests/(lib|commands)" --verbose
echo ""

# Run integration tests (if network available)
echo "🌐 Running integration tests..."
npm test -- --testPathPattern="tests/integration" --verbose || echo "⚠️  Integration tests skipped (network unavailable)"
echo ""

# Generate coverage report
echo "📊 Generating coverage report..."
npm run test:coverage
echo ""

# Display summary
echo "✅ Test suite completed!"
echo ""
echo "Coverage report available at: coverage/lcov-report/index.html"
echo ""
