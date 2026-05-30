//! KISS template engine for `@VAR@` placeholders.
//!
//! Part of the [`Inherit`](https://crates.io/crates/cargo-inherit) ecosystem.
//! For detailed documentation, examples, and design rationale, see the
//! [Inherit Book — kissreplace chapter](https://vi-is-ramen.github.io/book/en/my-crates/kissreplace).

mod error;
pub use error::{KissReplaceError, Result};
mod strlike;
pub use strlike::Variables;
pub mod replace;
pub use replace::KissReplace;
pub mod scan;
pub mod valid;
