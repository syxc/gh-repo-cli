//! Cache inventory and retention: scan cached repositories, determine when
//! each was last used, parse age specifications like `30d`, and format ages
//! for display.

use std::fs::{File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};

use fs2::FileExt;

const ROOT_MARKER: &str = ".ghr-cache-root";
const ROOT_SIGNATURE: &str = "ghr cache v1\n";
const LOCK_NAME: &str = ".ghr.lock";

/// The lock file stays in place so waiting processes always lock the same inode.
pub struct CacheLock {
    file: File,
}

impl Drop for CacheLock {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self.file);
    }
}

impl CacheLock {
    pub fn acquire(cache_dir: &Path) -> io::Result<Self> {
        validate_root(cache_dir)?;
        std::fs::create_dir_all(cache_dir)?;
        let lock_path = cache_dir.join(LOCK_NAME);
        if std::fs::symlink_metadata(&lock_path).is_ok_and(|meta| !meta.is_file()) {
            return Err(io::Error::other("cache lock is not a regular file"));
        }
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(lock_path)?;
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            match file.try_lock_exclusive() {
                Ok(()) => break,
                Err(err) if err.raw_os_error() == fs2::lock_contended_error().raw_os_error() => {
                    if Instant::now() >= deadline {
                        return Err(io::Error::new(
                            io::ErrorKind::TimedOut,
                            "cache is in use by another ghr process",
                        ));
                    }
                    std::thread::sleep(Duration::from_millis(50));
                }
                Err(err) => return Err(err),
            }
        }
        validate_root(cache_dir)?;
        let marker = cache_dir.join(ROOT_MARKER);
        if !marker.exists() {
            std::fs::write(marker, ROOT_SIGNATURE)?;
        }

        Ok(Self { file })
    }
}

fn validate_root(cache_dir: &Path) -> io::Result<()> {
    if !cache_dir.try_exists()? {
        return Ok(());
    }
    let marker = cache_dir.join(ROOT_MARKER);
    if let Ok(meta) = std::fs::symlink_metadata(&marker) {
        if meta.is_file() && std::fs::read_to_string(&marker)? == ROOT_SIGNATURE {
            return Ok(());
        }
        return Err(io::Error::other("invalid ghr cache marker"));
    }

    let contents = std::fs::read_dir(cache_dir)?.collect::<io::Result<Vec<_>>>()?;
    let unmarked: Vec<_> = contents
        .iter()
        .filter(|entry| entry.file_name() != LOCK_NAME && !is_finder_metadata(entry))
        .collect();
    if unmarked.is_empty() {
        return Ok(());
    }
    // A pre-marker cache must contain only recognized repository directories.
    let legacy = unmarked.iter().all(|owner| {
        owner.file_type().is_ok_and(|kind| kind.is_dir())
            && std::fs::read_dir(owner.path()).is_ok_and(|repos| {
                repos.into_iter().all(|repo| {
                    repo.is_ok_and(|repo| is_finder_metadata(&repo) || is_cached_repo(&repo.path()))
                })
            })
    });
    if legacy {
        return Ok(());
    }

    Err(io::Error::other(
        "cache directory contains unrecognized data; set GH_CACHE_DIR to an empty directory",
    ))
}

fn is_finder_metadata(entry: &std::fs::DirEntry) -> bool {
    entry.file_name() == ".DS_Store" && entry.file_type().is_ok_and(|kind| kind.is_file())
}

pub fn is_cached_repo(path: &Path) -> bool {
    if !is_directory(path) || !is_directory(&path.join(".git")) {
        return false;
    }
    let Some(owner) = path
        .parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
    else {
        return false;
    };
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let repo = format!("{owner}/{name}");
    if crate::git::parse_repo(&repo).is_err() {
        return false;
    }
    let config = path.join(".git").join("config");
    if !std::fs::symlink_metadata(&config).is_ok_and(|meta| meta.is_file()) {
        return false;
    }
    crate::git::origin_url(path)
        .is_some_and(|url| url.eq_ignore_ascii_case(&crate::git::build_clone_url(owner, name)))
        && crate::git::has_local_worktree(path)
}

pub fn is_directory(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|meta| meta.is_dir())
}

/// Validates both repository path components without following symlinks.
pub fn repo_path(cache_dir: &Path, repo: &str) -> Result<PathBuf, String> {
    let (owner, name) = crate::git::parse_repo(repo).map_err(|err| err.to_string())?;
    let owner_path = cache_dir.join(owner);
    let path = owner_path.join(name);
    for directory in [&owner_path, &path] {
        match std::fs::symlink_metadata(directory) {
            Ok(meta) if !meta.is_dir() => {
                return Err(format!(
                    "refusing cache path that is not a directory: {}",
                    directory.display()
                ));
            }
            Err(err) if err.kind() != io::ErrorKind::NotFound => return Err(err.to_string()),
            _ => {}
        }
    }
    if path.exists() && !is_cached_repo(&path) {
        return Err(format!(
            "refusing unrecognized cached repository: {}",
            path.display()
        ));
    }

    Ok(path)
}

#[derive(Debug)]
pub struct CacheEntry {
    /// `owner/repo`, relative to the cache root.
    pub name: String,
    pub path: PathBuf,
    pub last_used: SystemTime,
}

/// Parses a duration like `24h`, `7d` or `2w` (unit case-insensitive).
pub fn parse_duration(raw: &str) -> Result<Duration, String> {
    let invalid = || format!("invalid duration \"{raw}\" (expected e.g. 24h, 7d, 2w)");

    let (digits, unit) = match raw.char_indices().next_back() {
        Some((index, _)) => raw.split_at(index),
        None => return Err(invalid()),
    };
    let multiplier: u64 = match unit.to_lowercase().as_str() {
        "h" => 3_600,
        "d" => 86_400,
        "w" => 604_800,
        _ => return Err(invalid()),
    };
    let value: u64 = digits.parse().map_err(|_| invalid())?;
    value
        .checked_mul(multiplier)
        .map(Duration::from_secs)
        .ok_or_else(invalid)
}

/// Renders an age for display: `45m`, `3h` or `12d`.
pub fn format_age(age: Duration) -> String {
    let minutes = age.as_secs() / 60;
    if minutes < 60 {
        return format!("{minutes}m");
    }
    let hours = minutes / 60;
    if hours < 24 {
        return format!("{hours}h");
    }
    format!("{}d", hours / 24)
}

impl CacheEntry {
    /// Age since last use; zero on clock skew.
    pub fn age(&self) -> Duration {
        SystemTime::now()
            .duration_since(self.last_used)
            .unwrap_or_default()
    }
}

/// ghr's update path runs `git fetch` on every use, which rewrites
/// `.git/FETCH_HEAD`; its mtime is the last-use marker. Directory mtime is
/// the fallback for entries without one.
fn last_used(repo_path: &Path) -> io::Result<SystemTime> {
    std::fs::metadata(repo_path.join(".git").join("FETCH_HEAD"))
        .or_else(|_| std::fs::metadata(repo_path))
        .and_then(|meta| meta.modified())
}

/// Lists cached repositories as `owner/repo`, oldest first.
pub fn scan(cache_dir: &Path) -> Vec<CacheEntry> {
    let Ok(owners) = std::fs::read_dir(cache_dir) else {
        return Vec::new();
    };

    let mut entries = Vec::new();
    for owner in owners.flatten() {
        if !owner.file_type().is_ok_and(|kind| kind.is_dir()) {
            continue;
        }
        let Ok(repos) = std::fs::read_dir(owner.path()) else {
            continue;
        };
        for repo in repos.flatten() {
            let path = repo.path();
            if !is_cached_repo(&path) {
                continue;
            }
            let Ok(last_used) = last_used(&path) else {
                continue;
            };
            entries.push(CacheEntry {
                name: format!(
                    "{}/{}",
                    owner.file_name().to_string_lossy(),
                    repo.file_name().to_string_lossy()
                ),
                path,
                last_used,
            });
        }
    }

    entries.sort_by_key(|entry| entry.last_used);
    entries
}

/// Entries whose last use is older than `max_age`, excluding one repository
/// (the one about to be used).
pub fn stale_entries(
    cache_dir: &Path,
    max_age: Duration,
    exclude: Option<&str>,
) -> Vec<CacheEntry> {
    scan(cache_dir)
        .into_iter()
        .filter(|entry| Some(entry.name.as_str()) != exclude)
        .filter(|entry| entry.age() > max_age)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry_with_mtime(repo_path: &Path, age: Duration) -> CacheEntry {
        let modified = SystemTime::now()
            .checked_sub(age)
            .expect("age fits before now");
        let fetch_head = repo_path.join(".git").join("FETCH_HEAD");
        let file = std::fs::OpenOptions::new()
            .append(true)
            .open(&fetch_head)
            .unwrap();
        file.set_times(std::fs::FileTimes::new().set_modified(modified))
            .unwrap();

        CacheEntry {
            name: repo_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            path: repo_path.to_path_buf(),
            last_used: last_used(repo_path).unwrap(),
        }
    }

    fn temp_cache(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("ghr-cache-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("owner")).unwrap();
        dir
    }

    fn init_repo(path: &Path) {
        assert!(
            std::process::Command::new("git")
                .args(["init", "-q"])
                .arg(path)
                .status()
                .unwrap()
                .success()
        );
        let owner = path
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let name = path.file_name().unwrap().to_str().unwrap();
        assert!(
            std::process::Command::new("git")
                .arg("-C")
                .arg(path)
                .args([
                    "config",
                    "remote.origin.url",
                    &crate::git::build_clone_url(owner, name)
                ])
                .status()
                .unwrap()
                .success()
        );
    }

    #[test]
    fn legacy_cache_accepts_finder_metadata_at_root_and_owner_levels() {
        let dir = temp_cache("finder-metadata");
        let repo = dir.join("owner").join("repo");
        init_repo(&repo);
        for parent in [&dir, &dir.join("owner")] {
            std::fs::write(parent.join(".DS_Store"), "Finder metadata").unwrap();
        }

        validate_root(&dir).unwrap();
        for parent in [&dir, &dir.join("owner")] {
            assert_eq!(
                std::fs::read(parent.join(".DS_Store")).unwrap(),
                b"Finder metadata"
            );
        }
        assert_eq!(scan(&dir).len(), 1);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn legacy_cache_accepts_github_owner_and_repo_case_variations() {
        let dir = temp_cache("github-case");
        let repo = dir.join("owner").join("repo");
        init_repo(&repo);
        assert!(
            std::process::Command::new("git")
                .arg("-C")
                .arg(&repo)
                .args([
                    "config",
                    "remote.origin.url",
                    "https://github.com/Owner/Repo.git"
                ])
                .status()
                .unwrap()
                .success()
        );

        assert!(is_cached_repo(&repo));
        validate_root(&dir).unwrap();
        assert_eq!(repo_path(&dir, "owner/repo").unwrap(), repo);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn legacy_cache_still_rejects_unknown_files_beside_finder_metadata() {
        let dir = temp_cache("finder-unknown");
        let repo = dir.join("owner").join("repo");
        init_repo(&repo);
        std::fs::write(dir.join(".DS_Store"), "Finder metadata").unwrap();
        validate_root(&dir).unwrap();
        for parent in [&dir, &dir.join("owner")] {
            let unknown = parent.join("personal.txt");
            std::fs::write(&unknown, "keep").unwrap();
            assert!(validate_root(&dir).is_err());
            assert_eq!(std::fs::read(&unknown).unwrap(), b"keep");
            std::fs::remove_file(unknown).unwrap();
        }
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn legacy_cache_still_rejects_other_remotes() {
        let dir = temp_cache("other-remotes");
        let repo = dir.join("owner").join("repo");
        init_repo(&repo);
        for remote in [
            "https://github.com/Owner/Other.git",
            "https://example.invalid/Owner/Repo.git",
        ] {
            assert!(
                std::process::Command::new("git")
                    .arg("-C")
                    .arg(&repo)
                    .args(["config", "remote.origin.url", remote])
                    .status()
                    .unwrap()
                    .success()
            );
            assert!(!is_cached_repo(&repo));
            assert!(validate_root(&dir).is_err());
        }
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn cache_root_accepts_only_regular_finder_metadata() {
        let dir = temp_cache("finder-only");
        std::fs::write(dir.join(".DS_Store"), "Finder metadata").unwrap();
        validate_root(&dir).unwrap();
        std::fs::remove_file(dir.join(".DS_Store")).unwrap();
        std::fs::create_dir_all(dir.join(".DS_Store").join("unknown")).unwrap();

        assert!(validate_root(&dir).is_err());
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn legacy_cache_rejects_finder_metadata_symlinks() {
        let dir = temp_cache("finder-symlink");
        let repo = dir.join("owner").join("repo");
        init_repo(&repo);
        let target = repo.join(".git").join("config");
        let original = std::fs::read(&target).unwrap();
        validate_root(&dir).unwrap();
        std::os::unix::fs::symlink(&target, dir.join(".DS_Store")).unwrap();

        assert!(validate_root(&dir).is_err());
        std::fs::remove_file(dir.join(".DS_Store")).unwrap();
        std::os::unix::fs::symlink(&target, dir.join("owner").join(".DS_Store")).unwrap();
        assert!(validate_root(&dir).is_err());
        assert_eq!(std::fs::read(target).unwrap(), original);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn cache_root_rejects_existing_unrelated_data() {
        let dir = temp_cache("unsafe-root");
        std::fs::write(dir.join("personal.txt"), "keep").unwrap();
        assert!(CacheLock::acquire(&dir).is_err());
        assert_eq!(
            std::fs::read_to_string(dir.join("personal.txt")).unwrap(),
            "keep"
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn cache_lock_is_exclusive_and_released_on_drop() {
        let dir = temp_cache("locking");
        let first = CacheLock::acquire(&dir).unwrap();
        let other = OpenOptions::new()
            .read(true)
            .write(true)
            .open(dir.join(LOCK_NAME))
            .unwrap();
        assert_eq!(
            other.try_lock_exclusive().unwrap_err().raw_os_error(),
            fs2::lock_contended_error().raw_os_error()
        );
        drop(first);
        other.try_lock_exclusive().unwrap();
        drop(other);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn parse_duration_accepts_hours_days_and_weeks() {
        assert_eq!(parse_duration("24h"), Ok(Duration::from_secs(86_400)));
        assert_eq!(parse_duration("7d"), Ok(Duration::from_secs(604_800)));
        assert_eq!(parse_duration("2w"), Ok(Duration::from_secs(1_209_600)));
        assert_eq!(parse_duration("30D"), Ok(Duration::from_secs(2_592_000)));
    }

    #[test]
    fn parse_duration_rejects_malformed_input() {
        for raw in ["", "30", "d30", "30x", "-1d", "1.5d"] {
            assert!(parse_duration(raw).is_err(), "{raw} should not parse");
        }
    }

    #[test]
    fn parse_duration_rejects_overflow() {
        assert!(parse_duration("99999999999999999999w").is_err());
    }

    #[test]
    fn format_age_uses_minutes_hours_and_days() {
        assert_eq!(format_age(Duration::from_secs(30)), "0m");
        assert_eq!(format_age(Duration::from_secs(45 * 60)), "45m");
        assert_eq!(format_age(Duration::from_secs(3 * 3600)), "3h");
        assert_eq!(format_age(Duration::from_secs(45 * 86400)), "45d");
    }

    #[test]
    fn scan_lists_owner_repo_pairs_sorted_oldest_first() {
        let dir = temp_cache("scan");
        let old_repo = dir.join("owner").join("old");
        let new_repo = dir.join("owner").join("new");
        init_repo(&old_repo);
        init_repo(&new_repo);
        std::fs::write(old_repo.join(".git").join("FETCH_HEAD"), "x").unwrap();
        std::fs::write(new_repo.join(".git").join("FETCH_HEAD"), "x").unwrap();
        entry_with_mtime(&old_repo, Duration::from_secs(10 * 86400));
        entry_with_mtime(&new_repo, Duration::from_secs(3600));

        let entries = scan(&dir);

        assert_eq!(
            entries
                .iter()
                .map(|entry| entry.name.as_str())
                .collect::<Vec<_>>(),
            vec!["owner/old", "owner/new"]
        );
        assert!(entries[0].age() > entries[1].age());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn stale_entries_filters_by_age_and_exclusion() {
        let dir = temp_cache("stale");
        let old_repo = dir.join("owner").join("old");
        let fresh_repo = dir.join("owner").join("fresh");
        init_repo(&old_repo);
        init_repo(&fresh_repo);
        std::fs::write(old_repo.join(".git").join("FETCH_HEAD"), "x").unwrap();
        std::fs::write(fresh_repo.join(".git").join("FETCH_HEAD"), "x").unwrap();
        entry_with_mtime(&old_repo, Duration::from_secs(40 * 86400));
        entry_with_mtime(&fresh_repo, Duration::from_secs(3600));

        let stale = stale_entries(&dir, Duration::from_secs(30 * 86400), None);
        assert_eq!(stale.len(), 1);
        assert_eq!(stale[0].name, "owner/old");

        let excluded = stale_entries(&dir, Duration::from_secs(30 * 86400), Some("owner/old"));
        assert!(excluded.is_empty());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn scan_preserves_unrelated_directories() {
        let dir = temp_cache("unrelated");
        std::fs::create_dir_all(dir.join("owner").join("documents")).unwrap();

        assert!(scan(&dir).is_empty());

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn scan_does_not_follow_owner_or_repository_symlinks() {
        let dir = temp_cache("symlinks");
        let outside = temp_cache("outside");
        let repo = outside.join("owner").join("repo");
        std::fs::create_dir_all(repo.join(".git")).unwrap();
        std::fs::write(repo.join(".git").join("FETCH_HEAD"), "x").unwrap();
        std::os::unix::fs::symlink(outside.join("owner"), dir.join("alias")).unwrap();
        std::os::unix::fs::symlink(&repo, dir.join("owner").join("alias")).unwrap();

        assert!(scan(&dir).is_empty());

        std::fs::remove_dir_all(&dir).unwrap();
        std::fs::remove_dir_all(&outside).unwrap();
    }

    #[test]
    fn scan_returns_empty_for_missing_cache_dir() {
        assert!(scan(Path::new("/definitely/not/a/cache")).is_empty());
    }
}
