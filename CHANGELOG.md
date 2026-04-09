# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-04-09

### Changed
- **Complete rewrite in Go** - Migrated from Node.js to Go for better performance and distribution
- New installation methods: `go install`, pre-built binaries, or build from source
- Updated all GitHub Actions workflows for Go build process
- Updated documentation to reflect Go-based installation and development

### Added
- Cross-platform binary releases for Linux, macOS, and Windows (amd64/arm64)
- Improved error handling and resource management
- Better CJK (Chinese, Japanese, Korean) character support in search and display

### Removed
- Node.js dependencies and npm-based installation (replaced by Go binaries)
- Jest testing framework (replaced by Go's built-in testing)
- ESLint configuration (replaced by go vet and golint)

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
- Testing framework (Jest)
- ESLint code quality checks

## [Unreleased]

### Planned
- Enhanced error handling
- More configuration options
- Performance optimizations
- Additional output formats

### Changed
- Add `.npmignore` to exclude development files from package
- Add `files` field to package.json for explicit publish control
- Update README with manual installation instructions (git clone + npm link)
- Add `prepublishOnly` and `prepack` scripts for quality checks

## [1.0.0] - 2026-02-02

### Added
- Initial release
- Repository analysis with language detection
- Code search functionality
- Directory structure visualization
- File reading capability
- HTTP/HTTPS/SOCKS5 proxy support
- Local caching system
- JSON output support
- Comprehensive documentation with Chinese translations
- AI integration guides for Claude Code
- Testing framework and CI/CD workflows

### Changed
- Move CONTRIBUTING.md to root directory for better accessibility
- Optimize documentation structure and organization
- Improve AI_INTEGRATION guides with practical examples
- Separate English and Chinese documentation for better clarity
