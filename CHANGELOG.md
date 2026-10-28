# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.3] - 2026-10-28

### Added
- `analyze --depth` controls the structure tree; `--sections` selects report sections. JSON always keeps `repo` and names the file-types section `fileTypes`.
- System SOCKS proxy detection on macOS, GNOME and Windows, using `socks5h://`; HTTP(S) proxies take priority.
- `GH_CACHE_DIR` selects an empty directory or a recognized existing cache.

### Fixed
- Cache cleanup preserves unrelated data and rejects symlink paths and redirected Git worktrees. A process lock protects repository access from concurrent updates and cleanup.
- Refreshes keep depth 1 and follow the current default branch. Failed refreshes preserve the cache. Git automatic maintenance replaces forced full pruning on every call.
- File reading, README lookup, listing and search reject links outside the repository. Terminal control characters in names and content are escaped.
- Non-ASCII depth values return errors instead of panicking; help/version flags honor explicit boolean values. Invalid UTF-8 arguments are rejected without changing their bytes.
- System proxy parsing validates ports, formats IPv6 addresses and ignores nested macOS proxy scopes.
- `clean --old -o` also clears the output directory; automatic cleanup reports deletion failures.
- JSON uses Go-compatible backspace/form-feed escapes; common IO errors use platform-independent wording.

### Changed
- `search` skips files with NUL bytes in the first 8 KiB. `analyze` statistics stay at depth 2 and retain the top 10 entries, independently of the displayed tree depth.
- CI adds native tests on Linux, macOS and Windows plus a Rust 1.85 check. Releases reject mismatched tags and missing notes; temporary notes stay outside the checkout.
- Documentation states cache growth, refresh behavior, JSON fields and search-output limits.

## [0.3.2] - 2026-10-28

### Changed
- Release notes are now generated from the matching `CHANGELOG.md` section instead of the raw PR list

### Added
- `AGENTS.md` for AI coding agents, plus a `CLAUDE.md` pointer to it

## [0.3.1] - 2026-10-28

### Added
- LRU-style cache retention: `ghr clean --old 30d` removes cached repositories unused for 30 days or more; bare `ghr clean` shows cache statistics (repos, size, oldest entry) as a reminder
- Automatic eviction via `GH_CACHE_MAX_AGE` (e.g. `30d`): stale repositories are evicted on every invocation before the cache is touched
- Clones use `--depth 1 --single-branch`; refreshes expire reflogs and prune superseded objects. This did not prevent reachable history from growing; 0.3.3 corrects the refresh depth and maintenance policy.

## [0.3.0] - 2026-10-28

### Changed
- Rewrite in Rust retaining the seven repository commands, report formats and exit-code conventions. The completion command was removed as noted below; Rust regex syntax and Unicode classes differ from Go regexp.
- Dependencies were `regex` and `thiserror`; JSON retains HTML escaping, field order and `null` vs `[]` semantics.
- Count tables (`analyze`) break ties deterministically by key; the Go build's tie order was random

### Added
- Automatic system proxy detection (macOS `scutil --proxy`, Linux GNOME `gsettings`, Windows registry), refreshed on every invocation; environment variables still take priority (`GH_PROXY`, `HTTPS_PROXY`/`https_proxy`, `HTTP_PROXY`/`http_proxy`, `ALL_PROXY`/`all_proxy`)
- `--no-cache` now actually re-clones as documented (it was accepted but ignored in the Go build)

### Removed
- Cobra's auto-generated `completion` command (CLI framework boilerplate; shell completion was never part of ghr's documented feature set)

## [0.2.3] - 2026-04-09

### Fixed
- Version string and packaging metadata

## [0.2.0] - 2026-04-09

### Changed
- **Complete rewrite in Go** - migrated from Node.js to Go for better performance and distribution
- New installation methods: pre-built binaries or build from source
- Updated GitHub Actions workflows for the Go build process

### Added
- Cross-platform binary releases for Linux, macOS, and Windows (amd64/arm64)
- Improved error handling and resource management
- Better CJK (Chinese, Japanese, Korean) character support in search and display

### Removed
- Node.js dependencies and npm-based installation
- Jest testing framework and ESLint configuration

## [0.1.2] - 2026-04-09

### Changed
- Changed package name to scoped package `@oknian1/gh-repo-cli`

## [0.1.1] - 2026-04-09

### Changed
- Simplified installation to `npm install -g @oknian1/gh-repo-cli`

## [0.1.0] - 2026-03-31

### Added
- Initial development release
- Repository analysis with language detection
- Code search functionality
- Directory structure visualization
- File reading capability
- HTTP/HTTPS/SOCKS5 proxy support
- Local caching system
- JSON output support
- Comprehensive documentation with Chinese translations
- AI integration guides for Claude Code
