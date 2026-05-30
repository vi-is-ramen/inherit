use thiserror::Error;

#[derive(Error, Debug)]
pub enum KissReplaceError {
    #[error("Invalid variable name: {0}")]
    InvalidVariableName(String),

    #[error("Invalid UTF-8 in path")]
    InvalidUtf8,
}

pub type Result<T> = std::result::Result<T, KissReplaceError>;
