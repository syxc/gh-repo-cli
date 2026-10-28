# Release Workflow Guide

> How ghr releases are built and published with GitHub Actions

## Overview

Pushing a version tag (`v*`) triggers `.github/workflows/release.yml`, which:

1. Checks that the tag equals `v<package.version>` and the matching changelog section has release notes, then runs tests and clippy. Any failure stops the release.
2. Builds release binaries for five targets:

   | Target | Runner |
   |--------|--------|
   | `x86_64-unknown-linux-musl` | ubuntu-latest |
   | `aarch64-unknown-linux-musl` | ubuntu-latest |
   | `x86_64-apple-darwin` | macos-latest |
   | `aarch64-apple-darwin` | macos-latest |
   | `x86_64-pc-windows-msvc` | windows-latest |

3. Packages archives named `ghr_<version>_<os>_<arch>` (`.tar.gz` for Unix, `.zip` for Windows, binary inside renamed to `ghr` / `ghr.exe`)
4. Generates `checksums.txt` (SHA256)
5. Publishes notes from the matching `CHANGELOG.md` section, with a link to the tagged changelog. Notes stay in `RUNNER_TEMP` and pass between jobs as an artifact.

## Triggering a Release

1. On a branch, set `Cargo.toml` to the release version and move `Unreleased` entries to `## [<version>] - <date>` in `CHANGELOG.md`. Refresh `Cargo.lock`, review it, and run the [validation checks](../CONTRIBUTING.md#4-validate):

   ```bash
   cargo check
   git diff -- Cargo.toml Cargo.lock CHANGELOG.md
   git add Cargo.toml Cargo.lock CHANGELOG.md
   git commit -m "chore: prepare release"
   ```

2. Merge the PR, then tag that commit on `master` with the manifest version:

   ```bash
   VERSION=$(python3 -c 'import tomllib; print(tomllib.load(open("Cargo.toml", "rb"))["package"]["version"])')
   git tag -a "v$VERSION" -m "Release v$VERSION"
   git push origin "v$VERSION"
   ```

3. Watch the workflow under the Actions tab; the release appears with all five archives plus `checksums.txt`.

`ghr --version` uses the manifest version. The workflow rejects a different tag or a missing or empty release-notes section.

## Versioning

Releases follow [Semantic Versioning](https://semver.org/): MAJOR for breaking
changes, MINOR for features, PATCH for fixes. Tags must carry the `v` prefix
(e.g. `v0.3.3`); the workflow only triggers on `v*`.

## Permissions

The release job needs `contents: write` to create the release; this is already
declared in the workflow file. If your repository restricts workflow
permissions (Settings → Actions → General → Workflow permissions), select
"Read and write permissions".

## Verifying a Release

Before tagging, run the workflow's metadata check on isolated copies. This needs Python 3.11+ and PyYAML, and does not build or publish:

```bash
RELEASE_TAG=v0.3.3 python3 - <<'PY'
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import yaml

workflow = yaml.safe_load(Path(".github/workflows/release.yml").read_text())
script = next(step["run"] for step in workflow["jobs"]["test"]["steps"]
              if step.get("id") == "release-metadata")
with tempfile.TemporaryDirectory() as work:
    for name in ("Cargo.toml", "CHANGELOG.md"):
        shutil.copyfile(name, Path(work) / name)
    env = dict(os.environ, GITHUB_REF_NAME=os.environ["RELEASE_TAG"],
               GITHUB_REPOSITORY="local/check", RUNNER_TEMP=work)
    subprocess.run(["bash", "-euo", "pipefail", "-c", script],
                   cwd=work, env=env, check=True)
    print((Path(work) / "release-notes.md").read_text())
PY
```

After release:

```bash
# Download and verify checksums
sha256sum -c checksums.txt

# Extract and smoke-test
tar -xzf "ghr_${VERSION}_darwin_arm64.tar.gz"
./ghr --version
./ghr analyze octocat/Hello-World
```

## Correcting a Release

Keep published tags unchanged. Fix the problem on a branch, validate it through a PR, and publish the next patch version. Mark an affected release in its notes when users need to avoid it.

## Troubleshooting

- **Workflow does not trigger**: tag must match `v*` and must be pushed
  (`git ls-remote --tags origin`).
- **"Resource not accessible by integration"**: repository workflow permissions
  are too restrictive; enable read/write as above.
- **Build fails on one target**: re-run the failed matrix job; the five targets
  build independently, so a single target can be retried without re-running tests.
