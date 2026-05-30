mod common;

use common::TestEnv;
use predicates::prelude::*;

#[test]
fn test_list_empty_defaults() {
    let env = TestEnv::new();
    env.write_empty_config();

    env.cmd()
        .args(["default", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No default values"));
}

#[test]
fn test_list_defaults() {
    let env = TestEnv::new();
    env.write_config(
        r#"
[defaults]
AUTHOR = "Alice"
VERSION = "1.0"
"#,
    );

    env.cmd()
        .args(["default", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("AUTHOR"))
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("VERSION"));
}

#[test]
fn test_unset_default() {
    let env = TestEnv::new();
    env.write_config(
        r#"
[defaults]
AUTHOR = "Alice"
VERSION = "1.0"
"#,
    );

    env.cmd()
        .args(["default", "unset", "AUTHOR"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Unset"));

    let content = std::fs::read_to_string(&env.config_path).unwrap();
    assert!(!content.contains("AUTHOR"));
    assert!(content.contains("VERSION")); // The rest is untouched
}

#[test]
fn test_unset_nonexistent_default() {
    let env = TestEnv::new();
    env.write_empty_config();

    env.cmd()
        .args(["default", "unset", "GHOST"])
        .assert()
        .success()
        .stdout(predicate::str::contains("not found"));
}
