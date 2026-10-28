use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{Duration, SystemTime};

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn new(name: &str) -> Self {
        let root =
            std::env::temp_dir().join(format!("ghr-regression-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let fixture = Self { root };
        let source = fixture.root.join("source");
        std::fs::create_dir_all(&source).unwrap();
        fixture.git(&source, &["init", "-q", "-b", "main"]);
        std::fs::write(source.join("README.md"), "fixture\n").unwrap();
        fixture.commit("initial");
        let remote = fixture.root.join("remote/owner/repo.git");
        std::fs::create_dir_all(remote.parent().unwrap()).unwrap();
        fixture.git(&source, &["clone", "--bare", ".", remote.to_str().unwrap()]);
        fixture.git(
            &source,
            &["remote", "add", "origin", remote.to_str().unwrap()],
        );
        let base = fixture
            .root
            .join("remote")
            .to_string_lossy()
            .replace('\\', "/");
        let prefix = if base.starts_with('/') {
            "file://"
        } else {
            "file:///"
        };
        std::fs::write(
            fixture.root.join("gitconfig"),
            format!("[url \"{prefix}{base}/\"]\n\tinsteadOf = https://github.com/\n"),
        )
        .unwrap();
        fixture
    }

    fn git(&self, directory: &Path, args: &[&str]) -> String {
        let output = Command::new("git")
            .arg("-C")
            .arg(directory)
            .args(args)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", self.root.join("gitconfig"))
            .env("GIT_CONFIG_COUNT", "0")
            .env("GIT_AUTHOR_NAME", "Test")
            .env("GIT_AUTHOR_EMAIL", "test@example.invalid")
            .env("GIT_COMMITTER_NAME", "Test")
            .env("GIT_COMMITTER_EMAIL", "test@example.invalid")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).trim().to_owned()
    }

    fn commit(&self, message: &str) {
        let source = self.root.join("source");
        self.git(&source, &["add", "."]);
        self.git(&source, &["commit", "-qm", message]);
    }

    fn command(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_ghr"));
        command
            .current_dir(&self.root)
            .env("HOME", &self.root)
            .env("USERPROFILE", &self.root)
            .env("GH_CACHE_DIR", self.root.join("cache"))
            .env("GH_CACHE_MAX_AGE", "")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", self.root.join("gitconfig"))
            .env("GIT_CONFIG_COUNT", "0");
        for name in [
            "GH_PROXY",
            "HTTPS_PROXY",
            "https_proxy",
            "HTTP_PROXY",
            "http_proxy",
            "ALL_PROXY",
            "all_proxy",
        ] {
            command.env(name, "");
        }
        command
    }

    fn run(&self, args: &[&str]) -> Output {
        self.command().args(args).output().unwrap()
    }

    fn succeed(&self, args: &[&str]) -> Output {
        let output = self.run(args);
        assert!(
            output.status.success(),
            "{args:?}: {}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        output
    }

    fn cached_repo(&self) -> PathBuf {
        self.root.join("cache/owner/repo")
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

#[cfg(unix)]
#[test]
fn invalid_utf8_arguments_are_rejected_without_lossy_conversion() {
    use std::os::unix::ffi::OsStringExt;
    let output = Command::new(env!("CARGO_BIN_EXE_ghr"))
        .arg("--version")
        .arg(std::ffi::OsString::from_vec(vec![0xff]))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("valid UTF-8"));
}

#[test]
fn refresh_keeps_history_shallow_and_tracks_default_branch_changes() {
    let fixture = Fixture::new("shallow");
    fixture.succeed(&["readme", "owner/repo"]);
    for revision in 1..=3 {
        std::fs::write(
            fixture.root.join("source/README.md"),
            format!("revision {revision}\n"),
        )
        .unwrap();
        fixture.commit(&format!("revision {revision}"));
        fixture.git(&fixture.root.join("source"), &["push", "origin", "main"]);
        fixture.succeed(&["readme", "owner/repo"]);
        assert_eq!(
            fixture.git(&fixture.cached_repo(), &["rev-list", "--count", "HEAD"]),
            "1"
        );
        assert_eq!(
            std::fs::read_to_string(fixture.cached_repo().join("README.md")).unwrap(),
            format!("revision {revision}\n")
        );
    }
    fixture.git(&fixture.root.join("source"), &["checkout", "-qb", "next"]);
    std::fs::write(fixture.root.join("source/README.md"), "new default\n").unwrap();
    fixture.commit("next default");
    fixture.git(&fixture.root.join("source"), &["push", "origin", "next"]);
    fixture.git(
        &fixture.root.join("remote/owner/repo.git"),
        &["symbolic-ref", "HEAD", "refs/heads/next"],
    );
    let output = fixture.succeed(&["readme", "owner/repo"]);
    assert!(String::from_utf8_lossy(&output.stdout).contains("new default"));
}

#[test]
fn clean_all_preserves_unrelated_data_and_rejects_unrecognized_roots() {
    let fixture = Fixture::new("clean-all");
    fixture.succeed(&["readme", "owner/repo"]);
    let personal = fixture.root.join("cache/owner/documents");
    std::fs::create_dir_all(&personal).unwrap();
    std::fs::write(personal.join("keep.txt"), "keep").unwrap();
    fixture.succeed(&["clean", "--all"]);
    assert!(!fixture.cached_repo().exists());
    assert_eq!(
        std::fs::read_to_string(personal.join("keep.txt")).unwrap(),
        "keep"
    );
    let output = fixture
        .command()
        .env("GH_CACHE_DIR", &fixture.root)
        .args(["clean", "--all"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(fixture.root.join("source/README.md").exists());
}

#[test]
fn clean_old_also_cleans_requested_output_directory() {
    let fixture = Fixture::new("old-output");
    fixture.succeed(&["readme", "owner/repo"]);
    let fetch_head = fixture.cached_repo().join(".git/FETCH_HEAD");
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(fetch_head)
        .unwrap();
    file.set_times(
        std::fs::FileTimes::new().set_modified(SystemTime::now() - Duration::from_secs(172_800)),
    )
    .unwrap();
    drop(file);
    std::fs::create_dir_all(fixture.root.join(".ghr-output")).unwrap();
    std::fs::write(fixture.root.join(".ghr-output/result.json"), "{}").unwrap();
    fixture.succeed(&["clean", "--old", "1d", "-o"]);
    assert!(!fixture.cached_repo().exists());
    assert!(!fixture.root.join(".ghr-output").exists());
}

#[test]
fn invalid_repository_cannot_trigger_automatic_cleanup() {
    let fixture = Fixture::new("invalid");
    fixture.succeed(&["readme", "owner/repo"]);
    let output = fixture
        .command()
        .env("GH_CACHE_MAX_AGE", "0h")
        .args(["readme", "../repo"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(fixture.cached_repo().join("README.md").exists());
}

#[test]
fn refresh_rejects_redirected_git_worktrees() {
    let fixture = Fixture::new("redirected-worktree");
    fixture.succeed(&["readme", "owner/repo"]);
    let outside = fixture.root.join("outside");
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(outside.join("README.md"), "keep outside").unwrap();
    fixture.git(
        &fixture.cached_repo(),
        &["config", "core.worktree", outside.to_str().unwrap()],
    );
    assert!(!fixture.run(&["readme", "owner/repo"]).status.success());
    assert_eq!(
        std::fs::read_to_string(outside.join("README.md")).unwrap(),
        "keep outside"
    );
}

#[test]
fn reading_the_repo_root_does_not_list_sibling_directories() {
    let fixture = Fixture::new("root-error");
    fixture.succeed(&["readme", "owner/repo"]);
    std::fs::create_dir_all(fixture.root.join("cache/owner/sibling-private-name")).unwrap();
    let output = fixture.run(&["read", "owner/repo", "."]);
    assert!(!output.status.success());
    assert!(!String::from_utf8_lossy(&output.stdout).contains("sibling-private-name"));
}

#[test]
fn failed_refresh_preserves_cached_files() {
    let fixture = Fixture::new("offline");
    fixture.succeed(&["readme", "owner/repo"]);
    std::fs::rename(
        fixture.root.join("remote"),
        fixture.root.join("unavailable"),
    )
    .unwrap();
    assert!(!fixture.run(&["readme", "owner/repo"]).status.success());
    assert_eq!(
        std::fs::read_to_string(fixture.cached_repo().join("README.md")).unwrap(),
        "fixture\n"
    );
}

#[test]
fn concurrent_cleanup_waits_for_the_cache_lock() {
    use fs2::FileExt;
    let fixture = Fixture::new("concurrent");
    fixture.succeed(&["readme", "owner/repo"]);
    let lock = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(fixture.root.join("cache/.ghr.lock"))
        .unwrap();
    lock.lock_exclusive().unwrap();
    let mut cleanup = fixture
        .command()
        .args(["clean", "--all"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    std::thread::sleep(Duration::from_millis(200));
    assert!(cleanup.try_wait().unwrap().is_none());
    assert!(fixture.cached_repo().exists());
    FileExt::unlock(&lock).unwrap();
    assert!(cleanup.wait().unwrap().success());
    assert!(!fixture.cached_repo().exists());
}

#[cfg(unix)]
#[test]
fn read_readme_and_ls_do_not_follow_links_outside_the_repository() {
    let fixture = Fixture::new("read-boundary");
    fixture.succeed(&["readme", "owner/repo"]);
    let outside = fixture.root.join("outside");
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(outside.join("private.txt"), "synthetic private fixture").unwrap();
    std::os::unix::fs::symlink(&outside, fixture.cached_repo().join("outside")).unwrap();
    for args in [
        vec!["read", "owner/repo", "outside/private.txt"],
        vec!["ls", "owner/repo", "outside"],
        vec!["read", "owner/repo", "../../../outside/private.txt"],
    ] {
        let output = fixture.run(&args);
        assert!(!output.status.success());
        assert!(!String::from_utf8_lossy(&output.stdout).contains("synthetic private fixture"));
    }
    std::os::unix::fs::symlink(fixture.cached_repo(), outside.join("back")).unwrap();
    let output = fixture.run(&["read", "owner/repo", "outside/back"]);
    assert!(!output.status.success());
    assert!(!String::from_utf8_lossy(&output.stdout).contains("private.txt"));

    fixture.git(&fixture.root.join("source"), &["rm", "README.md"]);
    std::os::unix::fs::symlink(
        outside.join("private.txt"),
        fixture.root.join("source/README.md"),
    )
    .unwrap();
    fixture.commit("external readme");
    fixture.git(&fixture.root.join("source"), &["push", "origin", "main"]);
    let output = fixture.succeed(&["readme", "owner/repo"]);
    assert!(!String::from_utf8_lossy(&output.stdout).contains("synthetic private fixture"));
}
