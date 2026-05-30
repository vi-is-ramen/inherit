//! Lazy artifact downloader with local caching.
//!
//! Part of the [`Inherit`](https://crates.io/crates/cargo-inherit) ecosystem.
//! For detailed documentation, examples, and design rationale, see the
//! [Inherit Book — lazyget chapter](https://vi-is-ramen.github.io/book/en/my-crates/lazyget).

mod error;
mod sync_impl;

#[cfg(feature = "async")]
mod async_impl;

pub use error::{LazyGetError, Result};
pub use sync_impl::{fetch, refetch};

#[cfg(feature = "async")]
pub use async_impl::{async_fetch, async_refetch};

use sha2::{Digest, Sha256};

/// Generates stable artifact identifier from URL and optional tag (e. g. Git commit).
///
/// Returns 64-char hex-string (SHA256).
pub fn make_id(url: &str, tag: Option<&str>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    if let Some(c) = tag {
        hasher.update(b":");
        hasher.update(c.as_bytes());
    }
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_make_id_deterministic() {
        let id1 = make_id("https://example.com", Some("abc"));
        let id2 = make_id("https://example.com", Some("abc"));
        assert_eq!(id1, id2);
        assert_eq!(id1.len(), 64);
    }

    #[test]
    fn test_make_id_different_urls() {
        let id1 = make_id("https://example.com/a", None);
        let id2 = make_id("https://example.com/b", None);
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_fetch_basic() {
        let tmp = tempdir().unwrap();
        let id = "test-artifact";

        let mut call_count = 0;
        let result1 = fetch(tmp.path(), id, |dir| {
            call_count += 1;
            std::fs::write(dir.join("marker.txt"), "hello")?;
            Ok(())
        });
        assert!(result1.is_ok());
        assert_eq!(call_count, 1);

        let result2 = fetch(tmp.path(), id, |dir| {
            call_count += 1;
            std::fs::write(dir.join("should_not_exist.txt"), "oops")?;
            Ok(())
        });
        assert!(result2.is_ok());
        assert_eq!(call_count, 1);

        assert!(result2.unwrap().join("marker.txt").exists());
    }

    #[test]
    fn test_fetch_cleanup_on_error() {
        let tmp = tempdir().unwrap();
        let id = "failing-artifact";

        let result = fetch(tmp.path(), id, |_dir| Err("simulated failure".into()));
        assert!(result.is_err());

        let temp_dir = tmp.path().join(format!(".{}-tmp", id));
        assert!(!temp_dir.exists());

        let target_dir = tmp.path().join(id);
        assert!(!target_dir.exists());
    }

    #[test]
    fn test_refetch_clears_cache() {
        let tmp = tempdir().unwrap();
        let id = "refetch-test";

        let mut counter = 0;
        fetch(tmp.path(), id, |dir| {
            counter += 1;
            std::fs::write(dir.join("v1.txt"), "version 1")?;
            Ok(())
        })
        .unwrap();
        assert_eq!(counter, 1);

        refetch(tmp.path(), id, |dir| {
            counter += 1;
            std::fs::write(dir.join("v2.txt"), "version 2")?;
            Ok(())
        })
        .unwrap();
        assert_eq!(counter, 2);
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_async_fetch() {
        let tmp = tempdir().unwrap();
        let id = "async-test";

        let mut call_count = 0;
        let r1 = async_fetch(tmp.path(), id, |dir| {
            call_count += 1;
            async move {
                tokio::fs::write(dir.join("marker.txt"), "async hello").await?;
                Ok(())
            }
        })
        .await;
        assert!(r1.is_ok());
        assert_eq!(call_count, 1);

        let r2 = async_fetch(tmp.path(), id, |_dir| {
            call_count += 1;
            async { Ok(()) }
        })
        .await;
        assert!(r2.is_ok());
        assert_eq!(call_count, 1);
    }
}
