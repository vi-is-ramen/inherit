#![allow(unused)]

use assert_cmd::Command;
use assert_fs::TempDir;
use std::path::{Path, PathBuf};
use std::process::Stdio;

pub struct TestEnv {
    pub tmp: TempDir,
    pub config_path: PathBuf,
    pub cache_dir: PathBuf,
}

impl TestEnv {
    pub fn new() -> Self {
        let tmp = TempDir::new().unwrap();
        let config_path = tmp.path().join("config.toml");
        let cache_dir = tmp.path().join("cache");
        std::fs::create_dir_all(&cache_dir).unwrap();
        Self {
            tmp,
            config_path,
            cache_dir,
        }
    }

    /// Creates a `Command` for the `inherit` binary with the required env vars.
    pub fn cmd(&self) -> Command {
        let mut cmd = Command::cargo_bin("cargo-inherit").unwrap();
        cmd.env("INHERIT_CONFIG", &self.config_path);
        cmd.env("INHERIT_CACHE_DIR", &self.cache_dir);
        cmd.env("INHERIT_NON_INTERACTIVE", "1");
        cmd.env("GIT_AUTHOR_NAME", "Test");
        cmd.env("GIT_AUTHOR_EMAIL", "test@example.com");
        cmd.env("GIT_COMMITTER_NAME", "Test");
        cmd.env("GIT_COMMITTER_EMAIL", "test@example.com");
        cmd
    }

    /// Creates a minimal config (no defaults, no aliases).
    pub fn write_empty_config(&self) {
        std::fs::write(&self.config_path, "").unwrap();
    }

    /// Creates a config with the given contents.
    pub fn write_config(&self, content: &str) {
        std::fs::write(&self.config_path, content).unwrap();
    }
}

/// Creates a local git repository with the template and returns the path to it (for file:// URL).
pub fn create_local_template<F>(base: &Path, name: &str, setup: F) -> PathBuf
where
    F: FnOnce(&Path),
{
    let repo_dir = base.join(name);
    std::fs::create_dir_all(&repo_dir).unwrap();

    // Initializing a regular repository
    run_git(&repo_dir, &["init", "-q", "-b", "main"]);
    run_git(&repo_dir, &["config", "user.email", "test@example.com"]);
    run_git(&repo_dir, &["config", "user.name", "Test"]);

    setup(&repo_dir);

    run_git(&repo_dir, &["add", "."]);
    run_git(&repo_dir, &["commit", "-q", "-m", "init"]);

    repo_dir
}

fn run_git(cwd: &Path, args: &[&str]) {
    let status = std::process::Command::new("git")
        .current_dir(cwd)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap();
    assert!(status.success(), "git {:?} failed", args);
}
