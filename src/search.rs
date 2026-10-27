//! Line-based regex search over a directory tree, mirroring the Go build's
//! `filepath.WalkDir` behavior: lexical order, pruned `.git`/`node_modules`
//! directories, per-line matches with trimmed text.

use std::path::Path;

use regex::Regex;

use crate::json::Json;

#[derive(Debug)]
pub struct SearchResult {
    pub file: String,
    pub line: i64,
    pub text: String,
    pub matches: Vec<String>,
}

/// How many leading bytes are sniffed for NUL to mark a file binary.
const BINARY_SNIFF_LEN: usize = 8 * 1024;

#[derive(Debug, Default)]
pub struct SearchOptions {
    pub ext: Option<String>,
    pub ignore_case: bool,
}

pub fn search_files(
    dir: &Path,
    pattern: &str,
    options: &SearchOptions,
) -> Result<Vec<SearchResult>, regex::Error> {
    let pattern = if options.ignore_case {
        format!("(?i){pattern}")
    } else {
        pattern.to_owned()
    };
    let regex = Regex::new(&pattern)?;

    let mut results = Vec::new();
    walk(dir, dir, &regex, options, &mut results);

    Ok(results)
}

fn walk(
    root: &Path,
    dir: &Path,
    regex: &Regex,
    options: &SearchOptions,
    results: &mut Vec<SearchResult>,
) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<std::fs::DirEntry> = entries.flatten().collect();
    entries.sort_by_key(std::fs::DirEntry::file_name);

    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == ".git" || name == "node_modules" {
            continue;
        }

        let path = dir.join(&name);
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_dir() {
            walk(root, &path, regex, options, results);
            continue;
        }

        if options
            .ext
            .as_ref()
            .is_some_and(|ext| !name.ends_with(ext.as_str()))
        {
            continue;
        }

        let Ok(bytes) = std::fs::read(&path) else {
            continue;
        };
        // NUL sniff like grep: binary files only produce mojibake matches.
        if bytes.iter().take(BINARY_SNIFF_LEN).any(|&byte| byte == 0) {
            continue;
        }
        let content = String::from_utf8_lossy(&bytes);
        // Line counts fit i64 for any real file; the Go build used int as well.
        #[expect(clippy::cast_possible_wrap)]
        for (index, line) in content.split('\n').enumerate() {
            if regex.is_match(line) {
                let relative = path.strip_prefix(root).unwrap_or(&path);
                results.push(SearchResult {
                    file: relative.to_string_lossy().into_owned(),
                    line: index as i64 + 1,
                    text: line.trim().to_owned(),
                    matches: regex
                        .find_iter(line)
                        .map(|m| m.as_str().to_owned())
                        .collect(),
                });
            }
        }
    }
}

pub fn results_to_json(results: &[SearchResult]) -> Json {
    Json::Array(
        results
            .iter()
            .map(|result| {
                Json::Object(vec![
                    ("file", Json::Str(result.file.clone())),
                    ("line", Json::Int(result.line)),
                    ("text", Json::Str(result.text.clone())),
                    (
                        "matches",
                        Json::Array(
                            result
                                .matches
                                .iter()
                                .map(|m| Json::Str(m.clone()))
                                .collect(),
                        ),
                    ),
                ])
            })
            .collect(),
    )
}

/// Maps file extensions to language names, matching the Go build's table.
pub fn detect_language(filename: &str) -> Option<&'static str> {
    const LANGUAGES: &[(&str, &str)] = &[
        (".js", "JavaScript"),
        (".ts", "TypeScript"),
        (".jsx", "JavaScript React"),
        (".tsx", "TypeScript React"),
        (".py", "Python"),
        (".java", "Java"),
        (".cpp", "C++"),
        (".c", "C"),
        (".cs", "C#"),
        (".go", "Go"),
        (".rs", "Rust"),
        (".php", "PHP"),
        (".rb", "Ruby"),
        (".swift", "Swift"),
        (".kt", "Kotlin"),
        (".scala", "Scala"),
        (".sh", "Shell"),
        (".bash", "Bash"),
        (".zsh", "Zsh"),
        (".html", "HTML"),
        (".css", "CSS"),
        (".scss", "SCSS"),
        (".less", "Less"),
        (".json", "JSON"),
        (".xml", "XML"),
        (".yaml", "YAML"),
        (".yml", "YAML"),
        (".toml", "TOML"),
        (".md", "Markdown"),
        (".txt", "Text"),
    ];

    // A single leading dot is a dotfile, not an extension ("README" style
    // names have no extension at all and yield no language).
    if filename.matches('.').count() == 1 && filename.starts_with('.') {
        return lookup_language(LANGUAGES, filename);
    }

    let dot = filename.rfind('.')?;
    let extension = filename[dot..].to_lowercase();
    lookup_language(LANGUAGES, &extension)
}

fn lookup_language(
    languages: &'static [(&'static str, &'static str)],
    key: &str,
) -> Option<&'static str> {
    languages
        .iter()
        .find(|(ext, _)| *ext == key)
        .map(|(_, language)| *language)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("ghr-search-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn search(dir: &Path, pattern: &str, options: &SearchOptions) -> Vec<SearchResult> {
        search_files(dir, pattern, options).unwrap()
    }

    #[test]
    fn search_files_finds_matches_with_file_line_and_text() {
        let dir = temp_dir("basic");
        std::fs::write(dir.join("test.js"), "const hello = \"world\";").unwrap();
        std::fs::write(dir.join("README.md"), "# Hello World").unwrap();

        let results = search(&dir, "hello", &SearchOptions::default());

        assert!(!results.is_empty());
        assert!(
            results
                .iter()
                .all(|r| !r.file.is_empty() && r.line >= 1 && !r.text.is_empty())
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn search_files_reports_all_matches_per_line() {
        let dir = temp_dir("multi");
        std::fs::write(dir.join("a.txt"), "hello and hello again").unwrap();

        let results = search(&dir, "hello", &SearchOptions::default());

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].matches, vec!["hello", "hello"]);
        assert_eq!(results[0].line, 1);

        let _ = std::fs::remove_dir_all(&dir);
    }

    // Case-sensitive on purpose: it mirrors the Go build's suffix match.
    #[expect(clippy::case_sensitive_file_extension_comparisons)]
    #[test]
    fn search_files_filters_by_extension() {
        let dir = temp_dir("ext");
        std::fs::write(dir.join("code.js"), "hello").unwrap();
        std::fs::write(dir.join("note.md"), "hello").unwrap();

        let options = SearchOptions {
            ext: Some(".js".to_owned()),
            ignore_case: false,
        };
        let results = search(&dir, "hello", &options);

        assert_eq!(results.len(), 1);
        assert!(results[0].file.ends_with(".js"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn search_files_supports_case_insensitive_matching() {
        let dir = temp_dir("case");
        std::fs::write(dir.join("a.txt"), "HeLLo").unwrap();

        let options = SearchOptions {
            ext: None,
            ignore_case: true,
        };
        assert!(!search(&dir, "hello", &options).is_empty());
        assert!(search(&dir, "hello", &SearchOptions::default()).is_empty());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn search_files_skips_git_and_node_modules_directories() {
        let dir = temp_dir("skip");
        std::fs::create_dir_all(dir.join(".git")).unwrap();
        std::fs::create_dir_all(dir.join("node_modules")).unwrap();
        std::fs::write(dir.join(".git").join("hello.txt"), "hello").unwrap();
        std::fs::write(dir.join("node_modules").join("hello.js"), "hello").unwrap();
        std::fs::write(dir.join("real.txt"), "hello").unwrap();

        let results = search(&dir, "hello", &SearchOptions::default());
        let files: Vec<&str> = results.iter().map(|r| r.file.as_str()).collect();

        assert_eq!(files, vec!["real.txt"]);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn search_files_skips_binary_files() {
        let dir = temp_dir("binary");
        std::fs::write(dir.join("blob.bin"), b"hello\x00binary").unwrap();
        std::fs::write(dir.join("text.txt"), "hello").unwrap();

        let results = search(&dir, "hello", &SearchOptions::default());
        let files: Vec<&str> = results.iter().map(|r| r.file.as_str()).collect();

        assert_eq!(files, vec!["text.txt"]);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn search_files_returns_error_for_invalid_regex() {
        assert!(search_files(Path::new("/"), "a(b", &SearchOptions::default()).is_err());
    }

    #[test]
    fn detect_language_maps_common_extensions() {
        assert_eq!(detect_language("main.go"), Some("Go"));
        assert_eq!(detect_language("app.js"), Some("JavaScript"));
        assert_eq!(detect_language("index.ts"), Some("TypeScript"));
        assert_eq!(detect_language("App.tsx"), Some("TypeScript React"));
        assert_eq!(detect_language("server.py"), Some("Python"));
        assert_eq!(detect_language("README.md"), Some("Markdown"));
        assert_eq!(detect_language("config.json"), Some("JSON"));
        assert_eq!(detect_language("docker.yaml"), Some("YAML"));
        assert_eq!(detect_language("App.TSX"), Some("TypeScript React"));
    }

    #[test]
    fn detect_language_ignores_dotfiles_and_extensionless_names() {
        assert_eq!(detect_language(".gitignore"), None);
        assert_eq!(detect_language("Makefile"), None);
        assert_eq!(detect_language("README"), None);
    }

    #[test]
    fn results_to_json_keeps_field_order_and_types() {
        let results = vec![SearchResult {
            file: "a.go".to_owned(),
            line: 3,
            text: "hi".to_owned(),
            matches: vec!["hi".to_owned()],
        }];

        assert_eq!(
            results_to_json(&results).marshal_indent(),
            "[\n  {\n    \"file\": \"a.go\",\n    \"line\": 3,\n    \"text\": \"hi\",\n    \"matches\": [\n      \"hi\"\n    ]\n  }\n]"
        );
    }
}
