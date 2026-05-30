use crate::error::{InheritError, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Default)]
pub struct Manifest {
    #[serde(default)]
    pub template: TemplateInfo,

    /// Variables declared in the manifest.
    /// Key is the variable name, Value is the description/hint for the user.
    #[serde(default)]
    pub variables: HashMap<String, String>,

    #[serde(default)]
    pub hooks: Hooks,
}

#[derive(Debug, Deserialize, Default)]
pub struct TemplateInfo {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Hooks {
    #[serde(default)]
    pub post_create: Vec<String>,
}

impl Manifest {
    pub fn load(source_dir: &Path) -> Result<Self> {
        let path = source_dir.join("Inherit.toml");
        if !path.exists() {
            return Err(InheritError::ManifestNotFound(path));
        }
        let content = fs::read_to_string(&path)?;
        let manifest: Manifest = toml::from_str(&content)?;
        Ok(manifest)
    }
}
