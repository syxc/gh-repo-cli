# Testing Guide

## Overview

This document describes the testing setup for ghr (Rust).

## Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run one module's tests
cargo test cli

# Run a single test
cargo test parse_repo_accepts_owner_and_name
```

Tests are `#[cfg(test)]` modules next to the code they cover (`src/*.rs`).

## Lint and Format

```bash
# Format (authoritative)
cargo fmt --all

# Lint with warnings as errors (same as CI)
cargo clippy --all-targets --all-features --locked -- -D warnings
```

Lint configuration lives in `Cargo.toml` under `[lints]`. Silence a lint only
with `#[expect(...)]` plus a comment explaining why.

## Test Layout

| Module | Covers |
|--------|--------|
| `src/cli.rs` | Argument parsing, help text, error strings, exit codes |
| `src/config.rs` | Proxy resolution order, system-proxy parsing |
| `src/git.rs` | Repo format parsing, clone URLs, proxy git args |
| `src/fsutil.rs` | Directory traversal, file IO, output saving, size formatting |
| `src/search.rs` | Regex search, extension filter, language detection |
| `src/output.rs` | Table and tree rendering |
| `src/json.rs` | Go-compatible JSON escaping and shapes |
| `src/commands.rs` | Counting, sorting, truncation helpers |

## Guidelines

- Name tests as sentences: `search_files_skips_git_and_node_modules_directories`
- One behavior per test; share setup helpers, not assertions
- Tests use temp directories under `std::env::temp_dir()` and clean up after themselves
- Deterministic behavior matters: outputs (JSON key order, tie-breaking) are
  asserted byte-for-byte so the output files stay stable across versions

## CI

GitHub Actions runs on every push and pull request:

- **CI**: formatting, clippy (`-D warnings`), tests, cross-platform builds
- **Code Quality**: weekly and on push; also checks for TODO comments and oversized files
- **Release**: on version tags, runs tests and clippy before building archives

## Troubleshooting

- **Tests fail on a slow machine**: traversal tests use their own temp dirs; if a
  previous run left stale state, delete `ghr-*` directories in your temp folder.
- **Clippy failures**: run `cargo clippy --all-targets --all-features` locally and
  fix, or justify an `#[expect(...)]` with a comment.
- **Lockfile issues**: run `cargo update --locked` errors mean `Cargo.lock` is out
  of date; commit the refreshed `Cargo.lock`.
