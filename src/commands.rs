//! The subcommand implementations. Output mirrors the original Go build
//! (including the "❌ Error:" line printed to stdout before an error
//! propagates to the CLI layer), plus the cache-retention features added
//! after the Rust rewrite.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::cache;
use crate::cli::Command;
use crate::color;
use crate::config::Config;
use crate::fsutil::{self, Kind};
use crate::git;
use crate::json::Json;
use crate::output;
use crate::search::{self, SearchOptions};

pub fn run(command: Command, config: &Config, no_cache: bool) -> Result<(), String> {
    match command {
        Command::Analyze { repo, output } => analyze(&repo, output.as_deref(), config, no_cache),
        Command::Structure {
            repo,
            depth,
            output,
        } => structure(&repo, depth, output.as_deref(), config, no_cache),
        Command::Search {
            repo,
            query,
            ext,
            ignore_case,
            output,
        } => search_command(
            &repo,
            &query,
            ext.as_deref(),
            ignore_case,
            output.as_deref(),
            config,
            no_cache,
        ),
        Command::Read { repo, file } => read(&repo, &file, config, no_cache),
        Command::Ls { repo, path, depth } => ls(&repo, &path, depth, config, no_cache),
        Command::Readme { repo } => readme(&repo, config, no_cache),
        Command::Clean {
            repo,
            all,
            output,
            old,
        } => clean(repo.as_deref(), all, output, old.as_deref(), config),
    }
}

fn analyze(
    repo: &str,
    output_file: Option<&str>,
    config: &Config,
    no_cache: bool,
) -> Result<(), String> {
    println!("{}", color::blue(&format!("\n📊 Analyzing {repo}...\n")));

    let repo_path = clone_or_report(repo, config, no_cache)?;
    let info = git::get_repo_info(&repo_path);
    let structure = fsutil::traverse_dir(&repo_path, 2, 0)
        .map_err(|err| report(&fsutil::traverse_error(&err, &repo_path)))?;

    let mut file_types = HashMap::new();
    let mut languages = HashMap::new();
    fsutil::count_file_types(&structure, &mut file_types, &mut languages);
    // Like the Go build, only the top 10 reach the tables and the JSON output.
    let languages = top_ten(sorted_by_count(languages));
    let file_types = top_ten(sorted_by_count(file_types));

    output::print_header("📁 Repository Info");
    println!("   Name: {repo}");
    if let Some(info) = &info {
        println!("   Last Update: {}", info.date);
        println!(
            "   Commit: {}",
            info.commit.chars().take(7).collect::<String>()
        );
    }

    output::print_header("\n💻 Top Languages");
    print_count_table("Language", "Files", &languages);

    output::print_header("\n📄 File Types");
    print_count_table("Extension", "Count", &file_types);

    output::print_header("\n🌳 Directory Structure (depth=2)");
    output::display_tree(&structure, "");

    if let Some(output_file) = non_empty(output_file) {
        let payload = Json::Object(vec![
            ("fileTypes", count_json(&file_types)),
            (
                "info",
                info.as_ref().map_or(Json::Null, git::RepoInfo::to_json),
            ),
            ("languages", count_json(&languages)),
            ("repo", Json::Str(repo.to_owned())),
            ("structure", fsutil::items_to_json(&structure)),
        ]);
        fsutil::save_output(&payload, output_file).map_err(|message| report(&message))?;
        println!(
            "{}",
            color::green(&format!("\n✅ Output saved to {output_file}"))
        );
    }

    Ok(())
}

fn structure(
    repo: &str,
    depth: i64,
    output_file: Option<&str>,
    config: &Config,
    no_cache: bool,
) -> Result<(), String> {
    println!(
        "{}",
        color::blue(&format!("\n🌳 Getting structure of {repo}...\n"))
    );

    let repo_path = clone_or_report(repo, config, no_cache)?;
    let tree = fsutil::traverse_dir(&repo_path, depth, 0)
        .map_err(|err| report(&fsutil::traverse_error(&err, &repo_path)))?;
    output::display_tree(&tree, "");

    if let Some(output_file) = non_empty(output_file) {
        fsutil::save_output(&fsutil::items_to_json(&tree), output_file)
            .map_err(|message| report(&message))?;
        println!(
            "{}",
            color::green(&format!("\n✅ Structure saved to {output_file}"))
        );
    }

    Ok(())
}

fn search_command(
    repo: &str,
    query: &str,
    ext: Option<&str>,
    ignore_case: bool,
    output_file: Option<&str>,
    config: &Config,
    no_cache: bool,
) -> Result<(), String> {
    println!(
        "{}",
        color::blue(&format!("\n🔍 Searching in {repo} for: \"{query}\"\n"))
    );

    let repo_path = clone_or_report(repo, config, no_cache)?;
    let options = SearchOptions {
        ext: non_empty(ext).map(str::to_owned),
        ignore_case,
    };
    let results = search::search_files(&repo_path, query, &options).map_err(|err| report(&err))?;

    if results.is_empty() {
        println!("{}", color::yellow("No matches found.\n"));
        return Ok(());
    }

    println!(
        "{}",
        color::green(&format!("Found {} matches:\n", results.len()))
    );

    let rows: Vec<Vec<String>> = results
        .iter()
        .take(50)
        .map(|result| {
            vec![
                result.file.clone(),
                result.line.to_string(),
                truncate_chars(&result.text, 75),
            ]
        })
        .collect();
    output::print_table(&["File", "Line", "Match"], &rows, &[40, 8, 80]);

    if results.len() > 50 {
        println!(
            "{}",
            color::gray(&format!("\n... and {} more matches", results.len() - 50))
        );
    }

    if let Some(output_file) = non_empty(output_file) {
        fsutil::save_output(&search::results_to_json(&results), output_file)
            .map_err(|message| report(&message))?;
        println!(
            "{}",
            color::green(&format!(
                "\n✅ All {} results saved to {output_file}",
                results.len()
            ))
        );
    }

    Ok(())
}

fn read(repo: &str, file: &str, config: &Config, no_cache: bool) -> Result<(), String> {
    let repo_path = clone_or_report(repo, config, no_cache)?;
    let full_path = join_repo_path(&repo_path, file);

    let content = match fsutil::read_file_content(&full_path) {
        Ok(content) => content,
        Err(err) => {
            output::print_error(&format!("File not found: {file}\n"));
            println!("{}", color::gray("Available files in this directory:\n"));
            if let Some(parent) = full_path.parent() {
                if let Ok(items) = fsutil::traverse_dir(parent, 1, 0) {
                    for item in &items {
                        println!("  {}", item.name);
                    }
                }
            }
            return Err(fsutil::read_error(&err, &full_path));
        }
    };

    println!("{}", color::blue(&format!("\n📄 {file}\n")));
    output::print_separator(80);
    println!("{content}");
    output::print_separator(80);

    Ok(())
}

fn ls(repo: &str, path: &str, depth: i64, config: &Config, no_cache: bool) -> Result<(), String> {
    println!("{}", color::blue(&format!("\n📂 Listing {repo}:{path}\n")));

    let repo_path = clone_or_report(repo, config, no_cache)?;
    let full_path = join_repo_path(&repo_path, path);
    let items = fsutil::traverse_dir(&full_path, depth, 0)
        .map_err(|err| report(&fsutil::traverse_error(&err, &full_path)))?;

    if items.is_empty() {
        println!("{}", color::gray("(empty directory)\n"));
        return Ok(());
    }

    for item in &items {
        let icon = match item.kind {
            Kind::Directory => "📁",
            Kind::File => "📄",
        };
        println!("  {icon} {}", item.name);
    }
    println!();

    Ok(())
}

const README_NAMES: [&str; 6] = [
    "README.md",
    "readme.md",
    "README.MD",
    "README.markdown",
    "README.txt",
    "README",
];

fn readme(repo: &str, config: &Config, no_cache: bool) -> Result<(), String> {
    println!(
        "{}",
        color::blue(&format!("\n📖 Getting README from {repo}...\n"))
    );

    let repo_path = clone_or_report(repo, config, no_cache)?;

    let mut found = find_readme(&repo_path, "");
    // Like the Go build: an existing but empty README falls through to docs/.
    if found.as_ref().is_none_or(|(_, content)| content.is_empty()) {
        found = find_readme(&repo_path, "docs");
    }

    let Some((name, content)) = found.filter(|(_, content)| !content.is_empty()) else {
        println!("{}", color::yellow("No README found.\n"));
        return Ok(());
    };

    println!("{}", color::bold(&name));
    println!();
    output::print_separator(80);
    println!("{content}");
    output::print_separator(80);

    Ok(())
}

fn find_readme(repo_path: &Path, sub_dir: &str) -> Option<(String, String)> {
    for name in README_NAMES {
        let path = if sub_dir.is_empty() {
            repo_path.join(name)
        } else {
            repo_path.join(sub_dir).join(name)
        };
        if let Ok(content) = fsutil::read_file_content(&path) {
            let label = if sub_dir.is_empty() {
                name.to_owned()
            } else {
                format!("{sub_dir}/{name}")
            };
            return Some((label, content));
        }
    }

    None
}

fn clean(
    repo: Option<&str>,
    all: bool,
    output_dir: bool,
    old: Option<&str>,
    config: &Config,
) -> Result<(), String> {
    let max_age = match old {
        Some(raw) => {
            if all || repo.is_some() {
                return Err(
                    "the --old flag cannot be combined with --all or a repository argument"
                        .to_owned(),
                );
            }
            Some(cache::parse_duration(raw)?)
        }
        None => None,
    };
    if let Some(max_age) = max_age {
        return clean_stale(config, max_age);
    }

    if let Some(repo) = repo {
        let (owner, name) = git::parse_repo(repo).map_err(|err| report(&err))?;
        let repo_path = config.cache_dir.join(owner).join(name);

        if repo_path.exists() {
            remove_and_report(&repo_path, &format!("✅ Cleaned cache for {repo}"))?;
        } else {
            println!(
                "{}",
                color::yellow(&format!("⚠️  No cache found for {repo}"))
            );
        }
    } else if all {
        if config.cache_dir.exists() {
            let size = fsutil::directory_size(&config.cache_dir);
            remove_and_report(
                &config.cache_dir,
                &format!("✅ Cleaned all cache ({})", fsutil::format_bytes(size)),
            )?;
        } else {
            println!("{}", color::yellow("⚠️  No cache found"));
        }
    } else {
        println!(
            "{}",
            color::yellow("⚠️  Use --all flag to clean all cached repositories")
        );
        println!(
            "{}",
            color::gray("   Or specify a repository: ghr clean owner/repo")
        );
        println!(
            "{}",
            color::gray("   Or clean stale entries:   ghr clean --old 30d")
        );
        print_cache_stats(config);
    }

    if output_dir && config.output_dir.exists() {
        remove_and_report(&config.output_dir, "✅ Cleaned output directory")?;
    }

    Ok(())
}

/// Removes cached repositories whose last use is older than `max_age`.
fn clean_stale(config: &Config, max_age: std::time::Duration) -> Result<(), String> {
    let stale = cache::stale_entries(&config.cache_dir, max_age, None);
    if stale.is_empty() {
        println!(
            "{}",
            color::yellow(&format!(
                "⚠️  No cached repos unused for {} or more",
                cache::format_age(max_age)
            ))
        );
        return Ok(());
    }

    let mut total = 0;
    for entry in &stale {
        let size = fsutil::directory_size(&entry.path);
        total += size;
        remove_and_report(
            &entry.path,
            &format!(
                "✅ Cleaned cache for {} (unused {}, {})",
                entry.name,
                cache::format_age(entry.age()),
                fsutil::format_bytes(size)
            ),
        )?;
    }
    println!(
        "{}",
        color::green(&format!(
            "✅ Cleaned {} repos ({})",
            stale.len(),
            fsutil::format_bytes(total)
        ))
    );

    Ok(())
}

/// Reminder shown by bare `ghr clean`: what the cache holds right now.
fn print_cache_stats(config: &Config) {
    let entries = cache::scan(&config.cache_dir);
    if entries.is_empty() {
        return;
    }
    let size: u64 = entries
        .iter()
        .map(|entry| fsutil::directory_size(&entry.path))
        .sum();
    let oldest = cache::format_age(entries[0].age());
    println!(
        "{}",
        color::gray(&format!(
            "   Cache: {} repos, {} (oldest last used {} ago)",
            entries.len(),
            fsutil::format_bytes(size),
            oldest
        ))
    );
}

fn remove_and_report(path: &Path, success_message: &str) -> Result<(), String> {
    match std::fs::remove_dir_all(path) {
        Ok(()) => {
            println!("{}", color::green(success_message));
            Ok(())
        }
        Err(err) => Err(report(&fsutil::describe_io(&err, "remove", path))),
    }
}

fn clone_or_report(repo: &str, config: &Config, no_cache: bool) -> Result<PathBuf, String> {
    auto_clean_cache(config, repo);
    git::clone_repo(repo, &config.cache_dir, &config.proxy, no_cache).map_err(|err| report(&err))
}

/// Opt-in LRU-style retention: when `GH_CACHE_MAX_AGE` is set (e.g. `30d`),
/// cached repositories unused for that long are evicted before every
/// clone/fetch, keeping the cache bounded without a separate command.
fn auto_clean_cache(config: &Config, current: &str) {
    let Some(raw) = std::env::var_os("GH_CACHE_MAX_AGE") else {
        return;
    };
    let raw = raw.to_string_lossy();
    if raw.is_empty() {
        return;
    }

    match cache::parse_duration(&raw) {
        Ok(max_age) => {
            for entry in cache::stale_entries(&config.cache_dir, max_age, Some(current)) {
                let _ = std::fs::remove_dir_all(&entry.path);
                println!(
                    "{}",
                    color::green(&format!(
                        "✅ Auto-cleaned cache for {} (unused {})",
                        entry.name,
                        cache::format_age(entry.age())
                    ))
                );
            }
        }
        Err(message) => println!(
            "{}",
            color::yellow(&format!("⚠️  Ignoring invalid GH_CACHE_MAX_AGE: {message}"))
        ),
    }
}

/// Prints the "❌ Error:" line for `err` and returns its message so the CLI
/// layer adds the Cobra-style "Error:" line on stderr.
fn report<E: ToString>(err: &E) -> String {
    let message = err.to_string();
    output::print_error(&message);
    message
}

fn non_empty(value: Option<&str>) -> Option<&str> {
    value.filter(|value| !value.is_empty())
}

/// Mirrors Go's `filepath.Join`: appends `sub` to `base` and cleans the
/// result lexically ("." and empty segments drop, ".." pops one segment).
fn join_repo_path(base: &Path, sub: &str) -> PathBuf {
    let mut parts: Vec<String> = base
        .iter()
        .map(|segment| segment.to_string_lossy().into_owned())
        .collect();
    for segment in sub.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            other => parts.push(other.to_owned()),
        }
    }

    parts.into_iter().collect()
}

fn sorted_by_count(counts: HashMap<String, i64>) -> Vec<(String, i64)> {
    let mut pairs: Vec<(String, i64)> = counts.into_iter().collect();
    // Value descending; the key tie-break keeps output deterministic where
    // the Go build relied on unstable map iteration order.
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    pairs
}

/// Keeps the first 10 entries, matching the Go build's table and JSON cap.
fn top_ten(mut pairs: Vec<(String, i64)>) -> Vec<(String, i64)> {
    pairs.truncate(10);
    pairs
}

fn print_count_table(first_header: &str, second_header: &str, pairs: &[(String, i64)]) {
    let rows: Vec<Vec<String>> = pairs
        .iter()
        .map(|(key, value)| vec![key.clone(), value.to_string()])
        .collect();
    output::print_table(&[first_header, second_header], &rows, &[30, 15]);
}

fn count_json(pairs: &[(String, i64)]) -> Json {
    Json::Array(
        pairs
            .iter()
            .map(|(key, value)| {
                Json::Object(vec![
                    ("key", Json::Str(key.clone())),
                    ("value", Json::Int(*value)),
                ])
            })
            .collect(),
    )
}

fn truncate_chars(text: &str, max: usize) -> String {
    if text.chars().count() > max {
        return format!("{}...", text.chars().take(max).collect::<String>());
    }

    text.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted_by_count_orders_values_descending_with_key_tiebreak() {
        let counts = HashMap::from([
            ("go".to_owned(), 10),
            ("js".to_owned(), 5),
            ("python".to_owned(), 15),
            ("rust".to_owned(), 3),
        ]);

        let sorted = sorted_by_count(counts);
        let keys: Vec<&str> = sorted.iter().map(|(key, _)| key.as_str()).collect();

        assert_eq!(keys, vec!["python", "go", "js", "rust"]);
    }

    #[test]
    fn sorted_by_count_breaks_ties_by_key() {
        let sorted = sorted_by_count(HashMap::from([("b".to_owned(), 1), ("a".to_owned(), 1)]));

        assert_eq!(sorted[0].0, "a");
    }

    #[test]
    fn sorted_by_count_handles_empty_input() {
        assert_eq!(sorted_by_count(HashMap::new()), Vec::new());
    }

    #[test]
    fn top_ten_caps_lists_at_ten_entries_like_go() {
        let pairs: Vec<(String, i64)> = (0..15).map(|i| (format!("k{i}"), i)).collect();

        assert_eq!(top_ten(pairs).len(), 10);
        assert_eq!(top_ten(Vec::new()).len(), 0);
    }

    #[test]
    fn truncate_chars_limits_by_characters_with_ellipsis() {
        assert_eq!(truncate_chars("short", 10), "short");
        assert_eq!(truncate_chars("gopher gopher", 6), "gopher...");
        assert_eq!(truncate_chars("中文中文中文", 4), "中文中文...");
    }

    #[test]
    fn non_empty_rejects_empty_flag_values() {
        assert_eq!(non_empty(Some("out.json")), Some("out.json"));
        assert_eq!(non_empty(Some("")), None);
        assert_eq!(non_empty(None), None);
    }

    #[test]
    fn count_json_renders_key_value_objects() {
        assert_eq!(
            count_json(&[("Go".to_owned(), 3)]).marshal_indent(),
            "[\n  {\n    \"key\": \"Go\",\n    \"value\": 3\n  }\n]"
        );
    }
}
