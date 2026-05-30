use anyhow::{anyhow, Context, Result};
use console::style;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub defaults: HashMap<String, String>,

    #[serde(default)]
    pub aliases: HashMap<String, String>,

    pub cache_dir: Option<PathBuf>,

    pub github_token: Option<String>,

    #[serde(default = "default_true")]
    pub init_git: bool,

    #[serde(default = "default_true")]
    pub run_hooks: bool,

    pub open_with: Option<String>,
}

fn default_true() -> bool {
    true
}

pub const DEFAULT_CONFIG: &str = r#"# Inherit configuration file
# See: https://github.com/yourname/inherit

# Default values for template variables.
# When a template requires a variable listed here, its value is used as the
# default suggestion — you can still override it via the interactive prompt.
[defaults]
AUTHOR = "Your Name <you@example.com>"
# VERSION = "0.1.0"
# LICENSE = "MIT"

# Short aliases for templates.
# Example: `inherit rust-lib` will resolve to `username/rust-lib-template`.
[aliases]
# rust-lib = "username/rust-lib-template"

# Directory used to cache downloaded templates.
# cache_dir = "~/.cache/inherit"

# GitHub personal access token (for private repositories).
# github_token = "ghp_..."

# Whether to automatically run `git init` in generated projects.
# init_git = true

# Whether to execute `post_create` hooks defined by templates.
# run_hooks = true

# Command to open the project after generation (e.g., "code", "nvim", "idea").
# open_with = "code"
"#;

pub fn load_or_create(path: &Path) -> Result<Config> {
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, DEFAULT_CONFIG)?;
        println!(
            "{} Created default config at {}",
            style(crate::CM).green(),
            path.display()
        );
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read config {}", path.display()))?;
    let cfg: Config = toml::from_str(&content)
        .with_context(|| format!("Failed to parse config {}", path.display()))?;
    Ok(cfg)
}

/// Surgically adds/updates one entry to `[defaults]` or `[aliases]`,
/// without touching the rest of the file (and comments!).
pub fn patch_table_entry(path: &Path, table: &str, key: &str, value: &str) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

    let table_header = format!("[{}]", table);
    let entry_line = format!("{} = \"{}\"", key, value.escape_default());

    // Searching section
    let header_idx = lines.iter().position(|l| l.trim() == table_header);

    match header_idx {
        Some(idx) => {
            // We are looking to see if such a key already exists in this section.
            let mut existing_idx = None;
            let mut insert_at = idx + 1;
            let mut i = idx + 1;
            while i < lines.len() {
                let line = lines[i].trim();
                if line.starts_with('[') {
                    // The next section has begun - stop
                    insert_at = i;
                    break;
                }
                if let Some(rest) = line.strip_prefix(&format!("{} =", key)) {
                    let rest = rest.trim();
                    if rest.starts_with('"') || rest.starts_with('\'') || !rest.is_empty() {
                        existing_idx = Some(i);
                    }
                }
                i += 1;
                if i == lines.len() {
                    insert_at = i;
                }
            }

            if let Some(ei) = existing_idx {
                lines[ei] = entry_line;
            } else {
                lines.insert(insert_at, entry_line);
            }
        }
        None => {
            // There is no section - add to the end
            if !lines.is_empty() && !lines.last().unwrap().is_empty() {
                lines.push(String::new());
            }
            lines.push(table_header);
            lines.push(entry_line);
            lines.push(String::new());
        }
    }

    let mut out = lines.join("\n");
    if !out.ends_with('\n') {
        out.push('\n');
    }
    fs::write(path, out)?;
    Ok(())
}

pub fn config_path() -> Result<PathBuf> {
    if let Ok(p) = std::env::var("INHERIT_CONFIG") {
        return Ok(PathBuf::from(p));
    }
    let config_dir =
        dirs::config_dir().ok_or_else(|| anyhow!("Cannot determine config directory"))?;
    Ok(config_dir.join("inherit").join("config.toml"))
}

pub fn cache_dir(cfg: &Config) -> Result<PathBuf> {
    if let Ok(p) = std::env::var("INHERIT_CACHE_DIR") {
        return Ok(PathBuf::from(p));
    }

    if let Some(ref p) = cfg.cache_dir {
        let s = p.to_string_lossy();
        if let Some(rest) = s.strip_prefix("~/") {
            let home = dirs::home_dir().ok_or_else(|| anyhow!("No home directory"))?;
            return Ok(home.join(rest));
        }
        return Ok(p.clone());
    }
    let base = dirs::cache_dir().ok_or_else(|| anyhow!("Cannot determine cache directory"))?;
    Ok(base.join("inherit"))
}

/// Surgically removes an entry from the specified TOML section without affecting the rest of the file.
pub fn remove_table_entry(path: &Path, table: &str, key: &str) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    let table_header = format!("[{}]", table);

    if let Some(idx) = lines.iter().position(|l| l.trim() == table_header) {
        let mut i = idx + 1;
        while i < lines.len() {
            let line = lines[i].trim();
            if line.starts_with('[') {
                break; // The next section has begun
            }
            // We check whether the string starts with the search key
            if let Some(k) = line.split('=').next() {
                if k.trim() == key {
                    lines.remove(i);
                    let mut out = lines.join("\n");
                    if !out.ends_with('\n') {
                        out.push('\n');
                    }
                    return fs::write(path, out).context("Failed to update configuration");
                }
            }
            i += 1;
        }
    }
    Ok(()) // Key not found - do nothing
}
