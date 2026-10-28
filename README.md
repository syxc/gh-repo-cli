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
tar -xzf "ghr_<version>_darwin_arm64.tar.gz"
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
ghr clean --all                         # drop recognized cached repositories
ghr clean --old 30d                     # drop repos unused for 30 days or more
ghr clean                               # show usage plus cache statistics
```

Details worth knowing:

- `search` takes a Rust regular expression. The table shows at most 50 rows; `-o` saves all matches, but no file is written when there are no matches. Binary files (a NUL byte in the first 8 KiB) are skipped.
- `analyze` counts files at the root and one directory level below it (depth 2). Both tables and JSON retain the top 10 languages and file types; `--depth` changes only the structure tree.
- `-o <file>` writes JSON for `analyze`, `structure` and `search`, creating parent directories as needed. `analyze --sections` selects `info`, `languages`, `file-types` and `structure`; JSON uses `fileTypes` for `file-types` and always includes `repo`.
- `--no-cache` re-clones instead of updating the cached copy. A failed normal refresh returns an error and preserves the cache.
- Command arguments must be valid UTF-8. Terminal control characters in repository content are escaped; normal line breaks and tabs remain.
- `read` prints the file content; for a missing file it lists the parent directory so you can pick the right path. `read`, `ls`, `search` and `readme` keep file access within the repository, including symbolic-link targets.

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

Repositories are cached in `~/.ghr-cache/<owner>/<repo>`. Set `GH_CACHE_DIR` to use another location; an existing directory must be empty or a recognized ghr cache. Commands using the same cache root run one at a time, with a lock wait of up to 30 seconds.

- Clones use depth 1. Each refresh fetches the current default branch at depth 1 without tags, then runs `git gc --auto` with Git's normal expiry policy. Cache size can grow with repository content and retained objects.
- Cleanup removes only verified GitHub repository directories, preserving the cache root and unrelated files. Symbolic links at the owner/repo path are rejected.
- `ghr clean --old 30d` removes repos unused for at least 30 days; bare `ghr clean` shows cache size and the oldest entry. Each successful use refreshes the last-used marker.
- Set `GH_CACHE_MAX_AGE` (e.g. `30d`) to remove stale repos before cloning or refreshing a repo, excluding the repo in use:

  ```bash
  export GH_CACHE_MAX_AGE=30d
  ```

`-o <file>` writes to the path you give. `ghr clean -o` deletes `~/.ghr-output/`; it does not track those output files.

## Documentation

- [AI Integration Guide](docs/AI_INTEGRATION.md): using ghr with Claude Code and other assistants
- [Release Workflow](docs/RELEASE_WORKFLOW.md): how version tags become releases
- [Testing Guide](docs/TESTING.md): running and writing tests
- [Contributing](CONTRIBUTING.md): development setup and PR process

## License

MIT — see [LICENSE](LICENSE).
