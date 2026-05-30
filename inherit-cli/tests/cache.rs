mod common;

use common::TestEnv;
use predicates::prelude::*;

#[test]
fn test_cache_list_empty() {
    let env = TestEnv::new();
    env.write_empty_config();

    env.cmd()
        .args(["cache", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("empty").or(predicate::str::contains("does not exist")));
}

#[test]
fn test_cache_list_with_entries() {
    let env = TestEnv::new();
    env.write_empty_config();

    // Create fake cached directories
    std::fs::create_dir_all(env.cache_dir.join("abc123def456")).unwrap();
    std::fs::create_dir_all(env.cache_dir.join("xyz789")).unwrap();
    std::fs::write(env.cache_dir.join("abc123def456/file.txt"), "hello world").unwrap();

    env.cmd()
        .args(["cache", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("abc123def456"))
        .stdout(predicate::str::contains("xyz789"));
}

#[test]
fn test_cache_clean() {
    let env = TestEnv::new();
    env.write_empty_config();

    std::fs::create_dir_all(env.cache_dir.join("abc123")).unwrap();
    std::fs::write(env.cache_dir.join("abc123/file.txt"), "data").unwrap();

    env.cmd()
        .args(["cache", "clean"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cache cleaned"));

    // Check that the directories have been deleted
    let entries: Vec<_> = std::fs::read_dir(&env.cache_dir).unwrap().collect();
    assert!(entries.is_empty());
}

#[test]
fn test_cache_clean_empty() {
    let env = TestEnv::new();
    env.write_empty_config();

    env.cmd()
        .args(["cache", "clean"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("already empty")
                .or(predicate::str::contains("does not exist")),
        );
}

#[test]
fn test_cache_persists_after_generation() {
    let env = TestEnv::new();
    let tpl = common::create_local_template(env.tmp.path(), "tpl", |repo| {
        std::fs::write(repo.join("Inherit.toml"), "[template]\nname = \"x\"\n").unwrap();
        std::fs::write(repo.join("a.txt"), "hi").unwrap();
    });

    env.write_empty_config();
    let target = env.tmp.path().join("output");

    env.cmd()
        .arg(tpl.to_string_lossy().to_string())
        .arg("to")
        .arg(&target)
        .assert()
        .success();

    // There should now be one entry in the cache.
    env.cmd()
        .args(["cache", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cached templates"));

    let entries: Vec<_> = std::fs::read_dir(&env.cache_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();
    assert_eq!(entries.len(), 1);
}
