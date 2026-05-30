mod common;

use common::TestEnv;
use predicates::prelude::*;

#[test]
fn test_add_alias() {
    let env = TestEnv::new();
    env.write_empty_config();

    env.cmd()
        .args(["user/repo", "to", "alias", "myalias"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added alias `myalias`"));

    let content = std::fs::read_to_string(&env.config_path).unwrap();
    assert!(content.contains("[aliases]"));
    assert!(content.contains("myalias = \"user/repo\""));
}

#[test]
fn test_update_alias() {
    let env = TestEnv::new();
    env.write_config(
        r#"
[aliases]
myalias = "old/repo"
"#,
    );

    env.cmd()
        .args(["new/repo", "to", "alias", "myalias"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Updated alias `myalias`"));

    let content = std::fs::read_to_string(&env.config_path).unwrap();
    assert!(content.contains("myalias = \"new/repo\""));
    assert!(!content.contains("old/repo"));
}

#[test]
fn test_list_aliases() {
    let env = TestEnv::new();
    env.write_config(
        r#"
[aliases]
alpha = "user/alpha"
beta = "user/beta"
"#,
    );

    env.cmd()
        .args(["alias", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alpha"))
        .stdout(predicate::str::contains("user/alpha"))
        .stdout(predicate::str::contains("beta"));
}

#[test]
fn test_list_empty_aliases() {
    let env = TestEnv::new();
    env.write_empty_config();

    env.cmd()
        .args(["alias", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No aliases"));
}

#[test]
fn test_remove_alias() {
    let env = TestEnv::new();
    env.write_config(
        r#"
[aliases]
myalias = "user/repo"
"#,
    );

    env.cmd()
        .args(["alias", "remove", "myalias"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed alias"));

    let content = std::fs::read_to_string(&env.config_path).unwrap();
    assert!(!content.contains("myalias"));
}

#[test]
fn test_remove_nonexistent_alias() {
    let env = TestEnv::new();
    env.write_empty_config();

    env.cmd()
        .args(["alias", "remove", "ghost"])
        .assert()
        .success()
        .stdout(predicate::str::contains("not found"));
}

#[test]
fn test_use_alias_for_generation() {
    let env = TestEnv::new();
    let tpl = common::create_local_template(env.tmp.path(), "tpl", |repo| {
        std::fs::write(repo.join("Inherit.toml"), "[template]\nname = \"x\"\n").unwrap();
        std::fs::write(repo.join("marker.txt"), "ok").unwrap();
    });

    let tpl_path = tpl.to_string_lossy().replace('\\', "/");
    
    env.write_config(&format!(
        r#"
[aliases]
mytpl = "{}"
"#,
        tpl_path
    ));

    let target = env.tmp.path().join("output");
    env.cmd()
        .args(["mytpl", "to", target.to_str().unwrap()])
        .assert()
        .success();

    assert!(target.join("marker.txt").exists());
}
