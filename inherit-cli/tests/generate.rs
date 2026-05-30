mod common;

use common::{create_local_template, TestEnv};
use predicates::prelude::*;

fn setup_template(base: &std::path::Path) -> std::path::PathBuf {
    create_local_template(base, "tpl", |repo| {
        std::fs::write(
            repo.join("Inherit.toml"),
            r#"
[template]
name = "test-tpl"

[variables]
PROJECT_NAME = "Project name"
AUTHOR = "Author"

[hooks]
post_create = []
"#,
        )
        .unwrap();
        std::fs::write(repo.join("hello.txt"), "Hello @AUTHOR@!").unwrap();
        std::fs::write(repo.join("project-@PROJECT_NAME@.txt"), "Project file").unwrap();
    })
}

#[test]
fn test_generate_basic() {
    let env = TestEnv::new();
    let tpl = setup_template(env.tmp.path());

    env.write_config(r#"[defaults]\nPROJECT_NAME = "myproj"\nAUTHOR = "Alice"\n"#);

    let target = env.tmp.path().join("output");

    env.cmd()
        .arg(tpl.to_string_lossy().to_string())
        .arg("to")
        .arg(&target)
        .assert()
        .success()
        .stdout(predicate::str::contains("Done!"));

    // Checking for substitutions in the content
    let hello = std::fs::read_to_string(target.join("hello.txt")).unwrap();
    assert_eq!(hello, "Hello Alice!");

    // Checking file name substitutions
    assert!(target.join("project-myproj.txt").exists());
    assert!(!target.join("project-@PROJECT_NAME@.txt").exists());
}

#[test]
fn test_generate_initializes_git() {
    let env = TestEnv::new();
    let tpl = setup_template(env.tmp.path());

    env.write_config(
        r#"
[defaults]
PROJECT_NAME = "p"
AUTHOR = "A"
"#,
    );

    let target = env.tmp.path().join("output");
    env.cmd()
        .arg(tpl.to_string_lossy().to_string())
        .arg("to")
        .arg(&target)
        .assert()
        .success();

    // A fresh .git must be created
    assert!(target.join(".git").exists());
    assert!(target.join(".git/HEAD").exists());
}

#[test]
fn test_generate_missing_variable_error() {
    let env = TestEnv::new();
    let tpl = setup_template(env.tmp.path());

    // AUTHOR is missing from defaults
    env.write_config(
        r#"
[defaults]
PROJECT_NAME = "p"
"#,
    );

    let target = env.tmp.path().join("output");
    env.cmd()
        .arg(tpl.to_string_lossy().to_string())
        .arg("to")
        .arg(&target)
        .assert()
        .failure()
        .stderr(predicate::str::contains("AUTHOR"));
}

#[test]
fn test_generate_runs_post_create_hook() {
    let env = TestEnv::new();
    let tpl = create_local_template(env.tmp.path(), "hook-tpl", |repo| {
        std::fs::write(
            repo.join("Inherit.toml"),
            r#"
[template]
name = "hook-tpl"

[hooks]
post_create = ["touch hook-marker.txt"]
"#,
        )
        .unwrap();
    });

    env.write_empty_config();
    let target = env.tmp.path().join("output");

    env.cmd()
        .arg(tpl.to_string_lossy().to_string())
        .arg("to")
        .arg(&target)
        .assert()
        .success();

    assert!(target.join("hook-marker.txt").exists());
}

#[test]
fn test_generate_respects_inherignore() {
    let env = TestEnv::new();
    let tpl = create_local_template(env.tmp.path(), "ignore-tpl", |repo| {
        std::fs::write(repo.join("Inherit.toml"), "[template]\nname = \"x\"\n").unwrap();
        std::fs::write(repo.join(".inherignore"), "ignored.txt\n").unwrap();
        std::fs::write(repo.join("ignored.txt"), "@SHOULD_NOT_EXIST@").unwrap();
        std::fs::write(repo.join("kept.txt"), "hello").unwrap();
    });

    env.write_empty_config();
    let target = env.tmp.path().join("output");

    env.cmd()
        .arg(tpl.to_string_lossy().to_string())
        .arg("to")
        .arg(&target)
        .assert()
        .success();

    // ignored.txt should be absent from the result
    assert!(!target.join("ignored.txt").exists());
    assert!(target.join("kept.txt").exists());
}
