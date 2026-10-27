//! Cache inventory and retention: scan cached repositories, determine when
//! each was last used, parse age specifications like `30d`, and format ages
//! for display.

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

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
fn last_used(repo_path: &Path) -> SystemTime {
    std::fs::metadata(repo_path.join(".git").join("FETCH_HEAD"))
        .or_else(|_| std::fs::metadata(repo_path))
        .and_then(|meta| meta.modified())
        .unwrap_or(SystemTime::UNIX_EPOCH)
}

/// Lists cached repositories as `owner/repo`, oldest first.
pub fn scan(cache_dir: &Path) -> Vec<CacheEntry> {
    let Ok(owners) = std::fs::read_dir(cache_dir) else {
        return Vec::new();
    };

    let mut entries = Vec::new();
    for owner in owners.flatten() {
        let Ok(repos) = std::fs::read_dir(owner.path()) else {
            continue;
        };
        for repo in repos.flatten() {
            let path = repo.path();
            if !path.is_dir() {
                continue;
            }
            entries.push(CacheEntry {
                name: format!(
                    "{}/{}",
                    owner.file_name().to_string_lossy(),
                    repo.file_name().to_string_lossy()
                ),
                path: path.clone(),
                last_used: last_used(&path),
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
            last_used: last_used(repo_path),
        }
    }

    fn temp_cache(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("ghr-cache-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("owner")).unwrap();
        dir
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
        std::fs::create_dir_all(old_repo.join(".git")).unwrap();
        std::fs::create_dir_all(new_repo.join(".git")).unwrap();
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
        std::fs::create_dir_all(old_repo.join(".git")).unwrap();
        std::fs::create_dir_all(fresh_repo.join(".git")).unwrap();
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
    fn scan_returns_empty_for_missing_cache_dir() {
        assert!(scan(Path::new("/definitely/not/a/cache")).is_empty());
    }
}
