# Release Workflow Configuration Guide

> Complete guide for configuring and triggering automated releases for ghr

## 📚 Table of Contents

- [Overview](#overview)
- [GITHUB_TOKEN Configuration](#github_token-configuration)
- [How to Trigger a Release](#how-to-trigger-a-release)
- [Release Process Details](#release-process-details)
- [Troubleshooting](#troubleshooting)
- [Best Practices](#best-practices)

---

## Overview

The ghr project uses GitHub Actions for automated releases. When you push a version tag, the workflow automatically:

1. ✅ Runs all tests
2. ✅ Runs linter checks
3. ✅ Builds binaries for multiple platforms (Linux, macOS, Windows)
4. ✅ Creates archives (.tar.gz, .zip)
5. ✅ Generates SHA256 checksums
6. ✅ Creates a GitHub Release with assets
7. ✅ Auto-generates release notes

**Workflow File**: `.github/workflows/release.yml`

---

## GITHUB_TOKEN Configuration

### What is GITHUB_TOKEN?

`GITHUB_TOKEN` is a built-in authentication token automatically provided by GitHub Actions to workflows. It requires **NO manual configuration**.

### Permissions Needed

The workflow automatically requests the necessary permissions:

```yaml
permissions:
  contents: write  # Needed to create releases
```

**This is already configured in the workflow file** - you don't need to do anything!

### How It Works

1. **Automatic Provisioning**: When the workflow runs, GitHub automatically creates a `GITHUB_TOKEN` with the specified permissions
2. **No Secrets Required**: Unlike personal access tokens, you don't need to add this to repository secrets
3. **Scope Limited**: The token only has access to the repository where the workflow runs
4. **Auto-Expiration**: The token expires automatically after the workflow completes

### Verifying Permissions

You can verify the workflow has the correct permissions:

1. Go to your repository on GitHub
2. Click **Settings** → **Actions** → **General**
3. Scroll to **Workflow permissions**
4. Ensure **Read and write permissions** is selected

```
⚠️ Important: If you see "Read repository contents permission", you need to change it
   to "Read and write permissions" for releases to work!
```

---

## How to Trigger a Release

### Method 1: Using Git Commands (Recommended)

#### Step 1: Update Version

Update the version in `cmd/root.go`:

```bash
# Edit cmd/root.go
var Version = "1.0.0"  // Update this
```

#### Step 2: Commit Changes

```bash
git add cmd/root.go
git commit -m "chore: bump version to 1.0.0"
```

#### Step 3: Create and Push Tag

```bash
# Create annotated tag
git tag -a v1.0.0 -m "Release v1.0.0"

# Push tag to trigger workflow
git push origin v1.0.0
```

**That's it!** The workflow will automatically start.

---

### Method 2: Using GitHub Web UI

#### Step 1: Update and Commit Version

1. Go to your repository on GitHub
2. Edit `cmd/root.go` to update version
3. Commit the changes

#### Step 2: Create Release on GitHub

1. Go to **Releases** page
2. Click **Create a new release**
3. Click **Choose a tag** → enter `v1.0.0`
4. Click **Create new tag**: `v1.0.0`
5. Set **Target**: `master` (or your main branch)
6. Add release notes (optional - will be auto-generated if empty)
7. Click **Publish release**

**The workflow will automatically trigger when you publish the release!**

---

## Release Process Details

### What Happens When You Push a Tag?

#### Phase 1: Setup (1-2 minutes)

```yaml
- Checkout code
- Setup Go 1.23
- Install dependencies
```

#### Phase 2: Quality Checks (2-3 minutes)

```yaml
- Run tests (go test)
- Run linter (go vet, golint)
```

**If tests fail**: The workflow stops, no release is created

#### Phase 3: Build Binaries (3-5 minutes)

```yaml
- Build for Linux (amd64, arm64)
- Build for macOS (amd64, arm64)
- Build for Windows (amd64)
```

#### Phase 4: Package Creation (1 minute)

```yaml
- Create .tar.gz archives for Unix systems
- Create .zip archives for Windows
- Generates: ghr-v1.0.0-linux-amd64.tar.gz, etc.
```

#### Phase 5: Checksum Generation (< 1 minute)

```yaml
- Generate SHA256 checksums for all archives
- Creates: checksums.txt
```

#### Phase 6: Release Creation (1 minute)

```yaml
- Create GitHub Release
- Upload all archives
- Upload checksums.txt
- Auto-generate release notes
```

**Total Time**: ~8-12 minutes

---

### Artifacts Created

Each release creates the following files:

#### Binaries

- `ghr-v1.0.0-linux-amd64.tar.gz`
- `ghr-v1.0.0-linux-arm64.tar.gz`
- `ghr-v1.0.0-darwin-amd64.tar.gz`
- `ghr-v1.0.0-darwin-arm64.tar.gz`
- `ghr-v1.0.0-windows-amd64.zip`

#### Checksums

- `checksums.txt` - SHA256 checksums for all archives

---

## Troubleshooting

### Issue 1: Workflow doesn't trigger

**Symptoms**: Push tag but nothing happens on GitHub

**Possible Causes**:

1. **Tag format is wrong**
   ```bash
   # ❌ Wrong (missing 'v' prefix)
   git tag 1.0.0

   # ✅ Correct
   git tag v1.0.0
   ```

2. **Tag not pushed**
   ```bash
   # Make sure to push the tag!
   git push origin v1.0.0
   ```

3. **Workflow file has syntax error**
   - Check `.github/workflows/release.yml`
   - Validate YAML syntax

**Solution**:

```bash
# Check if tag exists locally
git tag

# Check if tag was pushed
git ls-remote --tags origin

# Re-push tag if needed
git push origin v1.0.0 --force
```

---

### Issue 2: Workflow fails with "Permission Denied"

**Symptoms**: Workflow fails at "Create Release" step

**Error Message**:
```
Error: Resource not accessible by integration
```

**Solution**:

1. Go to repository **Settings** → **Actions** → **General**
2. Find **Workflow permissions**
3. Change from:
   ```
   ◉ Read repository contents permission
   ```
   To:
   ```
   ◉ Read and write permissions
   ```
4. Click **Save**
5. Re-run the workflow

---

### Issue 3: Tests fail in CI but pass locally

**Symptoms**: Release workflow fails during test phase

**Possible Causes**:

1. **Go version mismatch**
   ```bash
   # Check CI version (from .github/workflows/release.yml)
   go-version: '1.23'

   # Check your local version
   go version

   # If different, install correct version
   # (Use go's built-in version management)
   ```

2. **Environment differences**
   ```bash
   # Clean build
   go clean -cache
   go mod tidy
   go test -v ./...
   ```

3. **Missing dependencies**
   ```bash
   # Ensure all dependencies are committed
   git add go.mod go.sum
   git commit -m "chore: update dependencies"
   ```

**Solution**:

```bash
# Mimic CI environment locally
docker run --rm -v "$PWD":/app -w /app golang:1.23 go test -v ./...
```

---

### Issue 4: Build fails

**Symptoms**: Workflow fails at "Build binaries" step

**Error Message**:
```
go build: cannot find main module
```

**Solution**:

1. Check if `go.mod` exists:
   ```bash
   ls -la go.mod
   ```

2. Ensure it's valid:
   ```bash
   go mod verify
   ```

3. Commit the fix:
   ```bash
   git add go.mod go.sum
   git commit -m "fix: update go.mod"
   git push
   ```

---

### Issue 5: Release created but assets missing

**Symptoms**: GitHub Release is created but no files attached

**Possible Causes**:

1. **Build step failed silently**
2. **File glob pattern incorrect**
3. **Upload step failed**

**Solution**:

Check workflow logs:

1. Go to **Actions** tab
2. Click on the failed workflow run
3. Expand each step to find errors
4. Look for "Build binaries" and "Create Release" steps

**Common Fixes**:

```yaml
# Ensure file paths are correct
- name: Create Release
  uses: softprops/action-gh-release@v1
  with:
    files: |
      dist/*.tar.gz
      dist/*.zip
      dist/checksums.txt
```

---

## Best Practices

### 1. Use Semantic Versioning

Follow semver for version numbers:

```bash
# MAJOR: Breaking changes
git tag v2.0.0

# MINOR: New features (backward compatible)
git tag v1.1.0

# PATCH: Bug fixes (backward compatible)
git tag v1.0.1
```

### 2. Create Changelog

Keep `CHANGELOG.md` updated with each release:

```markdown
## [1.0.0] - 2026-04-09

### Added
- Complete Go rewrite for better performance
- Cross-platform binary releases
- Improved error handling

### Changed
- Installation: npm → go install / binary download

### Removed
- Node.js dependencies
```

### 3. Test Before Tagging

Always run full test suite before creating release tag:

```bash
# Run all checks
go vet ./...
go test -v ./...

# Check formatting
gofmt -s -w .

# If everything passes, then create release
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

### 4. Use Release Branches (Optional)

For larger projects, use release branches:

```bash
# Create release branch
git checkout -b release/1.1.0

# Make final adjustments
# ... (fix bugs, update docs, etc.)

# Merge to master
git checkout master
git merge release/1.1.0

# Create and push tag
git tag -a v1.1.0 -m "Release v1.1.0"
git push origin master --tags
```

### 5. Verify After Release

After each release:

```bash
# 1. Download the release binary
wget https://github.com/syxc/gh-repo-cli/releases/download/v1.0.0/ghr-v1.0.0-$(uname -s)-$(uname -m).tar.gz

# 2. Verify checksum
sha256sum -c checksums.txt

# 3. Extract and test
tar -xzf ghr-v1.0.0-*.tar.gz
./ghr --version

# 4. Try basic commands
./ghr analyze facebook/react
```

---

## Quick Reference

### Release Commands

```bash
# Manual tag (patch)
git tag -a v1.0.1 -m "Release v1.0.1"
git push origin v1.0.1

# Minor release
git tag -a v1.1.0 -m "Release v1.1.0"
git push origin v1.1.0

# Major release
git tag -a v2.0.0 -m "Release v2.0.0"
git push origin v2.0.0
```

### Monitoring Release

```bash
# Check recent tags
git tag -l "v*" --sort=-version:refname | head -5

# Check tag details
git show v1.0.0

# Compare releases
git diff v1.0.0 v1.1.0 --stat
```

### Rollback (If Needed)

```bash
# If something goes wrong, delete the tag
git tag -d v1.0.0
git push origin :refs/tags/v1.0.0

# Also delete the release on GitHub web UI
```

---

## Workflow File Reference

The complete workflow file at `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'  # Triggers on tags like v1.0.0, v2.1.3, etc.

permissions:
  contents: write  # Required for creating releases

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Setup Go
      uses: actions/setup-go@v5
      with:
        go-version: '1.23'
        cache: true

    - name: Install dependencies
      run: go mod download

    - name: Run tests
      run: go test -v ./...

    - name: Run linter
      run: |
        go vet ./...
        go install golang.org/x/lint/golint@latest
        golint ./...

    - name: Build binaries
      run: |
        VERSION=${GITHUB_REF#refs/tags/v}
        mkdir -p dist
        
        # Build for multiple platforms
        platforms=(
          "linux/amd64"
          "linux/arm64"
          "darwin/amd64"
          "darwin/arm64"
          "windows/amd64"
        )
        
        for platform in "${platforms[@]}"; do
          IFS='/' read -r GOOS GOARCH <<< "$platform"
          output="ghr-${VERSION}-${GOOS}-${GOARCH}"
          if [ "$GOOS" = "windows" ]; then
            output="${output}.exe"
          fi
          
          GOOS=$GOOS GOARCH=$GOARCH go build \
            -ldflags="-s -w -X github.com/syxc/gh-repo-cli/cmd.Version=${VERSION}" \
            -o "dist/${output}" .
        done

    - name: Create archives
      run: |
        cd dist
        for file in ghr-*; do
          if [[ "$file" == *.exe ]]; then
            zip "${file%.exe}.zip" "$file"
          else
            tar -czf "${file}.tar.gz" "$file"
          fi
        done

    - name: Generate checksums
      run: |
        cd dist
        sha256sum *.tar.gz *.zip > checksums.txt

    - name: Create Release
      uses: softprops/action-gh-release@v1
      with:
        files: |
          dist/*.tar.gz
          dist/*.zip
          dist/checksums.txt
        generate_release_notes: true
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

**Key Points**:
- ✅ Triggers only on version tags (v*)
- ✅ Requires `contents: write` permission
- ✅ Cross-compilation for multiple platforms
- ✅ Uses `go mod download` for dependencies
- ✅ Runs tests before building
- ✅ Auto-generates release notes
- ✅ Uses built-in `GITHUB_TOKEN` (no secrets needed)

---

## Additional Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Creating Releases](https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository)
- [Semantic Versioning](https://semver.org/)
- [Go Cross Compilation](https://go.dev/doc/install/source#environment)
- [softprops/action-gh-release](https://github.com/softprops/action-gh-release)

---

<div align="center">

**Need Help?** Open an issue: [github.com/syxc/gh-repo-cli/issues](https://github.com/syxc/gh-repo-cli/issues)

**Happy Releasing! 🚀**

</div>
