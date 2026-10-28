//! Git operations: repo parsing, cached shallow clones, and repo metadata.
//! All git invocations shell out to the `git` binary, like the Go build.

use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

use crate::json::Json;

const CLONE_TIMEOUT: Duration = Duration::from_secs(120);
const UPDATE_TIMEOUT: Duration = Duration::from_secs(30);
const POLL_INTERVAL: Duration = Duration::from_millis(50);

#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error("invalid repo format: {0}. Expected: owner/repo")]
    InvalidRepoFormat(String),
    #[error("failed to create cache directory: {0}")]
    CreateCacheDir(String),
    #[error("failed to clone {0}: {1}")]
    CloneFailed(String, String),
    #[error("{0}")]
    Cache(String),
}

pub struct RepoInfo {
    pub remote: String,
    pub commit: String,
    pub date: String,
}

impl RepoInfo {
    pub fn to_json(&self) -> Json {
        Json::Object(vec![
            ("remote", Json::Str(self.remote.clone())),
            ("commit", Json::Str(self.commit.clone())),
            ("date", Json::Str(self.date.clone())),
        ])
    }
}

pub fn parse_repo(repo: &str) -> Result<(&str, &str), GitError> {
    match repo.split('/').collect::<Vec<_>>().as_slice() {
        [owner, name] if is_segment(owner) && is_segment(name) => Ok((owner, name)),
        _ => Err(GitError::InvalidRepoFormat(repo.to_owned())),
    }
}

/// A path segment that cannot escape the cache directory: dot segments like
/// `..` would otherwise resolve `clean`/`clone` targets outside (or onto the
/// root of) the cache tree.
fn is_segment(segment: &str) -> bool {
    if segment.is_empty()
        || segment == "."
        || segment == ".."
        || segment.ends_with('.')
        || !segment
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"-_.".contains(&byte))
    {
        return false;
    }
    let stem = segment
        .split('.')
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    !matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        && !(stem.len() == 4
            && (stem.starts_with("COM") || stem.starts_with("LPT"))
            && matches!(stem.as_bytes()[3], b'1'..=b'9'))
}

pub fn build_clone_url(owner: &str, name: &str) -> String {
    format!("https://github.com/{owner}/{name}.git")
}

/// Clones or refreshes `repo` while the caller holds the cache lock.
/// Failed refreshes preserve the existing cache; `no_cache` forces a clone.
pub fn clone_repo(
    repo: &str,
    cache_dir: &Path,
    proxy: &str,
    no_cache: bool,
) -> Result<PathBuf, GitError> {
    let (owner, name) = parse_repo(repo)?;
    let repo_path = crate::cache::repo_path(cache_dir, repo).map_err(GitError::Cache)?;

    if repo_path.exists() {
        if no_cache {
            std::fs::remove_dir_all(&repo_path).map_err(|err| {
                GitError::Cache(crate::fsutil::describe_io(&err, "remove", &repo_path))
            })?;
        } else if fetch_latest(&repo_path, proxy) {
            return Ok(repo_path);
        } else {
            return Err(GitError::Cache(format!(
                "failed to refresh {repo}; existing cache was preserved"
            )));
        }
    }

    let parent = repo_path.parent().unwrap_or(Path::new(""));
    std::fs::create_dir_all(parent).map_err(|err| {
        GitError::CreateCacheDir(crate::fsutil::describe_io(&err, "mkdir", parent))
    })?;

    let mut command = git_command();
    command
        .args(proxy_args(proxy))
        // --single-branch is implied by --depth 1; spelled out so the cache is
        // guaranteed to hold only the default branch tip.
        .args(["clone", "--depth", "1", "--single-branch"])
        .arg(build_clone_url(owner, name))
        .arg(&repo_path);

    println!("Cloning {repo}...");
    match run_with_deadline(command, Instant::now() + CLONE_TIMEOUT, StreamMode::Inherit) {
        Outcome::Succeeded => Ok(repo_path),
        Outcome::Failed(status) => Err(GitError::CloneFailed(
            repo.to_owned(),
            status_message(status),
        )),
        Outcome::TimedOut => Err(GitError::CloneFailed(
            repo.to_owned(),
            "context deadline exceeded".to_owned(),
        )),
        Outcome::SpawnError(err) => {
            Err(GitError::CloneFailed(repo.to_owned(), spawn_message(&err)))
        }
    }
}

/// Refreshes the remote's current default branch at depth one within 30s.
/// Git's automatic maintenance retains its normal object expiry policy.
fn fetch_latest(repo_path: &Path, proxy: &str) -> bool {
    let deadline = Instant::now() + UPDATE_TIMEOUT;

    let mut fetch = git_command();
    fetch
        .args(proxy_args(proxy))
        .arg("-C")
        .arg(repo_path)
        .args(["fetch", "--depth", "1", "--no-tags", "origin", "HEAD"]);

    let mut reset = git_command();
    reset
        .args(proxy_args(proxy))
        .arg("-C")
        .arg(repo_path)
        .args(["reset", "--hard", "FETCH_HEAD"]);

    if !(is_success(&run_with_deadline(fetch, deadline, StreamMode::Discard))
        && is_success(&run_with_deadline(reset, deadline, StreamMode::Discard)))
    {
        return false;
    }

    let mut gc = git_command();
    gc.args(["-c", "gc.autoDetach=false", "-C"])
        .arg(repo_path)
        .args(["gc", "--auto", "--quiet"]);
    let _ = run_with_deadline(gc, deadline, StreamMode::Discard);

    true
}

/// Reads repository metadata; `None` when the path is not a usable git repo.
pub fn get_repo_info(repo_path: &Path) -> Option<RepoInfo> {
    let remote = origin_url(repo_path)?;
    let commit = git_output(repo_path, &["rev-parse", "HEAD"])?;
    let date = git_output(repo_path, &["log", "-1", "--format=%ci"])?;

    Some(RepoInfo {
        remote,
        commit,
        date,
    })
}

pub fn origin_url(repo_path: &Path) -> Option<String> {
    git_output(
        repo_path,
        &["config", "--local", "--get", "remote.origin.url"],
    )
}

pub fn has_local_worktree(repo_path: &Path) -> bool {
    let Ok(root) = repo_path.canonicalize() else {
        return false;
    };
    let Some(worktree) = git_output(repo_path, &["rev-parse", "--show-toplevel"]) else {
        return false;
    };
    let Some(git_dir) = git_output(repo_path, &["rev-parse", "--absolute-git-dir"]) else {
        return false;
    };
    let Some(common_dir) = git_output(repo_path, &["rev-parse", "--git-common-dir"]) else {
        return false;
    };
    Path::new(&worktree)
        .canonicalize()
        .is_ok_and(|path| path == root)
        && Path::new(&git_dir)
            .canonicalize()
            .is_ok_and(|path| path == root.join(".git"))
        && repo_path
            .join(common_dir)
            .canonicalize()
            .is_ok_and(|path| path == root.join(".git"))
}

fn git_command() -> Command {
    let mut command = Command::new("git");
    for variable in [
        "GIT_DIR",
        "GIT_WORK_TREE",
        "GIT_COMMON_DIR",
        "GIT_INDEX_FILE",
        "GIT_OBJECT_DIRECTORY",
        "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    ] {
        command.env_remove(variable);
    }
    command.args(["-c", "maintenance.auto=false"]);
    command
}

fn git_output(repo_path: &Path, args: &[&str]) -> Option<String> {
    let output = git_command()
        .arg("-C")
        .arg(repo_path)
        .args(args)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn proxy_args(proxy: &str) -> Vec<String> {
    if proxy.is_empty() {
        return Vec::new();
    }

    vec![
        "-c".to_owned(),
        format!("http.proxy={proxy}"),
        "-c".to_owned(),
        format!("https.proxy={proxy}"),
    ]
}

#[derive(Debug, Clone, Copy)]
enum StreamMode {
    Inherit,
    Discard,
}

enum Outcome {
    Succeeded,
    Failed(ExitStatus),
    TimedOut,
    SpawnError(io::Error),
}

fn run_with_deadline(mut command: Command, deadline: Instant, streams: StreamMode) -> Outcome {
    if Instant::now() >= deadline {
        return Outcome::TimedOut;
    }
    command.env("GIT_TERMINAL_PROMPT", "0");
    match streams {
        StreamMode::Inherit => {
            command
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());
        }
        StreamMode::Discard => {
            command
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null());
        }
    }

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(err) => return Outcome::SpawnError(err),
    };

    loop {
        match child.try_wait() {
            Ok(Some(status)) if status.success() => return Outcome::Succeeded,
            Ok(Some(status)) => return Outcome::Failed(status),
            Ok(None) if Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait();
                return Outcome::TimedOut;
            }
            Ok(None) => std::thread::sleep(POLL_INTERVAL),
            Err(err) => {
                let _ = child.kill();
                let _ = child.wait();
                return Outcome::SpawnError(err);
            }
        }
    }
}

fn is_success(outcome: &Outcome) -> bool {
    matches!(outcome, Outcome::Succeeded)
}

/// Mirrors Go's `*exec.ExitError` strings for the messages users see.
fn status_message(status: ExitStatus) -> String {
    match status.code() {
        Some(code) => format!("exit status {code}"),
        // The only signal we send ourselves is SIGKILL from the timeout.
        None => "signal: killed".to_owned(),
    }
}

fn spawn_message(err: &io::Error) -> String {
    if err.kind() == io::ErrorKind::NotFound {
        return "exec: \"git\": executable file not found in $PATH".to_owned();
    }

    err.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_repo_accepts_owner_and_name() {
        assert_eq!(parse_repo("facebook/react").unwrap(), ("facebook", "react"));
        assert_eq!(parse_repo("vercel/next.js").unwrap(), ("vercel", "next.js"));
        assert_eq!(
            parse_repo("some-org/project-name").unwrap(),
            ("some-org", "project-name")
        );
    }

    #[test]
    fn parse_repo_rejects_dot_segments_that_escape_the_cache() {
        for repo in ["a/..", "../repo", "./repo", "a/.", "..", "."] {
            assert!(parse_repo(repo).is_err(), "{repo} should not parse");
        }
    }

    #[test]
    fn parse_repo_rejects_malformed_input() {
        for repo in ["react", "facebook/", "", "/repo", "a/b/c"] {
            let err = parse_repo(repo).unwrap_err();
            assert_eq!(
                err.to_string(),
                format!("invalid repo format: {repo}. Expected: owner/repo")
            );
        }
    }

    #[test]
    fn parse_repo_rejects_windows_paths_and_url_delimiters() {
        for repo in [
            r"owner/..\outside",
            "owner/C:outside",
            "owner/repo?ref=main",
            "owner/repo#fragment",
            "owner/repo ",
            "owner/CON",
            "owner/nul.txt",
        ] {
            assert!(parse_repo(repo).is_err(), "{repo} should not parse");
        }
    }

    #[test]
    fn build_clone_url_targets_https_github() {
        assert_eq!(
            build_clone_url("facebook", "react"),
            "https://github.com/facebook/react.git"
        );
    }

    #[test]
    fn get_repo_info_returns_none_for_non_git_directories() {
        let dir = std::env::temp_dir().join(format!("ghr-git-nongit-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);

        assert!(get_repo_info(&dir).is_none());
        assert!(get_repo_info(Path::new("/non/existent/path")).is_none());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn proxy_args_are_empty_without_proxy() {
        assert_eq!(proxy_args(""), Vec::<String>::new());
    }

    #[test]
    fn proxy_args_prepend_http_and_https_config() {
        assert_eq!(
            proxy_args("http://127.0.0.1:7890"),
            vec![
                "-c",
                "http.proxy=http://127.0.0.1:7890",
                "-c",
                "https.proxy=http://127.0.0.1:7890"
            ]
        );
    }
}
