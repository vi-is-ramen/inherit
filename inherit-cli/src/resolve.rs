use crate::config::Config;
use anyhow::{bail, Result};

#[derive(Debug)]
pub enum TemplateSource {
    GitHub { user: String, repo: String },
    Url(String),
}

pub fn resolve_template(name: &str, cfg: &Config) -> Result<TemplateSource> {
    let expanded = cfg.aliases.get(name).map(|s| s.as_str()).unwrap_or(name);

    // Direct URL (file://, http://, https://) or absolute path
    if expanded.starts_with("file://")
        || expanded.starts_with("http://")
        || expanded.starts_with("https://")
        || expanded.starts_with('/')
    {
        return Ok(TemplateSource::Url(expanded.to_string()));
    }

    let parts: Vec<&str> = expanded.splitn(2, '/').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        bail!(
            "Template '{}' must be 'user/repo', a URL, or resolve to one via an alias",
            name
        );
    }
    Ok(TemplateSource::GitHub {
        user: parts[0].to_string(),
        repo: parts[1].to_string(),
    })
}

pub fn source_to_clone_url(src: &TemplateSource, token: Option<&str>) -> String {
    match src {
        TemplateSource::Url(u) => u.clone(),
        TemplateSource::GitHub { user, repo } => {
            if let Some(t) = token {
                format!("https://{}@github.com/{}/{}.git", t, user, repo)
            } else {
                format!("https://github.com/{}/{}.git", user, repo)
            }
        }
    }
}

pub fn public_cache_key(src: &TemplateSource) -> String {
    match src {
        TemplateSource::Url(u) => u.clone(),
        TemplateSource::GitHub { user, repo } => {
            format!("https://github.com/{}/{}.git", user, repo)
        }
    }
}
