# AGENTS.md

Guidance for AI coding agents working in this repository.

## What this is

ghr is a Rust CLI that clones GitHub repositories with `git` and analyzes them locally (languages, structure, code search, file reading). The CLI surface mirrors a legacy Go build; output compatibility is a maintained contract, not an accident.

## Commands

```bash
cargo test                                                        # unit tests
cargo fmt --all --check                                           # formatting (authoritative)
cargo clippy --all-targets --all-features --locked -- -D warnings  # lint, same as CI
cargo run -q -- analyze octocat/Hello-World                       # run locally
```

CI runs these plus a 5-target build matrix.

Local gotcha: when `~/.cargo/config.toml` sets a `rustc-wrapper` (e.g. kache), clippy can replay stale diagnostics and hide real lint errors. If local clippy is green but CI is red - or any clippy result looks stale - recheck without the wrapper before trusting either side:

```bash
CARGO_RUSTC_WRAPPER= cargo clippy --all-targets --all-features --locked -- -D warnings
```

Judge clippy by its EXIT CODE, never by grepping the output: shell wrappers on this machine (rtk shim, kache) reformat and summarize clippy results, so text-based checks undercount. Both misses in October 2026 (unreadable_literal, map_unwrap_or) were visible locally but read as clean through output parsing.

`cfg`-gated code is invisible to a host-only clippy run (the `similar_names` hit in the Linux-only gsettings path passed on macOS). Lint the CI target too:

```bash
CARGO_RUSTC_WRAPPER= cargo clippy --target aarch64-unknown-linux-musl --all-targets --all-features --locked -- -D warnings
```

## Layout

| File | Responsibility |
|------|----------------|
| `src/cli.rs` | Argument parsing, hand-maintained help/usage text, exit codes, Cobra-compatible error strings |
| `src/commands.rs` | Subcommand implementations; output order and wording matter |
| `src/git.rs` | Repo parsing, cached shallow clones, fetch+reset+prune, repo metadata |
| `src/cache.rs` | Cache scan, last-used detection, duration parsing, retention |
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
- Cache retention uses `.git/FETCH_HEAD` mtime as the last-used marker; eviction never removes the repository being used.
- Lints: pedantic is warn, CI denies warnings. Silence only with `#[expect(clippy::lint)]` plus a reason comment.
- `.cargo/config.toml` pins the aarch64-unknown-linux-musl linker to rust-lld; the release matrix needs it. Do not remove.
- Clone/update behavior can only be tested against a real git remote: create a local bare repo and clone with `file://` URLs (plain local paths ignore `--depth`).

## Testing

- Tests live next to the code in `#[cfg(test)] mod tests`; name them as behavior sentences (`search_files_skips_git_and_node_modules_directories`), one behavior per test.
- CLI parsing, help text and error strings are covered in `cli.rs` tests; filesystem tests use temp dirs and clean up after themselves.

## Releases

1. Bump `version` in `Cargo.toml`, move CHANGELOG `[Unreleased]` to `[x.y.z] - <date>`, commit on a branch, merge via PR.
2. Push tag `vX.Y.Z` on master; the Release workflow runs tests + clippy, builds 5 targets, and publishes `ghr_<version>_<os>_<arch>` archives plus `checksums.txt`.
3. `ghr --version` comes from `Cargo.toml`; keep the tag and the manifest in sync.

## Commits

Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`, `refactor:`, `test:`, `perf:`). Work on branches, merge via PR.
