use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InheritError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Manifest `{0}` not found in template")]
    ManifestNotFound(PathBuf),

    #[error("Failed to parse manifest: {0}")]
    ManifestParse(#[from] toml::de::Error),

    #[error("The following required variables are missing: {0:?}")]
    MissingVariables(Vec<String>),

    #[error("Invalid variable name `{0}` in manifest")]
    InvalidVariable(String),

    #[error("Command `{cmd}` failed with status {status}")]
    CommandFailed {
        cmd: String,
        status: std::process::ExitStatus,
    },

    #[error("kissreplace error: {0}")]
    KissReplace(#[from] kissreplace::KissReplaceError),
}

pub type Result<T> = std::result::Result<T, InheritError>;
