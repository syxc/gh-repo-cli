#!/bin/bash
# Package gh-repo-cli for distribution

set -e

VERSION="1.0.0"
PACKAGE_NAME="gh-repo-cli-v${VERSION}"
OUTPUT_DIR="/Users/syxc/.workany/sessions/20260202143307_glm-coding-plan-lite-search-mcp"
PACKAGE_DIR="${OUTPUT_DIR}/${PACKAGE_NAME}"

echo "📦 Packaging gh-repo-cli v${VERSION}..."

# Create temporary directory
rm -rf "${PACKAGE_DIR}"
mkdir -p "${PACKAGE_DIR}"

# Copy all files
cp -r . "${PACKAGE_DIR}/"

# Remove unnecessary files
cd "${PACKAGE_DIR}"
rm -rf node_modules .git gh-repo-cli-v*.zip
rm -f package.sh

# Create zip
cd "${OUTPUT_DIR}"
zip -r "${PACKAGE_NAME}.zip" "${PACKAGE_NAME}" -q

# Generate checksum
shasum -a 256 "${PACKAGE_NAME}.zip" > "${PACKAGE_NAME}.zip.sha256"

# Cleanup
rm -rf "${PACKAGE_DIR}"

echo "✅ Package created: ${PACKAGE_NAME}.zip"
echo "📋 Checksum: $(cat ${PACKAGE_NAME}.zip.sha256)"
echo "📊 Size: $(du -h ${PACKAGE_NAME}.zip | cut -f1)"
