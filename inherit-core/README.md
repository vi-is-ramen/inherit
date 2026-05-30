# inherit-core

> Core library for [`cargo-inherit`](https://crates.io/crates/cargo-inherit).

[![Crates.io](https://img.shields.io/crates/v/inherit-core.svg)](https://crates.io/crates/inherit-core)
[![Documentation](https://img.shields.io/badge/docs-book-blue)](https://vi-is-ramen.github.io/book/en/my-crates/inherit-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

The engine behind `cargo-inherit`: template loading, variable resolution, and project generation.
Use this crate to embed Inherit's templating logic into your own tools.

## Features

- **Manifest-driven**: `Inherit.toml` declares template metadata, variables, and hooks
- **Variable collection**: Automatically scans template files for `@VAR@` placeholders
- **Git-aware ignoring**: `.inherignore` support using gitignore-compatible syntax
- **Safe generation**: Atomic file operations, validation, and clear error reporting
- **Hook system**: Run post-creation commands in the generated project

## Quick Start

Add to your dependencies:

```toml
cargo add inherit-core
```

Basic usage:

```rust
use inherit_core::{load_template, process_template, ProcessOptions, Variables};
use std::path::Path;

// Load template context (manifest + scanned variables)
let ctx = load_template(Path::new("path/to/template"))?;

// Prepare variable values (e.g., from user input or config)
let mut vars = Variables::new();
vars.insert("PROJECT_NAME".into(), "my_project".into());
// ... fill all required variables ...

// Generate the project
let result = process_template(
    Path::new("path/to/template"),
    Path::new("path/to/output"),
    &vars,
    ProcessOptions::default(),
)?;

println!("Generated {} files", result.processed_files);
```

## Documentation

Full API reference, `Inherit.toml` specification, and integration guides are available in the [book](https://vi-is-ramen.github.io/book/en/my-crates/inherit-core).

## License

MIT OR Apache-2.0
