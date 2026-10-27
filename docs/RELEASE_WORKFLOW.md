# Release Workflow Guide

> How ghr releases are built and published with GitHub Actions

## Overview

Pushing a version tag (`v*`) triggers `.github/workflows/release.yml`, which:

1. Runs tests and clippy (release is aborted if either fails)
2. Builds release binaries for five targets on native runners:

   | Target | Runner |
   |--------|--------|
   | `x86_64-unknown-linux-musl` | ubuntu-latest |
   | `aarch64-unknown-linux-musl` | ubuntu-latest |
   | `x86_64-apple-darwin` | macos-latest |
   | `aarch64-apple-darwin` | macos-latest |
   | `x86_64-pc-windows-msvc` | windows-latest |

3. Packages archives named `ghr_<version>_<os>_<arch>` (`.tar.gz` for Unix, `.zip` for Windows, binary inside renamed to `ghr` / `ghr.exe`)
4. Generates `checksums.txt` (SHA256)
5. Creates a GitHub Release whose notes are extracted from the matching `CHANGELOG.md` section (falls back to a changelog link if the section is missing)

## Triggering a Release

1. Bump `version` in `Cargo.toml` and commit:

   ```bash
   # edit Cargo.toml: version = "0.2.4"
   git add Cargo.toml Cargo.lock
   git commit -m "chore: bump version to 0.2.4"
   ```

2. Tag and push:

   ```bash
   git tag -a v0.2.4 -m "Release v0.2.4"
   git push origin v0.2.4
   ```

3. Watch the workflow under the Actions tab; the release appears with all five archives plus `checksums.txt`.

The reported `ghr --version` string comes from `Cargo.toml`'s `version` — no
build-time injection needed. Keep the tag in sync with the manifest version.

## Versioning

Releases follow [Semantic Versioning](https://semver.org/): MAJOR for breaking
changes, MINOR for features, PATCH for fixes. Tags must carry the `v` prefix
(`v0.2.4`, not `0.2.4`) — the workflow only triggers on `v*`.

## Permissions

The release job needs `contents: write` to create the release; this is already
declared in the workflow file. If your repository restricts workflow
permissions (Settings → Actions → General → Workflow permissions), select
"Read and write permissions".

## Verifying a Release

```bash
# Download and verify checksums
sha256sum -c checksums.txt

# Extract and smoke-test
tar -xzf ghr_0.2.4_darwin_arm64.tar.gz
./ghr --version
./ghr analyze octocat/Hello-World
```

## Rollback

Delete the tag and the release if something goes wrong:

```bash
git tag -d v0.2.4
git push origin :refs/tags/v0.2.4
# then remove the release on GitHub
```

## Troubleshooting

- **Workflow does not trigger**: tag must match `v*` and must be pushed
  (`git ls-remote --tags origin`).
- **"Resource not accessible by integration"**: repository workflow permissions
  are too restrictive; enable read/write as above.
- **Build fails on one target**: re-run the failed matrix job; the five targets
  build independently, so a single target can be retried without re-running tests.
