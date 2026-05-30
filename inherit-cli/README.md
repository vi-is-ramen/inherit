# cargo-inherit

> Generate new Rust projects from templates — with variables, hooks, and zero boilerplate.

[![Crates.io](https://img.shields.io/crates/v/cargo-inherit.svg)](https://crates.io/crates/cargo-inherit)
[![Documentation](https://img.shields.io/badge/docs-book-blue)](https://vi-is-ramen.github.io/book/en/my-crates/cargo-inherit)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

`cargo-inherit` is a CLI tool that scaffolds new projects from Git-hosted templates.
Define variables in `Inherit.toml`, use `@VAR@` placeholders in your files, and let Inherit handle the rest.

## Installation

```sh
cargo install cargo-inherit
```

## Quick Start

Generate a project from a GitHub template:

```sh
# Basic usage
cargo inherit user/template-repo

# With custom output directory
cargo inherit user/template-repo to my-project

# Using an alias (configured in ~/.config/inherit/config.toml)
cargo inherit rust-lib to my-lib
```

Template example (`Inherit.toml`):

```toml
[template]
name = "rust-lib"
description = "Minimal Rust library template"

[variables]
PROJECT_NAME = "Name of the crate"
AUTHOR = "Your name and email"
LICENSE = "License identifier (MIT, Apache-2.0, etc.)"

[hooks]
post_create = [
  "cargo fmt",
  "cargo clippy --fix --allow-dirty"
]
```

Use variables in your template files:

```toml
# Cargo.toml template
[package]
name = "@PROJECT_NAME@"
version = "0.1.0"
authors = ["@AUTHOR@"]
license = "@LICENSE@"
```

## Configuration

Inherit uses `~/.config/inherit/config.toml` for:
- Default variable values (skips prompts for known vars)
- Template aliases (`rust-lib` → `vi-is-ramen/rust-template`)
- Cache directory, GitHub token, post-generation commands

Run `cargo inherit` once to generate a commented config template.

## Documentation

Full CLI reference, template authoring guide, and configuration options are available in the [book](https://vi-is-ramen.github.io/book/en/my-crates/cargo-inherit).

## License

MIT OR Apache-2.0
