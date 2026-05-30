use crate::config::{self, Config};
use anyhow::Result;
use console::style;
use std::fs;
use std::path::Path;

pub fn list(cfg: &Config) -> Result<()> {
    let cache_dir = config::cache_dir(cfg)?;
    if !cache_dir.exists() {
        println!("Cache directory does not exist yet.");
        return Ok(());
    }

    let entries: Vec<_> = fs::read_dir(&cache_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .collect();

    if entries.is_empty() {
        println!("Cache is empty.");
        return Ok(());
    }

    println!("{}", style("Cached templates:").cyan().bold());
    for entry in entries {
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        let size = dir_size(&path).unwrap_or(0);
        println!("  {} ({})", style(name).yellow(), format_size(size));
    }
    Ok(())
}

pub fn clean(cfg: &Config) -> Result<()> {
    let cache_dir = config::cache_dir(cfg)?;
    if !cache_dir.exists() {
        println!("Cache directory does not exist. Nothing to clean.");
        return Ok(());
    }

    let entries: Vec<_> = fs::read_dir(&cache_dir)?.filter_map(|e| e.ok()).collect();
    if entries.is_empty() {
        println!("Cache is already empty.");
        return Ok(());
    }

    println!(
        "{} Clearing cache at {}...",
        style("->").blue(),
        cache_dir.display()
    );
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }
    println!("{} Cache cleaned.", style("✓").green());
    Ok(())
}

fn dir_size(path: &Path) -> std::io::Result<u64> {
    let mut size = 0;
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let p = entry.path();
            if p.is_file() {
                size += entry.metadata()?.len();
            } else if p.is_dir() {
                size += dir_size(&p)?;
            }
        }
    }
    Ok(size)
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}
