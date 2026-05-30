# kissreplace

> KISS template engine for `@VAR@` placeholders.

[![Crates.io](https://img.shields.io/crates/v/kissreplace.svg)](https://crates.io/crates/kissreplace)
[![Documentation](https://img.shields.io/badge/docs-book-blue)](https://vi-is-ramen.github.io/book/en/my-crates/kissreplace)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A minimal, zero-macro template engine that replaces `@VARIABLE@` patterns in strings and paths. 
Designed for build tools, code generators, and configuration templating where simplicity matters.

## Features

- **Simple syntax**: `@VAR_NAME@` placeholders, nothing more
- **Type-safe**: Uses `HashMap<String, String>` — no proc macros, no DSL
- **Path-aware**: Replace variables in file/directory names, not just file contents
- **Graceful degradation**: Unknown or invalid variables are preserved as-is
- **Optional async**: Async API available behind the `async` feature flag

## Quick Start

Add to your dependencies:

```shell
cargo add kissreplace
```

Basic usage:

```rust
use kissreplace::{KissReplace, Variables};

let mut vars = Variables::new();
vars.insert("PROJECT".to_string(), "my_app".to_string());
vars.insert("VERSION".to_string(), "0.1.0".to_string());

let result = vars.replace_str("crate @PROJECT@ v@VERSION@");
assert_eq!(result, "crate my_app v0.1.0");

// Works with paths too:
let paths = vars.replace_paths(vec![
    std::path::PathBuf::from("src/@PROJECT@/lib.rs"),
]);
```

## Documentation

Full API reference and advanced usage examples are available in the [book](https://vi-is-ramen.github.io/book/en/my-crates/kissreplace).

## License

MIT OR Apache-2.0
