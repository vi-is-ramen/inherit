//! Core library for [`cargo-inherit`](https://crates.io/crates/cargo-inherit).
//!
//! This crate provides the templating engine, manifest parsing, and project generation logic.
//! For detailed documentation, examples, and design rationale, see the
//! [Inherit Book — inherit-core chapter](https://vi-is-ramen.github.io/book/en/my-crates/inherit-core).

pub mod error;
pub mod ignore;
pub mod manifest;
pub mod pipeline;
pub mod scanner;

pub use error::{InheritError, Result};
pub use kissreplace::Variables;
pub use manifest::Manifest;
pub use pipeline::{
    load_template, process_template, ProcessOptions, ProcessResult, TemplateContext,
};
