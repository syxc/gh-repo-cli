# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-10-28

### Added
- LRU-style cache retention: `ghr clean --old 30d` removes cached repositories unused for 30 days or more; bare `ghr clean` shows cache statistics (repos, size, oldest entry) as a reminder
- Automatic eviction via `GH_CACHE_MAX_AGE` (e.g. `30d`): stale repositories are evicted on every invocation before the cache is touched
- Clones fetch only the default branch tip (`--depth 1 --single-branch`, explicit), and each refresh expires reflogs and prunes superseded objects so cached repositories stop growing over time

## [0.3.0] - 2026-10-28

### Changed
- **Complete rewrite in Rust** - same CLI surface, commands, flags, output formats and exit codes as the Go implementation
- Single dependency set (`regex`, `thiserror`); output files stay byte-compatible with the Go build (HTML-escaped JSON, `null` vs `[]` semantics)
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
