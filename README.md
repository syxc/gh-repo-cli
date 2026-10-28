# ghr (gh-repo-cli)

> A lightweight CLI tool for analyzing GitHub repositories without API tokens

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange)](https://www.rust-lang.org)
[![CI](https://github.com/syxc/gh-repo-cli/workflows/CI/badge.svg)](https://github.com/syxc/gh-repo-cli/actions)
[![Code Quality](https://github.com/syxc/gh-repo-cli/workflows/Code%20Quality/badge.svg)](https://github.com/syxc/gh-repo-cli/actions)

ghr clones a repository with `git` and analyzes it locally: languages, file types, directory structure, code search, file reading. It is a good fit for terminal work and for AI coding assistants that need repository context without burning API or MCP quotas. Public repositories only.

## Installation

Download an archive for your platform from the [Releases](https://github.com/syxc/gh-repo-cli/releases) page and extract the `ghr` binary:

```bash
# macOS ARM64; substitute <version> and your platform (see the releases page)
curl -LO "https://github.com/syxc/gh-repo-cli/releases/latest/download/ghr_<version>_darwin_arm64.tar.gz"
tar -xzf ghr_<version>_darwin_arm64.tar.gz
sudo mv ghr /usr/local/bin/
```

Platforms: `linux_amd64`, `linux_arm64`, `darwin_amd64`, `darwin_arm64`, `windows_amd64` (`.zip`), with SHA256 sums in `checksums.txt`.

Or build from source (Rust 1.85+ and Git):

```bash
git clone https://github.com/syxc/gh-repo-cli.git
cd gh-repo-cli
cargo install --path .
```

## Usage

```bash
ghr analyze facebook/react              # languages, file types, structure
ghr analyze facebook/react --depth 3    # deeper structure tree (default 2)
ghr analyze facebook/react --sections languages   # only the language table
ghr structure vuejs/core --depth 2      # directory tree (depth defaults to 3)
ghr search facebook/react useState -e .js
ghr search facebook/react types --ignore-case
ghr read facebook/react README.md
ghr ls facebook/react src               # depth defaults to 1
ghr readme facebook/react

ghr clean facebook/react                # drop one cached repo
ghr clean --all                         # drop the whole cache
ghr clean --old 30d                     # drop repos unused for 30 days or more
ghr clean                               # show usage plus cache statistics
```

Details worth knowing:

- `search` takes a regular expression. The table shows at most 50 rows; `-o` always saves the full result set. Binary files (a NUL byte in the first 8 KiB) are skipped.
- `-o <file>` writes JSON (created directories included) for `analyze`, `structure` and `search`. With `--sections`, `analyze` writes only the selected sections.
- `--no-cache` re-clones instead of updating the cached copy.
- `read` prints the file content; for a missing file it lists the parent directory so you can pick the right path.

## Proxy

The proxy is resolved on every invocation, so a proxy that changes on your system is picked up by the next command. First non-empty wins:

1. `GH_PROXY`
2. `HTTPS_PROXY` / `https_proxy`
3. `HTTP_PROXY` / `http_proxy`
4. `ALL_PROXY` / `all_proxy`
5. OS settings: `scutil --proxy` on macOS, GNOME `gsettings` on Linux, the registry on Windows. HTTP(S) proxies are preferred; a SOCKS-only configuration is used as `socks5h://` (remote DNS)

```bash
GH_PROXY="http://127.0.0.1:7890" ghr analyze facebook/react   # temporary override
```

## Cache and output

Repositories are cached in `~/.ghr-cache/<owner>/<repo>`; `ghr clean` removes entries from it. `~/.ghr-output/` exists only for `ghr clean -o`, which deletes it entirely; `-o` itself always writes to the path you give.

Long-lived caches stay small by design. The cache location itself can be moved with `GH_CACHE_DIR` (default `~/.ghr-cache`):

- Clones are shallow and limited to the default branch tip (`--depth 1 --single-branch`); each refresh expires reflogs and prunes superseded objects, so a cached repo does not grow over time.
- LRU-style retention: every use of a repo refreshes its last-used marker. `ghr clean --old 30d` evicts repos unused for 30 days or more; bare `ghr clean` shows the cache size and the oldest entry as a reminder.
- Automatic eviction: set `GH_CACHE_MAX_AGE` (same format, e.g. `30d`) and ghr evicts stale repos on every invocation before touching the cache:

  ```bash
  export GH_CACHE_MAX_AGE=30d
  ```

## Documentation

- [AI Integration Guide](docs/AI_INTEGRATION.md) — using ghr with Claude Code and other assistants
- [Release Workflow](docs/RELEASE_WORKFLOW.md) — how version tags become releases
- [Testing Guide](docs/TESTING.md) — running and writing tests
- [Contributing](CONTRIBUTING.md) — development setup and PR process

## License

MIT — see [LICENSE](LICENSE).
