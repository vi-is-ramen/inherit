# Inherit

> Git-native project templating for Rust.

[![CI](https://github.com/vi-is-ramen/inherit/actions/workflows/ci.yml/badge.svg)](https://github.com/vi-is-ramen/inherit/actions/workflows/ci.yml)
[![Documentation](https://img.shields.io/badge/docs-book-blue)](https://vi-is-ramen.github.io/book/en/my-crates/cargo-inherit)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Inherit is a workspace of crates for scaffolding Rust projects from templates:

| Crate | Purpose |
|-------|---------|
| [`cargo-inherit`](./inherit-cli) | CLI tool for generating projects from Git templates |
| [`inherit-core`](./inherit-core) | Core templating engine (for embedding in other tools) |
| [`kissreplace`](./kissreplace) | Minimal `@VAR@` template engine |
| [`lazyget`](./lazyget) | Lazy artifact downloader with atomic caching |

## Book

All documentation, tutorials, and design notes live in the [book](https://vi-is-ramen.github.io/book).

## Quick Start

Install the CLI:

``sh
cargo install cargo-inherit
``

Generate a project:

``sh
cargo inherit vi-is-ramen/rust-lib-template to my-crate
``

## Development

This workspace uses [`just`](https://just.systems) for common tasks:

``sh
just check    # Run fmt, clippy, test, build (same as CI)
just fmt-fix  # Auto-format code
just clippy-fix  # Auto-fix clippy warnings where possible
``

## License

MIT OR Apache-2.0
