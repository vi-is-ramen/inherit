use crate::config::{self, Config};
use crate::prompt::prompt_for_variables;
use crate::resolve::{public_cache_key, resolve_template, source_to_clone_url, TemplateSource};
use anyhow::{Context, Result};
use console::style;
use inherit_core::{load_template, process_template, ProcessOptions};
use lazyget::fetch;
use std::path::PathBuf;
use std::process::Command;

pub fn run(cfg: &Config, template: String, target: Option<String>) -> Result<()> {
    let target_dir = match target {
        None => std::env::current_dir()?,
        Some(dir) => {
            let p = PathBuf::from(&dir);
            if p.is_absolute() {
                p
            } else {
                std::env::current_dir()?.join(p)
            }
        }
    };

    let source = resolve_template(&template, cfg)?;
    let url = source_to_clone_url(&source, cfg.github_token.as_deref());

    let (display_user, display_repo) = match &source {
        TemplateSource::GitHub { user, repo } => (user.as_str(), repo.as_str()),
        TemplateSource::Url(u) => ("<url>", u.as_str()),
    };

    println!(
        "{} Resolving template {} / {}",
        style("->").blue(),
        style(display_user).yellow(),
        style(display_repo).yellow()
    );

    let cache = config::cache_dir(cfg)?;
    let artifact_id = lazyget::make_id(&public_cache_key(&source), None);

    let source_dir = fetch(&cache, &artifact_id, |dir| {
        let status = Command::new("git")
            .args(["clone", "--depth", "1", &url, &dir.to_string_lossy()])
            .status()
            .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { Box::new(e) })?;
        if !status.success() {
            return Err(format!("git clone failed with status {}", status).into());
        }
        Ok(())
    })
    .context("Failed to fetch template")?;

    println!("{} Template loaded from cache", style(crate::CM).green());

    let ctx = load_template(&source_dir).context("Failed to load template manifest")?;

    let final_vars =
        prompt_for_variables(&ctx.required_vars, &ctx.var_descriptions, &cfg.defaults)?;

    let opts = ProcessOptions {
        init_git: cfg.init_git,
        run_hooks: cfg.run_hooks,
    };

    println!(
        "{} Generating project at {}",
        style("->").blue(),
        style(target_dir.display()).yellow()
    );

    let result = process_template(&source_dir, &target_dir, &final_vars, opts)
        .context("Failed to process template")?;

    println!(
        "{} Done! Processed {} files ({} binary)",
        style(crate::CM).green().bold(),
        result.processed_files,
        result.binary_files
    );

    if let Some(cmd) = cfg.open_with.as_deref() {
        if !cmd.trim().is_empty() {
            println!(
                "{} Opening with `{}`",
                style("->").blue(),
                style(cmd).yellow()
            );
            let mut parts = cmd.split_whitespace();
            let program = parts.next().unwrap();
            if let Err(e) = Command::new(program).args(parts).arg(&target_dir).status() {
                eprintln!("{} Failed to run `{}`: {}", style("!").red(), cmd, e);
            }
        }
    }

    Ok(())
}
