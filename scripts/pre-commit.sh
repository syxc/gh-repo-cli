#!/bin/bash

# Pre-commit hook for gh-repo-cli
# Run this script before committing to ensure code quality

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "${SCRIPT_DIR}")"

cd "${PROJECT_DIR}"

echo "🔍 Pre-commit checks..."
echo ""

# Check for console.log statements (except in tests)
echo "🧹 Checking for console.log..."
CONSOLE_LOGS=$(git diff --cached --name-only | grep -E '\.js$' | grep -v test | xargs grep -l 'console\.log' || true)
if [ -n "$CONSOLE_LOGS" ]; then
  echo "⚠️  Warning: console.log found in:"
  echo "$CONSOLE_LOGS"
  echo ""
fi

# Run linter on staged files
echo "📝 Running linter on staged files..."
STAGED_JS=$(git diff --cached --name-only --diff-filter=ACM | grep -E '\.js$' || true)
if [ -n "$STAGED_JS" ]; then
  echo "$STAGED_JS" | xargs npm run lint -- || true
fi

# Run tests
echo "🧪 Running tests..."
npm test -- --passWithNoTests
echo ""

echo "✅ Pre-commit checks passed!"
echo ""
