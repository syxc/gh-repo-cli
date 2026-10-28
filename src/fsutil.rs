//! Filesystem helpers shared across commands: directory trees, file reads,
//! output persistence, size formatting and Go-style IO error messages.

use std::collections::HashMap;
use std::io;
use std::path::Path;

use crate::json::Json;
use crate::search::detect_language;

#[derive(Debug, Eq, PartialEq)]
pub enum Kind {
    File,
    Directory,
}

impl Kind {
    fn json_name(&self) -> &'static str {
        match self {
            Kind::File => "file",
            Kind::Directory => "directory",
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FileItem {
    pub name: String,
    pub path: String,
    pub kind: Kind,
    /// `None` mirrors Go's nil slice (JSON `null`, used for files); `Some`
    /// holds directory children, possibly empty (JSON `[]`).
    pub children: Option<Vec<FileItem>>,
}

pub fn items_to_json(items: &[FileItem]) -> Json {
    Json::Array(items.iter().map(file_item_to_json).collect())
}

fn file_item_to_json(item: &FileItem) -> Json {
    let children = match &item.children {
        Some(children) => Json::Array(children.iter().map(file_item_to_json).collect()),
        None => Json::Null,
    };

    Json::Object(vec![
        ("name", Json::Str(item.name.clone())),
        ("path", Json::Str(item.path.clone())),
        ("type", Json::Str(item.kind.json_name().to_owned())),
        ("children", children),
    ])
}

/// Walks `dir` breadth-limited by `max_depth`, skipping `.git` and
/// `node_modules`, with entries sorted by name like Go's `os.ReadDir`.
pub fn traverse_dir(dir: &Path, max_depth: i64, current_depth: i64) -> io::Result<Vec<FileItem>> {
    if current_depth >= max_depth {
        return Ok(Vec::new());
    }

    let mut entries: Vec<std::fs::DirEntry> = std::fs::read_dir(dir)?.collect::<Result<_, _>>()?;
    entries.sort_by_key(std::fs::DirEntry::file_name);

    let mut items = Vec::with_capacity(entries.len());
    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == ".git" || name == "node_modules" {
            continue;
        }

        let path = entry.path();
        let (kind, children) = if entry.file_type().is_ok_and(|file_type| file_type.is_dir()) {
            // Like the Go build: a failed sub-walk yields nil children (`null`).
            let children = traverse_dir(&path, max_depth, current_depth + 1).ok();
            (Kind::Directory, children)
        } else {
            (Kind::File, None)
        };

        items.push(FileItem {
            name,
            path: path.to_string_lossy().into_owned(),
            kind,
            children,
        });
    }

    Ok(items)
}

/// Reads a file as text; invalid UTF-8 is replaced like Go's JSON encoding.
pub fn read_file_content(path: &Path) -> io::Result<String> {
    std::fs::read(path).map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
}

/// Files-per-extension and files-per-language counts for a directory.
#[derive(Debug, Default)]
pub struct FileStats {
    pub file_types: HashMap<String, i64>,
    pub languages: HashMap<String, i64>,
}

/// Counts files per extension and language, `max_depth` levels deep.
///
/// This walks the tree independently of any display tree, so statistics stay
/// stable when the structure depth changes. Errors below the root are
/// skipped, mirroring the traversal's tolerance for unreadable subdirectories.
pub fn count_file_stats(dir: &Path, max_depth: i64) -> io::Result<FileStats> {
    let mut stats = FileStats::default();
    count_dir(dir, max_depth, &mut stats)?;

    Ok(stats)
}

fn count_dir(dir: &Path, remaining_depth: i64, stats: &mut FileStats) -> io::Result<()> {
    if remaining_depth <= 0 {
        return Ok(());
    }

    for entry in std::fs::read_dir(dir)?.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == ".git" || name == "node_modules" {
            continue;
        }

        if entry.file_type().is_ok_and(|file_type| file_type.is_dir()) {
            let _ = count_dir(&entry.path(), remaining_depth - 1, stats);
            continue;
        }

        // Matches `filepath.Ext`: everything from the last dot, including
        // dotfiles where the whole name is the extension.
        let extension = match name.rfind('.') {
            Some(dot) => name[dot..].to_lowercase(),
            None => "(no extension)".to_owned(),
        };
        *stats.file_types.entry(extension).or_insert(0) += 1;

        if let Some(language) = detect_language(&name) {
            *stats.languages.entry(language.to_owned()).or_insert(0) += 1;
        }
    }

    Ok(())
}

/// Writes a JSON payload, creating parent directories as needed.
pub fn save_output(json: &Json, output_file: &str) -> Result<(), String> {
    let path = Path::new(output_file);
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent).map_err(|err| {
            format!(
                "failed to create output directory: {}",
                describe_io(&err, "mkdir", parent)
            )
        })?;
    }

    std::fs::write(path, json.marshal_indent()).map_err(|err| {
        format!(
            "failed to write output file: {}",
            describe_io(&err, "open", path)
        )
    })?;

    Ok(())
}

/// Formats bytes like the Go build: "0 B", "512 B", "1.5 KB", "1.0 MB", ...
// The Go build also widens to f64 for display; precision loss above 2^52
// bytes is irrelevant for human-readable sizes.
#[expect(clippy::cast_precision_loss)]
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

    if bytes == 0 {
        return "0 B".to_owned();
    }

    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{bytes} {}", UNITS[unit])
    } else {
        format!("{size:.1} {}", UNITS[unit])
    }
}

/// Sums the sizes of all files under `dir`, skipping unreadable entries.
pub fn directory_size(dir: &Path) -> u64 {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };

    entries
        .flatten()
        .map(|entry| match entry.metadata() {
            Ok(meta) if meta.is_dir() => directory_size(&entry.path()),
            Ok(meta) => meta.len(),
            Err(_) => 0,
        })
        .sum()
}

/// Formats an IO error the way Go's `os` package does ("open /path: no such
/// file or directory"), keeping CLI error output identical to the Go build.
/// Common kinds are mapped directly so the wording is correct on every
/// platform; unmapped cases fall back to the raw errno, then the error text.
pub fn describe_io(err: &io::Error, op: &str, path: &Path) -> String {
    let detail = match err.kind() {
        io::ErrorKind::NotFound => "no such file or directory".to_owned(),
        io::ErrorKind::PermissionDenied => "permission denied".to_owned(),
        io::ErrorKind::IsADirectory => "is a directory".to_owned(),
        io::ErrorKind::NotADirectory => "not a directory".to_owned(),
        io::ErrorKind::DirectoryNotEmpty => "directory not empty".to_owned(),
        io::ErrorKind::AlreadyExists => "file exists".to_owned(),
        _ => match err.raw_os_error() {
            Some(code) => errno_message(code).to_owned(),
            None => err.to_string(),
        },
    };

    format!("{op} {}: {}", path.display(), detail)
}

/// Error text for a failed `read_file_content`: Go's `os.ReadFile` reports
/// "open ..." for open failures but "read ..." when reading a directory.
pub fn read_error(err: &io::Error, path: &Path) -> String {
    let op = if err.kind() == io::ErrorKind::IsADirectory {
        "read"
    } else {
        "open"
    };
    describe_io(err, op, path)
}

/// Error text for a failed `traverse_dir`: Go's `os.ReadDir` reports
/// "open ..." when the path is missing but "readdirent ..." for non-directories.
pub fn traverse_error(err: &io::Error, path: &Path) -> String {
    let op = if err.kind() == io::ErrorKind::NotADirectory {
        "readdirent"
    } else {
        "open"
    };
    describe_io(err, op, path)
}

#[cfg(unix)]
fn errno_message(code: i32) -> &'static str {
    match code {
        1 => "operation not permitted",
        2 => "no such file or directory",
        13 => "permission denied",
        17 => "file exists",
        20 => "not a directory",
        21 => "is a directory",
        22 => "invalid argument",
        28 => "no space left on device",
        39 => "directory not empty",
        _ => "unknown error",
    }
}

#[cfg(not(unix))]
fn errno_message(_code: i32) -> &'static str {
    "unknown error"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("ghr-fsutil-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn traverse_dir_sorts_entries_and_skips_git_and_node_modules() {
        let dir = temp_dir("traverse");
        std::fs::create_dir_all(dir.join("src")).unwrap();
        std::fs::create_dir_all(dir.join(".git")).unwrap();
        std::fs::create_dir_all(dir.join("node_modules")).unwrap();
        std::fs::write(dir.join("b.txt"), "b").unwrap();
        std::fs::write(dir.join("a.txt"), "a").unwrap();
        std::fs::write(dir.join("src").join("inner.js"), "x").unwrap();

        let items = traverse_dir(&dir, 3, 0).unwrap();
        let names: Vec<&str> = items.iter().map(|item| item.name.as_str()).collect();

        assert_eq!(names, vec!["a.txt", "b.txt", "src"]);
        let src = items.iter().find(|item| item.name == "src").unwrap();
        assert_eq!(src.kind, Kind::Directory);
        let children = src.children.as_ref().unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].name, "inner.js");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn traverse_dir_gives_directories_empty_children_and_files_none_at_limit() {
        let dir = temp_dir("depth");
        std::fs::create_dir_all(dir.join("sub")).unwrap();
        std::fs::write(dir.join("sub").join("inner.txt"), "x").unwrap();
        std::fs::write(dir.join("top.txt"), "x").unwrap();

        let items = traverse_dir(&dir, 1, 0).unwrap();
        let sub = items.iter().find(|item| item.name == "sub").unwrap();
        let top = items.iter().find(|item| item.name == "top.txt").unwrap();

        assert_eq!(sub.children, Some(Vec::new()));
        assert_eq!(top.children, None);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn traverse_dir_respects_max_depth() {
        let dir = temp_dir("maxdepth");
        std::fs::create_dir_all(dir.join("level1").join("level2")).unwrap();
        std::fs::write(dir.join("level1").join("level2").join("deep.txt"), "x").unwrap();

        let items = traverse_dir(&dir, 1, 0).unwrap();
        let level1 = items.iter().find(|item| item.name == "level1").unwrap();
        assert_eq!(level1.children, Some(Vec::new()));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn traverse_dir_errors_for_missing_directory() {
        let missing = std::env::temp_dir().join("ghr-fsutil-missing-dir");
        let _ = std::fs::remove_dir_all(&missing);

        assert!(traverse_dir(&missing, 3, 0).is_err());
    }

    #[test]
    fn read_file_content_returns_text_and_errors_for_missing_files() {
        let dir = temp_dir("read");
        std::fs::write(dir.join("hello.txt"), "Hello, World!").unwrap();
        std::fs::write(dir.join("empty.txt"), "").unwrap();

        assert_eq!(
            read_file_content(&dir.join("hello.txt")).unwrap(),
            "Hello, World!"
        );
        assert_eq!(read_file_content(&dir.join("empty.txt")).unwrap(), "");
        assert!(read_file_content(&dir.join("missing.txt")).is_err());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn count_file_stats_buckets_extensions_and_languages() {
        let dir = temp_dir("stats");
        std::fs::write(dir.join("main.go"), "package main").unwrap();
        std::fs::write(dir.join("README.md"), "# hi").unwrap();
        std::fs::write(dir.join("Makefile"), "all:").unwrap();
        std::fs::create_dir_all(dir.join("src")).unwrap();
        std::fs::write(dir.join("src").join("app.TS"), "x").unwrap();
        std::fs::create_dir_all(dir.join(".git")).unwrap();
        std::fs::write(dir.join(".git").join("ignored.go"), "x").unwrap();

        let stats = count_file_stats(&dir, 2).unwrap();

        assert_eq!(stats.file_types.get(".go"), Some(&1));
        assert_eq!(stats.file_types.get(".md"), Some(&1));
        assert_eq!(stats.file_types.get(".ts"), Some(&1));
        assert_eq!(stats.file_types.get("(no extension)"), Some(&1));
        assert_eq!(stats.languages.get("Go"), Some(&1));
        assert_eq!(stats.languages.get("Markdown"), Some(&1));
        assert_eq!(stats.languages.get("TypeScript"), Some(&1));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn count_file_stats_respects_max_depth() {
        let dir = temp_dir("stats-depth");
        std::fs::create_dir_all(dir.join("one").join("two")).unwrap();
        std::fs::write(dir.join("root.txt"), "x").unwrap();
        std::fs::write(dir.join("one").join("mid.txt"), "x").unwrap();
        std::fs::write(dir.join("one").join("two").join("deep.txt"), "x").unwrap();

        let shallow = count_file_stats(&dir, 1).unwrap();
        let deep = count_file_stats(&dir, 3).unwrap();

        assert_eq!(shallow.file_types.get(".txt"), Some(&1));
        assert_eq!(deep.file_types.get(".txt"), Some(&3));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn save_output_writes_json_and_creates_nested_directories() {
        let dir = temp_dir("save");
        let target = dir.join("nested").join("out.json");

        save_output(&Json::Str("text".to_owned()), target.to_str().unwrap()).unwrap();

        let written = std::fs::read_to_string(&target).unwrap();
        assert_eq!(written, "\"text\"");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn format_bytes_matches_go_output() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.0 GB");
    }

    #[test]
    fn directory_size_sums_file_sizes() {
        let dir = temp_dir("size");
        std::fs::write(dir.join("a.txt"), "12345").unwrap();
        std::fs::create_dir_all(dir.join("sub")).unwrap();
        std::fs::write(dir.join("sub").join("b.txt"), "123").unwrap();

        assert_eq!(directory_size(&dir), 8);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn describe_io_maps_directory_not_empty_via_error_kind() {
        let dir = temp_dir("ioerr");
        std::fs::create_dir_all(dir.join("sub")).unwrap();
        std::fs::write(dir.join("sub").join("f.txt"), "x").unwrap();

        let err = std::fs::remove_dir(dir.join("sub")).unwrap_err();
        let message = describe_io(&err, "remove", &dir.join("sub"));

        assert!(
            message.ends_with(": directory not empty"),
            "unexpected message: {message}"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn describe_io_formats_missing_files_like_go() {
        let err = std::fs::read("/definitely/not/here").unwrap_err();
        let message = describe_io(&err, "open", Path::new("/definitely/not/here"));

        assert_eq!(
            message,
            "open /definitely/not/here: no such file or directory"
        );
    }
}
