use crate::config::{self, Config};
use crate::resolve::{resolve_template, TemplateSource};
use anyhow::Result;
use console::style;
use std::path::Path;

pub fn add(cfg_path: &Path, cfg: &mut Config, template: String, alias: String) -> Result<()> {
    let ts = resolve_template(&template, cfg)?;
    let full_name = match ts {
        TemplateSource::GitHub { user, repo } => format!("{}/{}", user, repo),
        TemplateSource::Url(url) => url,
    };
    let existing = cfg.aliases.get(&alias).cloned();

    cfg.aliases.insert(alias.clone(), full_name.clone());
    config::patch_table_entry(cfg_path, "aliases", &alias, &full_name)?;

    if let Some(old) = existing {
        println!(
            "{} Updated alias `{}`: {} -> {}",
            style("✓").green(),
            style(&alias).yellow(),
            old,
            full_name
        );
    } else {
        println!(
            "{} Added alias `{}` -> {}",
            style("✓").green(),
            style(&alias).yellow(),
            full_name
        );
    }
    Ok(())
}

pub fn list(cfg: &Config) -> Result<()> {
    if cfg.aliases.is_empty() {
        println!("No aliases configured.");
        return Ok(());
    }
    println!("{}", style("Configured aliases:").cyan().bold());
    let mut aliases: Vec<_> = cfg.aliases.iter().collect();
    aliases.sort_by_key(|(k, _)| (*k).clone());
    for (name, target) in aliases {
        println!("  {} -> {}", style(name).yellow(), target);
    }
    Ok(())
}

pub fn remove(cfg_path: &Path, cfg: &mut Config, name: String) -> Result<()> {
    if !cfg.aliases.contains_key(&name) {
        println!(
            "{} Alias `{}` not found.",
            style("!").yellow(),
            style(&name).yellow()
        );
        return Ok(());
    }
    cfg.aliases.remove(&name);
    config::remove_table_entry(cfg_path, "aliases", &name)?;
    println!(
        "{} Removed alias `{}`.",
        style("✓").green(),
        style(&name).yellow()
    );
    Ok(())
}
