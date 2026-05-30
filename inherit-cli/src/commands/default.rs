use crate::config::{self, Config};
use crate::prompt::prompt_single_value;
use crate::CM;
use anyhow::{bail, Result};
use console::style;
use kissreplace::valid::is_valid_var_name;
use std::path::Path;

pub fn set(cfg_path: &Path, cfg: &mut Config, name: String) -> Result<()> {
    if !is_valid_var_name(&name) {
        bail!(
            "Invalid variable name `{}`. Must match [a-zA-Z_][a-zA-Z0-9_]*",
            name
        );
    }
    let current = cfg.defaults.get(&name).map(|s| s.as_str());
    let value = prompt_single_value(&name, current)?;

    if value.is_empty() {
        cfg.defaults.remove(&name);
        config::remove_table_entry(cfg_path, "defaults", &name)?;
        println!(
            "{} Cleared default for `{}`.",
            style(CM).green(),
            style(&name).yellow()
        );
    } else {
        cfg.defaults.insert(name.clone(), value.clone());
        config::patch_table_entry(cfg_path, "defaults", &name, &value)?;
        println!(
            "{} Set default for `{}` = `{}`.",
            style(CM).green(),
            style(&name).yellow(),
            style(&value).yellow()
        );
    }
    Ok(())
}

pub fn list(cfg: &Config) -> Result<()> {
    if cfg.defaults.is_empty() {
        println!("No default values configured.");
        return Ok(());
    }
    println!("{}", style("Configured defaults:").cyan().bold());
    let mut defaults: Vec<_> = cfg.defaults.iter().collect();
    defaults.sort_by_key(|(k, _)| (*k).clone());
    for (name, value) in defaults {
        println!("  {} = {}", style(name).yellow(), style(value).green());
    }
    Ok(())
}

pub fn unset(cfg_path: &Path, cfg: &mut Config, name: String) -> Result<()> {
    if !cfg.defaults.contains_key(&name) {
        println!(
            "{} Default for `{}` not found.",
            style("!").yellow(),
            style(&name).yellow()
        );
        return Ok(());
    }
    cfg.defaults.remove(&name);
    config::remove_table_entry(cfg_path, "defaults", &name)?;
    println!(
        "{} Unset default for `{}`.",
        style(CM).green(),
        style(&name).yellow()
    );
    Ok(())
}
