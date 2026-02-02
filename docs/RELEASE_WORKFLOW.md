# Release Workflow Configuration Guide

> Complete guide for configuring and triggering automated releases for gh-repo-cli

## 📚 Table of Contents

- [Overview](#overview)
- [GITHUB_TOKEN Configuration](#github_token-configuration)
- [How to Trigger a Release](#how-to-trigger-a-release)
- [Release Process Details](#release-process-details)
- [Troubleshooting](#troubleshooting)
- [Best Practices](#best-practices)

---

## Overview

The gh-repo-cli project uses GitHub Actions for automated releases. When you push a version tag, the workflow automatically:

1. ✅ Runs all tests
2. ✅ Runs linter checks
3. ✅ Creates a distributable package
4. ✅ Generates SHA256 checksum
5. ✅ Creates a GitHub Release with assets
6. ✅ Auto-generates release notes

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

Update the version in `package.json`:

```bash
# Edit package.json
{
  "name": "gh-repo-cli",
  "version": "1.1.0",  # Update this
  ...
}
```

#### Step 2: Commit Changes

```bash
git add package.json
git commit -m "chore: bump version to 1.1.0"
```

#### Step 3: Create and Push Tag

```bash
# Create annotated tag
git tag -a v1.1.0 -m "Release v1.1.0"

# Push tag to trigger workflow
git push origin v1.1.0
```

**That's it!** The workflow will automatically start.

---

### Method 2: Using GitHub Web UI

#### Step 1: Update and Commit Version

1. Go to your repository on GitHub
2. Edit `package.json` to update version
3. Commit the changes

#### Step 2: Create Release on GitHub

1. Go to **Releases** page
2. Click **Create a new release**
3. Click **Choose a tag** → enter `v1.1.0`
4. Click **Create new tag**: `v1.1.0`
5. Set **Target**: `master` (or your main branch)
6. Add release notes (optional - will be auto-generated if empty)
7. Click **Publish release**

**The workflow will automatically trigger when you publish the release!**

---

### Method 3: Using npm version (Easiest)

```bash
# Automatically update version and create tag
npm version patch  # 1.0.0 → 1.0.1
npm version minor  # 1.0.0 → 1.1.0
npm version major  # 1.0.0 → 2.0.0

# Push the tag
git push origin master --tags
```

**This is the recommended method!** It automatically:
- Updates `package.json`
- Creates a git commit
- Creates an annotated tag
- Follows semantic versioning

---

## Release Process Details

### What Happens When You Push a Tag?

#### Phase 1: Setup (1-2 minutes)

```yaml
- Checkout code
- Setup Node.js 18.x
- Install dependencies
```

#### Phase 2: Quality Checks (2-3 minutes)

```yaml
- Run tests (npm test)
- Run linter (npm run lint)
```

**If tests fail**: The workflow stops, no release is created

#### Phase 3: Package Creation (1 minute)

```yaml
- Create package using scripts/package.sh
- Generates: gh-repo-cli-v1.1.0.zip
```

#### Phase 4: Checksum Generation (< 1 minute)

```yaml
- Generate SHA256 checksum
- Creates: gh-repo-cli-v1.1.0.zip.sha256
```

#### Phase 5: Release Creation (1 minute)

```yaml
- Create GitHub Release
- Upload: gh-repo-cli-v1.1.0.zip
- Upload: gh-repo-cli-v1.1.0.zip.sha256
- Auto-generate release notes
```

**Total Time**: ~5-8 minutes

---

### Artifacts Created

Each release creates two files:

#### 1. Package Zip: `gh-repo-cli-v1.1.0.zip`

Contains:
```
gh-repo-cli-v1.1.0/
├── commands/
│   ├── analyze.js
│   ├── clean.js
│   ├── ls.js
│   ├── read.js
│   ├── search.js
│   └── structure.js
├── lib/
│   ├── cache.js
│   ├── github.js
│   └── utils.js
├── index.js
├── package.json
└── README.md
```

**Excludes**:
- `node_modules/`
- `.git/`
- Test files
- Development scripts

#### 2. Checksum: `gh-repo-cli-v1.1.0.zip.sha256`

Contains:
```
a1b2c3d4e5f6...  gh-repo-cli-v1.1.0.zip
```

**Purpose**: Verify file integrity after download

---

## Troubleshooting

### Issue 1: Workflow doesn't trigger

**Symptoms**: Push tag but nothing happens on GitHub

**Possible Causes**:

1. **Tag format is wrong**
   ```bash
   # ❌ Wrong (missing 'v' prefix)
   git tag 1.1.0

   # ✅ Correct
   git tag v1.1.0
   ```

2. **Tag not pushed**
   ```bash
   # Make sure to push the tag!
   git push origin v1.1.0
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
git push origin v1.1.0 --force
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

1. **Node version mismatch**
   ```bash
   # Check CI version (from .github/workflows/release.yml)
   node-version: '18.x'

   # Check your local version
   node --version

   # If different, install correct version
   nvm install 18
   nvm use 18
   ```

2. **Environment differences**
   ```bash
   # Run tests in clean environment
   npm ci
   npm test
   ```

3. **Missing dependencies**
   ```bash
   # Ensure all dependencies are committed
   git add package.json package-lock.json
   git commit -m "chore: update dependencies"
   ```

**Solution**:

```bash
# Mimic CI environment locally
docker run --rm -v "$PWD":/app -w /app node:18 npm test
```

---

### Issue 4: Package creation fails

**Symptoms**: Workflow fails at "Create package" step

**Error Message**:
```
bash: scripts/package.sh: No such file or directory
```

**Solution**:

1. Check if `scripts/package.sh` exists:
   ```bash
   ls -la scripts/package.sh
   ```

2. Ensure it's executable:
   ```bash
   chmod +x scripts/package.sh
   ```

3. Verify script syntax:
   ```bash
   bash -n scripts/package.sh
   ```

4. Commit the fix:
   ```bash
   git add scripts/package.sh
   git commit -m "fix: add executable permission to package.sh"
   git push
   ```

---

### Issue 5: Release created but assets missing

**Symptoms**: GitHub Release is created but no files attached

**Possible Causes**:

1. **Package script failed silently**
2. **File glob pattern incorrect**
3. **Upload step failed**

**Solution**:

Check workflow logs:

1. Go to **Actions** tab
2. Click on the failed workflow run
3. Expand each step to find errors
4. Look for "Create package" and "Create Release" steps

**Common Fixes**:

```yaml
# Ensure file paths are correct (not using wildcards in paths)
- name: Create Release
  uses: softprops/action-gh-release@v1
  with:
    files: |
      gh-repo-cli-${{ env.VERSION }}.zip
      gh-repo-cli-${{ env.VERSION }}.zip.sha256
```

---

## Best Practices

### 1. Use Semantic Versioning

Follow semver for version numbers:

```bash
# MAJOR: Breaking changes
npm version major  # 1.0.0 → 2.0.0

# MINOR: New features (backward compatible)
npm version minor  # 1.0.0 → 1.1.0

# PATCH: Bug fixes (backward compatible)
npm version patch  # 1.0.0 → 1.0.1
```

### 2. Create Changelog

Keep `CHANGELOG.md` updated with each release:

```markdown
## [1.1.0] - 2024-02-02

### Added
- New command: `ghr search --ignore-case`
- Support for SOCKS5 proxies

### Fixed
- Cache directory creation on Windows
- File encoding issues in search results

### Changed
- Improved error messages
- Updated dependencies
```

### 3. Test Before Tagging

Always run full test suite before creating release tag:

```bash
# Run all checks
npm run lint
npm test

# If everything passes, then create release
npm version patch
git push origin master --tags
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
# 1. Download the release package
wget https://github.com/yourname/gh-repo-cli/releases/download/v1.1.0/gh-repo-cli-v1.1.0.zip

# 2. Verify checksum
sha256sum -c gh-repo-cli-v1.1.0.zip.sha256

# 3. Extract and test
unzip gh-repo-cli-v1.1.0.zip
cd gh-repo-cli-v1.1.0
npm install
npm link
ghr --version

# 4. Try basic commands
ghr analyze facebook/react
```

---

## Quick Reference

### Release Commands

```bash
# Quick release (patch)
npm version patch && git push origin master --tags

# Minor release
npm version minor && git push origin master --tags

# Major release
npm version major && git push origin master --tags

# Manual tag
git tag -a v1.1.0 -m "Release v1.1.0"
git push origin v1.1.0
```

### Monitoring Release

```bash
# Check recent tags
git tag -l "v*" --sort=-version:refname | head -5

# Check tag details
git show v1.1.0

# Compare releases
git diff v1.0.0 v1.1.0 --stat
```

### Rollback (If Needed)

```bash
# If something goes wrong, delete the tag
git tag -d v1.1.0
git push origin :refs/tags/v1.1.0

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
    # 1. Checkout code
    - name: Checkout code
      uses: actions/checkout@v3

    # 2. Setup Node.js
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18.x'
        cache: 'npm'

    # 3. Install dependencies
    - name: Install dependencies
      run: npm ci

    # 4. Run tests
    - name: Run tests
      run: npm test

    # 5. Run linter
    - name: Run linter
      run: npm run lint

    # 6. Create package
    - name: Create package
      run: |
        VERSION=${GITHUB_REF#refs/tags/v}
        bash scripts/package.sh ${VERSION}

    # 7. Generate checksum
    - name: Generate checksum
      run: |
        VERSION=${GITHUB_REF#refs/tags/v}
        sha256sum "gh-repo-cli-${VERSION}.zip" > "gh-repo-cli-${VERSION}.zip.sha256"

    # 8. Create Release
    - name: Create Release
      uses: softprops/action-gh-release@v1
      with:
        files: |
          gh-repo-cli-*.zip
          gh-repo-cli-*.zip.sha256
        generate_release_notes: true
        draft: false
        prerelease: false
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

**Key Points**:
- ✅ Triggers only on version tags (v*)
- ✅ Requires `contents: write` permission
- ✅ Uses `npm ci` for clean installs
- ✅ Runs tests before packaging
- ✅ Auto-generates release notes
- ✅ Uses built-in `GITHUB_TOKEN` (no secrets needed)

---

## Additional Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Creating Releases](https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository)
- [Semantic Versioning](https://semver.org/)
- [softprops/action-gh-release](https://github.com/softprops/action-gh-release)

---

<div align="center">

**Need Help?** Open an issue: [github.com/syxc/gh-repo-cli/issues](https://github.com/syxc/gh-repo-cli/issues)

**Happy Releasing! 🚀**

</div>
