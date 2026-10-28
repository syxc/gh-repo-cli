# AGENTS.md

Guidance for AI coding agents working in this repository.

## What this is

ghr is a Rust CLI that clones GitHub repositories with `git` and analyzes them locally (languages, structure, code search, file reading). The CLI surface mirrors a legacy Go build; output compatibility is a maintained contract, not an accident.

## Commands

```bash
cargo test --locked                                               # unit tests
cargo fmt --all --check                                           # formatting (authoritative)
cargo clippy --all-targets --all-features --locked -- -D warnings  # lint, same as CI
cargo run -q -- analyze octocat/Hello-World                       # run locally
```

CI runs tests on Linux, macOS and Windows, checks Rust 1.85 with `--locked`, and builds five targets. Formatting and clippy run on Linux.

If local clippy and CI disagree, compare toolchains and targets, then disable any configured `rustc-wrapper` to isolate its effect:

```bash
RUSTC_WRAPPER= cargo clippy --all-targets --all-features --locked -- -D warnings
```

Use clippy's exit code to determine success. Summarized output is not a substitute for the exit code.

A host-only clippy run excludes code behind other platform `cfg` conditions. Check changed platform code on its target too:

```bash
RUSTC_WRAPPER= cargo clippy --target aarch64-unknown-linux-musl --all-targets --all-features --locked -- -D warnings
```

## Layout

| File | Responsibility |
|------|----------------|
| `src/cli.rs` | Argument parsing, hand-maintained help/usage text, exit codes, Cobra-compatible error strings |
| `src/commands.rs` | Subcommand implementations; output order and wording matter |
| `src/git.rs` | Repo parsing, shallow clone/refresh, automatic Git maintenance, repo metadata |
| `src/cache.rs` | Cache validation, locking, last-used detection, duration parsing, retention |
| `src/fsutil.rs` | Directory trees, file IO, size formatting, Go-style IO error text |
| `src/search.rs` | Regex search, extension filter, language table |
| `src/output.rs` | Tables, trees, headers (character-based padding) |
| `src/color.rs` | ANSI codes with NO_COLOR/TERM/tty rules; bold resets with 22, colors with 0 |
| `src/json.rs` | Go-compatible JSON writer (HTML escaping, field order, `null` vs `[]`) |
| `src/config.rs` | Cache/output paths, proxy resolution chain |

## Conventions that are easy to break

- Output is a contract: emoji prefixes, blank lines, table borders and error strings were byte-compared against the Go build. When changing output, check the tests in `cli.rs`/`commands.rs` and update README (both languages).
- Help/usage texts in `cli.rs` are hand-maintained to match Cobra templates. A new flag needs both a `FlagDef` entry and a row in the command's usage constant, in Cobra's alphabetical flag order.
- Error flow: print `❌ Error: ...` to stdout at the error site (`report()`/`print_error`), return `Err(message)`; the cli layer adds `Error: <msg>` plus usage on stderr and exits 1.
- JSON output files must stay Go-compatible: no serde; extend `json.rs` (HTML escaping of `<>&`, declared field order, `null` vs `[]`).
- Cache commands share one file lock per cache root and wait at most 30 seconds. Cleanup removes only verified GitHub repository directories, preserving the cache root and unrelated files; owner/repo symlinks are rejected. Adopt a `GH_CACHE_DIR` only if it is empty or a recognized existing cache.
- Refresh with `git fetch --depth 1 --no-tags origin HEAD` and reset to the fetched default branch. Use `git gc --auto` with Git's normal expiry policy; do not force full pruning on each refresh.
- Keep `read`, `ls`, `search` and `readme` within the repository boundary, including symbolic-link targets. `analyze` statistics cover depth 2 and retain the top 10 entries per table; JSON always includes `repo`.
- Lints: pedantic is warn, CI denies warnings. Silence only with `#[expect(clippy::lint)]` plus a reason comment.
- `.cargo/config.toml` pins the aarch64-unknown-linux-musl linker to rust-lld; the release matrix needs it. Do not remove.
- Clone/update behavior can only be tested against a real git remote: create a local bare repo and clone with `file://` URLs (plain local paths ignore `--depth`).

## Testing

- Tests live next to the code in `#[cfg(test)] mod tests`; name them as behavior sentences (`search_files_skips_git_and_node_modules_directories`), one behavior per test.
- CLI parsing, help text and error strings are covered in `cli.rs` tests; filesystem tests use temp dirs and clean up after themselves.

## Releases

1. Bump `version` in `Cargo.toml`, refresh `Cargo.lock`, and move CHANGELOG `[Unreleased]` entries to `[x.y.z] - <date>`. Validate, commit on a branch, and merge via PR.
2. Push tag `vX.Y.Z` on master. Release rejects a tag/manifest mismatch or missing release notes, runs tests and clippy, then builds five targets and publishes archives plus `checksums.txt`.
3. `ghr --version` comes from `Cargo.toml`. Notes use `RUNNER_TEMP` and link to the tagged changelog; see `docs/RELEASE_WORKFLOW.md` for local metadata checks.

## Commits

Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`, `refactor:`, `test:`, `perf:`). Work on branches, merge via PR.
