#!/bin/bash
# NPM Publish Helper Script for ghr
# Usage: ./scripts/publish-npm.sh [version]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

cd "$PROJECT_ROOT"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_error() {
    echo -e "${RED}❌ Error: $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_info() {
    echo -e "${YELLOW}ℹ️  $1${NC}"
}

# Get version from arguments or package.json
if [ $# -eq 1 ]; then
    VERSION="$1"
else
    VERSION=$(jq -r '.version' package.json)
fi

print_info "Preparing to publish v${VERSION} to npm"
echo ""

# Check prerequisites
print_info "Checking prerequisites..."

if ! command -v npm &> /dev/null; then
    print_error "npm is not installed"
    exit 1
fi

if ! command -v jq &> /dev/null; then
    print_error "jq is not installed. Please install it: brew install jq"
    exit 1
fi

# Check if logged in to npm
if ! npm whoami &> /dev/null; then
    print_error "Not logged in to npm. Please run: npm login"
    exit 1
fi

print_success "Logged in as: $(npm whoami)"
echo ""

# Check if GitHub release exists
print_info "Checking GitHub release v${VERSION}..."

if ! curl -s "https://api.github.com/repos/syxc/gh-repo-cli/releases/tags/v${VERSION}" | grep -q "tag_name"; then
    print_error "GitHub release v${VERSION} not found!"
    echo ""
    echo "Please create a release first:"
    echo "  git tag -a v${VERSION} -m \"Release v${VERSION}\""
    echo "  git push origin v${VERSION}"
    echo ""
    echo "Or wait for the GitHub Actions release workflow to complete."
    exit 1
fi

print_success "GitHub release v${VERSION} found"
echo ""

# Verify binaries exist in release
print_info "Verifying release binaries..."

# Get asset list from GitHub API
ASSETS=$(curl -s "https://api.github.com/repos/syxc/gh-repo-cli/releases/tags/v${VERSION}" | jq -r '.assets | .[].name')

PLATFORMS=("linux_amd64" "linux_arm64" "darwin_amd64" "darwin_arm64" "windows_amd64")
MISSING=()

for platform in "${PLATFORMS[@]}"; do
    if [ "$platform" = "windows_amd64" ]; then
        expected="ghr_${VERSION}_${platform}.zip"
    else
        expected="ghr_${VERSION}_${platform}.tar.gz"
    fi
    
    if ! echo "$ASSETS" | grep -q "^${expected}$"; then
        MISSING+=("$platform")
    fi
done

if [ ${#MISSING[@]} -gt 0 ]; then
    print_error "Missing binaries for platforms: ${MISSING[*]}"
    echo ""
    echo "Available binaries:"
    echo "$ASSETS" | grep -v "checksums.txt" | sed 's/^/  - /'
    exit 1
fi

print_success "All binaries verified"
echo ""

# Check package.json version matches
PACKAGE_VERSION=$(jq -r '.version' package.json)
if [ "$PACKAGE_VERSION" != "$VERSION" ]; then
    print_info "Updating package.json version to ${VERSION}..."
    jq --arg ver "$VERSION" '.version = $ver' package.json > package.json.tmp
    mv package.json.tmp package.json
    print_success "Updated package.json version"
fi

# Show what will be published
echo ""
print_info "Package info:"
echo "  Name: $(jq -r '.name' package.json)"
echo "  Version: $(jq -r '.version' package.json)"
echo "  Binaries URL: $(jq -r '.goBinary.url' package.json)"
echo ""

# Preview files to be published
print_info "Files to be published:"
npm pack --dry-run 2>&1 | grep -E "^npm notice" | head -20
echo ""

# Confirm with user
echo -n "Are you sure you want to publish to npm? (yes/no): "
read -r CONFIRM

if [ "$CONFIRM" != "yes" ]; then
    print_error "Publish aborted by user"
    exit 1
fi

echo ""
print_info "Publishing to npm..."

# Publish (without 2FA prompt since it's done manually)
npm publish --access public

print_success "Published $(jq -r '.name' package.json)@$(jq -r '.version' package.json) to npm!"
echo ""
echo "Users can now install with:"
echo "  npm install -g $(jq -r '.name' package.json)"
