# Contributing to ghr

Thank you for your interest in contributing to ghr!

## Reporting Bugs and Suggesting Enhancements

Both are tracked as GitHub issues. Please check existing issues first, then include:

- A clear title and description
- Steps to reproduce (bugs) or the use case (enhancements)
- Expected vs actual behavior
- Environment details (OS, Rust version, ghr version)

## Pull Requests

### 1. Fork, Clone, Build

```bash
git clone https://github.com/syxc/gh-repo-cli.git
cd gh-repo-cli
cargo build
```

Requires Rust 1.85+ (defined in `Cargo.toml` as `rust-version`).

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name   # or fix/your-bug-fix
```

### 3. Make Changes

- Follow the existing code style; `cargo fmt` is the formatting authority
- Add or update tests for new behavior
- Keep changes focused on the request

### 4. Validate

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --locked
cargo +1.85.0 check --all-targets --all-features --locked
```

Install Rust 1.85.0 with rustup for the last check. CI runs tests on Linux, macOS and Windows, checks the MSRV, and builds five targets; formatting and clippy run on Linux.

### 5. Commit and Push

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```bash
git commit -m "feat: add support for .yml files"
git push origin feature/your-feature-name
```

Common prefixes: `feat:`, `fix:`, `docs:`, `test:`, `refactor:`, `chore:`, `perf:`.

Then open a pull request with a clear description and references to related issues.

## Project Structure

```
gh-repo-cli/
├── src/
│   ├── main.rs        # Entry point
│   ├── cli.rs         # Argument parsing, help/usage text, exit codes
│   ├── commands.rs    # The seven subcommand implementations
│   ├── config.rs      # Cache/output paths and proxy resolution
│   ├── cache.rs       # Cache validation, locking and retention
│   ├── git.rs         # Repo parsing, cached clones, repo metadata
│   ├── fsutil.rs      # Directory trees, file IO, size formatting
│   ├── search.rs      # Regex search and language detection
│   ├── output.rs      # Tables, trees, headers, error lines
│   ├── color.rs       # ANSI styling with NO_COLOR/tty handling
│   └── json.rs        # Go-compatible JSON writer (output file format)
├── .github/workflows/ # CI, code quality, release
├── docs/              # Guides (AI integration, testing, release)
├── Cargo.toml         # Package manifest and lint configuration
└── README.md
```

## Testing Guidelines

- Unit tests live next to the code in `#[cfg(test)]` modules; run them with `cargo test`
- Name tests like sentences describing the behavior, and test one behavior per test
- Table-driven cases use plain loops over `&[(input, expected)]`
- CLI behavior (help text, error strings, exit codes) is covered by `src/cli.rs` tests

## Documentation

- Update `README.md` (and `README.zh-CN.md`) for user-facing changes
- Update `CHANGELOG.md` for releases
- Document public items with `///` doc comments

## Getting Help

- Open an issue for bugs or questions
- See [docs/TESTING.md](docs/TESTING.md) for testing help

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
