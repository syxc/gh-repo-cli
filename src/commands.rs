//! The subcommand implementations. Output mirrors the original Go build
//! (including the "❌ Error:" line printed to stdout before an error
//! propagates to the CLI layer), plus the cache-retention features added
//! after the Rust rewrite.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::cache;
use crate::cli::{AnalyzeSections, Command};
use crate::color;
use crate::config::Config;
use crate::fsutil::{self, Kind};
use crate::git;
use crate::json::Json;
use crate::output;
use crate::search::{self, SearchOptions};

pub fn run(command: Command, config: &Config, no_cache: bool) -> Result<(), String> {
    let repo = match &command {
        Command::Analyze { repo, .. }
        | Command::Structure { repo, .. }
        | Command::Search { repo, .. }
        | Command::Read { repo, .. }
        | Command::Ls { repo, .. }
        | Command::Readme { repo } => Some(repo.as_str()),
        Command::Clean { repo, .. } => repo.as_deref(),
    };
    if let Some(repo) = repo {
        git::parse_repo(repo).map_err(|err| report(&err))?;
    }
    let _cache_lock = cache::CacheLock::acquire(&config.cache_dir)
        .map_err(|err| report(&fsutil::describe_io(&err, "lock", &config.cache_dir)))?;

    match command {
        Command::Analyze {
            repo,
            depth,
            sections,
            output,
        } => analyze(&repo, depth, sections, output.as_deref(), config, no_cache),
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

/// Statistics always cover this depth, like the original build, so the
/// counts do not shift when `--depth` changes the displayed tree.
const STATS_DEPTH: i64 = 2;

fn analyze(
    repo: &str,
    depth: i64,
    sections: AnalyzeSections,
    output_file: Option<&str>,
    config: &Config,
    no_cache: bool,
) -> Result<(), String> {
    println!("{}", color::blue(&format!("\n📊 Analyzing {repo}...\n")));

    let repo_path = clone_or_report(repo, config, no_cache)?;
    let info = if sections.info {
        git::get_repo_info(&repo_path)
    } else {
        None
    };

    let stats = if sections.needs_stats() {
        Some(
            fsutil::count_file_stats(&repo_path, STATS_DEPTH)
                .map_err(|err| report(&fsutil::traverse_error(&err, &repo_path)))?,
        )
    } else {
        None
    };
    let languages = stats
        .as_ref()
        .map(|stats| top_ten(sorted_by_count(&stats.languages)))
        .unwrap_or_default();
    let file_types = stats
        .as_ref()
        .map(|stats| top_ten(sorted_by_count(&stats.file_types)))
        .unwrap_or_default();

    let structure = if sections.structure {
        Some(
            fsutil::traverse_dir(&repo_path, depth, 0)
                .map_err(|err| report(&fsutil::traverse_error(&err, &repo_path)))?,
        )
    } else {
        None
    };

    if sections.info {
        output::print_header("📁 Repository Info");
        println!("   Name: {repo}");
        if let Some(info) = &info {
            println!("   Last Update: {}", color::escape_controls(&info.date));
            println!(
                "   Commit: {}",
                info.commit.chars().take(7).collect::<String>()
            );
        }
    }

    if sections.languages {
        output::print_header("\n💻 Top Languages");
        print_count_table("Language", "Files", &languages);
    }

    if sections.file_types {
        output::print_header("\n📄 File Types");
        print_count_table("Extension", "Count", &file_types);
    }

    if let Some(tree) = &structure {
        output::print_header(&format!("\n🌳 Directory Structure (depth={depth})"));
        output::display_tree(tree, "");
    }

    if let Some(output_file) = non_empty(output_file) {
        // Keys stay alphabetical; unselected sections are omitted entirely.
        let mut fields = Vec::new();
        if sections.file_types {
            fields.push(("fileTypes", count_json(&file_types)));
        }
        if sections.info {
            fields.push((
                "info",
                info.as_ref().map_or(Json::Null, git::RepoInfo::to_json),
            ));
        }
        if sections.languages {
            fields.push(("languages", count_json(&languages)));
        }
        fields.push(("repo", Json::Str(repo.to_owned())));
        if let Some(tree) = &structure {
            fields.push(("structure", fsutil::items_to_json(tree)));
        }

        fsutil::save_output(&Json::Object(fields), output_file)
            .map_err(|message| report(&message))?;
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
    let full_path = join_repo_path(&repo_path, file).map_err(|err| report(&err))?;

    let content = match fsutil::read_file_content(&full_path) {
        Ok(content) => content,
        Err(err) => {
            output::print_error(&format!("File not found: {file}\n"));
            println!("{}", color::gray("Available files in this directory:\n"));
            if let Ok(root) = repo_path.canonicalize() {
                let parent = full_path
                    .parent()
                    .and_then(|parent| parent.canonicalize().ok())
                    .filter(|parent| parent.starts_with(&root))
                    .unwrap_or(root);
                if let Ok(items) = fsutil::traverse_dir(&parent, 1, 0) {
                    for item in &items {
                        println!("  {}", color::escape_controls(&item.name));
                    }
                }
            }
            return Err(fsutil::read_error(&err, &full_path));
        }
    };

    println!("{}", color::blue(&format!("\n📄 {file}\n")));
    output::print_separator(80);
    println!("{}", escape_file_content(&content));
    output::print_separator(80);

    Ok(())
}

fn ls(repo: &str, path: &str, depth: i64, config: &Config, no_cache: bool) -> Result<(), String> {
    println!("{}", color::blue(&format!("\n📂 Listing {repo}:{path}\n")));

    let repo_path = clone_or_report(repo, config, no_cache)?;
    let full_path = join_repo_path(&repo_path, path).map_err(|err| report(&err))?;
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
        println!("  {icon} {}", color::escape_controls(&item.name));
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
    println!("{}", escape_file_content(&content));
    output::print_separator(80);

    Ok(())
}

fn find_readme(repo_path: &Path, sub_dir: &str) -> Option<(String, String)> {
    for name in README_NAMES {
        let sub = if sub_dir.is_empty() {
            name.to_owned()
        } else {
            format!("{sub_dir}/{name}")
        };
        let Ok(path) = join_repo_path(repo_path, &sub) else {
            continue;
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
        clean_stale(config, max_age)?;
    } else if let Some(repo) = repo {
        let repo_path = cache::repo_path(&config.cache_dir, repo).map_err(|err| report(&err))?;

        if repo_path.exists() {
            remove_and_report(&repo_path, &format!("✅ Cleaned cache for {repo}"))?;
        } else {
            println!(
                "{}",
                color::yellow(&format!("⚠️  No cache found for {repo}"))
            );
        }
    } else if all {
        let entries = cache::scan(&config.cache_dir);
        if entries.is_empty() {
            println!("{}", color::yellow("⚠️  No cache found"));
        } else {
            let mut size = 0;
            for entry in entries {
                let path =
                    cache::repo_path(&config.cache_dir, &entry.name).map_err(|err| report(&err))?;
                size += fsutil::directory_size(&path);
                std::fs::remove_dir_all(&path)
                    .map_err(|err| report(&fsutil::describe_io(&err, "remove", &path)))?;
            }
            println!(
                "{}",
                color::green(&format!(
                    "✅ Cleaned all cache ({})",
                    fsutil::format_bytes(size)
                ))
            );
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
        cache::repo_path(&config.cache_dir, &entry.name).map_err(|err| report(&err))?;
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
    cache::repo_path(&config.cache_dir, repo).map_err(|err| report(&err))?;
    auto_clean_cache(config, repo);
    git::clone_repo(repo, &config.cache_dir, &config.proxy, no_cache).map_err(|err| report(&err))
}

/// Opt-in retention removes unused repositories before clone/fetch.
/// The caller holds the cache lock throughout cleanup and repository use.
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
                let result = cache::repo_path(&config.cache_dir, &entry.name).and_then(|path| {
                    std::fs::remove_dir_all(&path)
                        .map_err(|err| fsutil::describe_io(&err, "remove", &path))
                });
                match result {
                    Ok(()) => println!(
                        "{}",
                        color::green(&format!(
                            "✅ Auto-cleaned cache for {} (unused {})",
                            entry.name,
                            cache::format_age(entry.age())
                        ))
                    ),
                    Err(message) => eprintln!("Warning: failed to clean {}: {message}", entry.name),
                }
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

fn escape_file_content(content: &str) -> String {
    let mut escaped = String::with_capacity(content.len());
    for ch in content.chars() {
        if ch.is_control() && !matches!(ch, '\n' | '\r' | '\t') {
            escaped.extend(ch.escape_default());
        } else {
            escaped.push(ch);
        }
    }
    escaped
}

fn non_empty(value: Option<&str>) -> Option<&str> {
    value.filter(|value| !value.is_empty())
}

/// Keeps normalized paths and symlink targets within the repository.
fn join_repo_path(base: &Path, sub: &str) -> Result<PathBuf, String> {
    let invalid = || format!("path is outside the repository: {sub}");
    let mut relative = PathBuf::new();
    for component in Path::new(sub).components() {
        match component {
            std::path::Component::Normal(part) => relative.push(part),
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir if relative.pop() => {}
            _ => return Err(invalid()),
        }
    }
    let root = base
        .canonicalize()
        .map_err(|err| fsutil::describe_io(&err, "open", base))?;
    let path = root.join(relative);
    let mut existing = path.as_path();
    loop {
        match existing.canonicalize() {
            Ok(resolved) if resolved.starts_with(&root) => break,
            Ok(_) => return Err(invalid()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                existing = existing.parent().ok_or_else(invalid)?;
            }
            Err(err) => return Err(fsutil::describe_io(&err, "open", existing)),
        }
    }

    Ok(path)
}

fn sorted_by_count(counts: &HashMap<String, i64>) -> Vec<(String, i64)> {
    let mut pairs: Vec<(String, i64)> = counts
        .iter()
        .map(|(key, value)| (key.clone(), *value))
        .collect();
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
    fn file_content_escapes_terminal_controls_and_keeps_text_layout() {
        assert_eq!(
            escape_file_content("a\n\tb\r\n\u{1b}]52;c;payload\u{7}"),
            "a\n\tb\r\n\\u{1b}]52;c;payload\\u{7}"
        );
    }

    #[test]
    fn sorted_by_count_orders_values_descending_with_key_tiebreak() {
        let counts = HashMap::from([
            ("go".to_owned(), 10),
            ("js".to_owned(), 5),
            ("python".to_owned(), 15),
            ("rust".to_owned(), 3),
        ]);

        let sorted = sorted_by_count(&counts);
        let keys: Vec<&str> = sorted.iter().map(|(key, _)| key.as_str()).collect();

        assert_eq!(keys, vec!["python", "go", "js", "rust"]);
    }

    #[test]
    fn sorted_by_count_breaks_ties_by_key() {
        let counts = HashMap::from([("b".to_owned(), 1), ("a".to_owned(), 1)]);
        let sorted = sorted_by_count(&counts);

        assert_eq!(sorted[0].0, "a");
    }

    #[test]
    fn sorted_by_count_handles_empty_input() {
        assert_eq!(sorted_by_count(&HashMap::new()), Vec::new());
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
