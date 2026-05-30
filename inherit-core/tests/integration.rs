use inherit_core::{load_template, process_template, ProcessOptions, Variables};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_cargo_lib_template() {
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("cargo-lib");

    let target = tempdir().unwrap();
    let target_path = target.path().join("output");

    // Let's emulate what the CLI did: collect all the variables (from user defaults and prompts)
    let mut final_vars = Variables::new();
    final_vars.insert("PROJECT_NAME".into(), "awesome_lib".into());
    final_vars.insert("AUTHOR".into(), "Alice <alice@example.com>".into());
    final_vars.insert("VERSION".into(), "0.1.0".into());
    final_vars.insert("DESCRIPTION".into(), "My awesome lib".into());

    let opts = ProcessOptions {
        init_git: false,
        run_hooks: false,
    };

    let result = process_template(&source, &target_path, &final_vars, opts).unwrap();
    assert!(result.processed_files > 0);

    let cargo_toml = fs::read_to_string(target_path.join("Cargo.toml")).unwrap();
    assert!(cargo_toml.contains("name = \"awesome_lib\""));
    assert!(!cargo_toml.contains("@PROJECT_NAME@"));
}

#[test]
fn test_missing_variable_error() {
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("cargo-lib");

    let target = tempdir().unwrap();
    let target_path = target.path().join("output");

    // Not all variables were passed (forgotten DESCRIPTION)
    let mut final_vars = Variables::new();
    final_vars.insert("PROJECT_NAME".into(), "awesome_lib".into());
    final_vars.insert("AUTHOR".into(), "Alice".into());
    final_vars.insert("VERSION".into(), "0.1.0".into());
    // DESCRIPTION absent!

    let opts = ProcessOptions {
        init_git: false,
        run_hooks: false,
    };

    let err = process_template(&source, &target_path, &final_vars, opts).unwrap_err();
    let err_msg = err.to_string();
    assert!(
        err_msg.contains("DESCRIPTION"),
        "Unexpected error: {}",
        err_msg
    );
}

#[test]
fn test_load_template_context() {
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("cargo-lib");

    let ctx = load_template(&source).unwrap();

    // We check that required_vars contains both what is in the manifest and what is in the files
    assert!(ctx.required_vars.contains("PROJECT_NAME"));
    assert!(ctx.required_vars.contains("AUTHOR"));

    // We check the descriptions
    assert_eq!(
        ctx.var_descriptions.get("PROJECT_NAME").unwrap(),
        "Name of the project"
    );
}

#[test]
fn test_variable_in_filename() {
    let src_dir = tempdir().unwrap();
    let src = src_dir.path();

    fs::write(
        src.join("Inherit.toml"),
        "[variables]\nNAME = \"User name\"\n",
    )
    .unwrap();
    fs::write(src.join(".inherignore"), "").unwrap();
    fs::write(src.join("hello-@NAME@.txt"), "Hi @NAME@").unwrap();

    let target = tempdir().unwrap();
    let target_path = target.path().join("out");

    let mut final_vars = Variables::new();
    final_vars.insert("NAME".into(), "world".into());

    let opts = ProcessOptions {
        init_git: false,
        run_hooks: false,
    };
    process_template(src, &target_path, &final_vars, opts).unwrap();

    let expected = target_path.join("hello-world.txt");
    assert!(expected.exists());
    let content = fs::read_to_string(expected).unwrap();
    assert_eq!(content, "Hi world");
}
