use crate::error::{InheritError, Result};
use crate::ignore::{InheritIgnore, ALWAYS_IGNORE};
use crate::manifest::Manifest;
use crate::scanner;
use ignore::Walk;
use kissreplace::{KissReplace, Variables};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The context of the loaded template. Used by the CLI to understand which variables to request.
#[derive(Debug)]
pub struct TemplateContext {
    pub manifest: Manifest,
    pub required_vars: HashSet<String>,
    /// Mapping: var_name -> desc (from manifest)
    pub var_descriptions: HashMap<String, String>,
}

/// Loads the template, parses the manifest, and scans the files to collect all required variables.
pub fn load_template(source_dir: &Path) -> Result<TemplateContext> {
    let manifest = Manifest::load(source_dir)?;
    let ignore = InheritIgnore::load(source_dir);
    let scanned_vars = scanner::collect_variables(source_dir, &ignore)?;

    let mut required_vars = HashSet::new();
    let mut var_descriptions = HashMap::new();

    // Adding variables from the manifest
    for (k, v) in &manifest.variables {
        required_vars.insert(k.clone());
        var_descriptions.insert(k.clone(), v.clone());
    }

    // Add variables found in files (even if they are not in the manifest)
    for k in scanned_vars {
        required_vars.insert(k);
    }

    Ok(TemplateContext {
        manifest,
        required_vars,
        var_descriptions,
    })
}

#[derive(Debug, Default)]
pub struct ProcessResult {
    pub processed_files: usize,
    pub binary_files: usize,
}

#[derive(Debug, Clone)]
pub struct ProcessOptions {
    pub init_git: bool,
    pub run_hooks: bool,
}

impl Default for ProcessOptions {
    fn default() -> Self {
        Self {
            init_git: true,
            run_hooks: true,
        }
    }
}

/// Applies replacements and generates the final project.
/// `final_vars` must contain ALL required variables (CLI is responsible for collection and prompts).
pub fn process_template(
    source_dir: &Path,
    target_dir: &Path,
    final_vars: &Variables,
    opts: ProcessOptions,
) -> Result<ProcessResult> {
    let ctx = load_template(source_dir)?;

    // Validating the names of the passed variables
    for key in final_vars.keys() {
        if !kissreplace::valid::is_valid_var_name(key) {
            return Err(InheritError::InvalidVariable(key.clone()));
        }
    }

    // We check that all required variables are filled (not empty)
    let missing: Vec<String> = ctx
        .required_vars
        .iter()
        .filter(|v| final_vars.get(*v).map(|s| s.is_empty()).unwrap_or(true))
        .cloned()
        .collect();

    if !missing.is_empty() {
        return Err(InheritError::MissingVariables(missing));
    }

    if target_dir.exists() {
        return Err(InheritError::Io(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("Target directory already exists: {}", target_dir.display()),
        )));
    }
    fs::create_dir_all(target_dir)?;

    let ignore = InheritIgnore::load(source_dir);
    let mut result = ProcessResult::default();

    for entry in Walk::new(source_dir) {
        let entry = entry.map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
        let path = entry.path();
        let rel = match path.strip_prefix(source_dir) {
            Ok(r) if r.as_os_str().is_empty() => continue,
            Ok(r) => r,
            Err(_) => continue,
        };

        let rel_str = rel.to_string_lossy();
        if ALWAYS_IGNORE
            .iter()
            .any(|&x| rel_str == x || rel_str.starts_with(&format!("{x}/")))
        {
            continue;
        }

        let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        if ignore.is_ignored(rel, is_dir) {
            continue;
        }

        let new_rel_str = final_vars.replace_str(&rel_str);
        let new_rel = PathBuf::from(new_rel_str);
        let new_abs = target_dir.join(&new_rel);

        if is_dir {
            fs::create_dir_all(&new_abs)?;
            continue;
        }

        if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            continue;
        }

        if let Some(parent) = new_abs.parent() {
            fs::create_dir_all(parent)?;
        }

        match fs::read_to_string(path) {
            Ok(text) => {
                let replaced = final_vars.replace_str(&text);
                fs::write(&new_abs, replaced)?;
                result.processed_files += 1;
            }
            Err(_) => {
                fs::copy(path, &new_abs)?;
                result.binary_files += 1;
            }
        }
    }

    if opts.init_git {
        let git_dir = target_dir.join(".git");
        if git_dir.exists() {
            fs::remove_dir_all(&git_dir)?;
        }
        let status = Command::new("git")
            .arg("init")
            .arg("-q")
            .current_dir(target_dir)
            .status()?;
        if !status.success() {
            return Err(InheritError::CommandFailed {
                cmd: "git init".into(),
                status,
            });
        }
    }

    if opts.run_hooks {
        for cmd in &ctx.manifest.hooks.post_create {
            let status = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(["/C", cmd])
                    .current_dir(target_dir)
                    .status()
            } else {
                Command::new("sh")
                    .args(["-c", cmd])
                    .current_dir(target_dir)
                    .status()
            }?;
            if !status.success() {
                return Err(InheritError::CommandFailed {
                    cmd: cmd.clone(),
                    status,
                });
            }
        }
    }

    Ok(result)
}
