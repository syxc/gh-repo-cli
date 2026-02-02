#!/bin/bash
# Package gh-repo-cli for distribution

set -e

# Accept version as argument or use default
VERSION="${1:-1.0.0}"
PACKAGE_NAME="gh-repo-cli-v${VERSION}"
# Use parent directory of script as output directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_DIR="$(dirname "${SCRIPT_DIR}")"
PACKAGE_DIR="${OUTPUT_DIR}/${PACKAGE_NAME}"

echo "📦 Packaging gh-repo-cli v${VERSION}..."

# Create temporary directory
rm -rf "${PACKAGE_DIR}"
mkdir -p "${PACKAGE_DIR}"

# Copy all files (exclude package directory itself)
rsync -av --exclude='node_modules' --exclude='.git' --exclude='gh-repo-cli-v*.zip' --exclude="${PACKAGE_NAME}" . "${PACKAGE_DIR}/"

# Remove unnecessary files
cd "${PACKAGE_DIR}"
rm -f scripts/package.sh

# Create zip
cd "${OUTPUT_DIR}"
zip -r "${PACKAGE_NAME}.zip" "${PACKAGE_NAME}" -q

# Cleanup
rm -rf "${PACKAGE_DIR}"

echo "✅ Package created: ${PACKAGE_NAME}.zip"
echo "📊 Size: $(du -h ${PACKAGE_NAME}.zip | cut -f1)"
