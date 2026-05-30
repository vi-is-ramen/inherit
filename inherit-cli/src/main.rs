mod cli;
mod commands;
mod config;
mod prompt;
mod resolve;

pub(crate) static CM: &str = "\u{2713}"; // checkmark symbol

use anyhow::Result;
use console::style;

fn run() -> Result<()> {
    let raw_args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = cli::parse(raw_args)?;

    let cfg_path = config::config_path()?;
    let mut cfg = config::load_or_create(&cfg_path)?;

    match cmd {
        cli::Command::Generate { template, target } => {
            commands::generate::run(&cfg, template, target)?;
        }
        cli::Command::AliasAdd { template, alias } => {
            commands::alias::add(&cfg_path, &mut cfg, template, alias)?;
        }
        cli::Command::AliasList => {
            commands::alias::list(&cfg)?;
        }
        cli::Command::AliasRemove { name } => {
            commands::alias::remove(&cfg_path, &mut cfg, name)?;
        }
        cli::Command::DefaultSet { name } => {
            commands::default::set(&cfg_path, &mut cfg, name)?;
        }
        cli::Command::DefaultList => {
            commands::default::list(&cfg)?;
        }
        cli::Command::DefaultUnset { name } => {
            commands::default::unset(&cfg_path, &mut cfg, name)?;
        }
        cli::Command::CacheList => {
            commands::cache::list(&cfg)?;
        }
        cli::Command::CacheClean => {
            commands::cache::clean(&cfg)?;
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", style("error:").red().bold(), e);
        let mut source = e.source();
        while let Some(s) = source {
            eprintln!("  {} {}", style("caused by:").red(), s);
            source = s.source();
        }
        std::process::exit(1);
    }
}
