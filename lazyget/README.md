# lazyget

> Lazy artifact downloader with local caching.

[![Crates.io](https://img.shields.io/crates/v/lazyget.svg)](https://crates.io/crates/lazyget)
[![Documentation](https://img.shields.io/badge/docs-book-blue)](https://vi-is-ramen.github.io/book/en/my-crates/lazyget)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

`lazyget` provides a simple, atomic caching layer for downloading or generating artifacts. 
Perfect for build tools, template engines, or any workflow where you want to avoid redundant work.

## Features

- **Atomic operations**: Uses temp directories and atomic renames to prevent partial/corrupted cache entries
- **Sync & Async**: Choose the API that fits your project
- **Zero-fuss**: Only `thiserror`, `sha2`, and `hex` (async mode adds optional `tokio`)
- **Idempotent**: Calling `fetch` multiple times with the same ID invokes your closure only once

## Quick Start

Add to your dependencies:

```shell
cargo add lazyget
```

Basic usage:

```rust
use lazyget::{fetch, make_id};

let cache_dir = std::env::temp_dir().join("my-app-cache");
let id = make_id("https://example.com/artifact.zip", Some("v1.2.3"));

let path = fetch(&cache_dir, &id, |dir| {
    // Download or generate your artifact into `dir`
    std::fs::write(dir.join("result.txt"), "hello")?;
    Ok(())
})?;

println!("Artifact ready at: {:?}", path);
```

## Documentation

Full API reference and advanced usage examples are available in the [book](https://vi-is-ramen.github.io/book/en/my-crates/lazyget).

## License

MIT OR Apache-2.0
