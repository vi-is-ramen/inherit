use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LazyGetError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Fetch operation failed")]
    Fetch(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("Failed to create cache directory {path}")]
    CacheCreate {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Atomic rename failed from {from} to {to}")]
    AtomicRename {
        from: PathBuf,
        to: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, LazyGetError>;
