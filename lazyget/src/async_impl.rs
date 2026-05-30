use crate::error::{LazyGetError, Result};
use std::path::{Path, PathBuf};
use tokio::fs;

/// Fetches artifact with local cache.
///
/// If directory `cache_dir/artifact_id` already exists returns its path;
/// otherwise creates temp dir, calles `fetch_fn` and atomicly moves it.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub async fn async_fetch<P, F, Fut>(cache_dir: P, artifact_id: &str, fetch_fn: F) -> Result<PathBuf>
where
    P: AsRef<Path>,
    F: FnOnce(PathBuf) -> Fut,
    Fut: std::future::Future<
        Output = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>,
    >,
{
    let cache_dir = cache_dir.as_ref().to_path_buf();
    let target_dir = cache_dir.join(artifact_id);

    if target_dir.exists() {
        return Ok(target_dir);
    }

    fs::create_dir_all(&cache_dir)
        .await
        .map_err(|e| LazyGetError::CacheCreate {
            path: cache_dir.clone(),
            source: e,
        })?;

    let temp_dir = cache_dir.join(format!(".{}-tmp", artifact_id));
    if temp_dir.exists() {
        let _ = fs::remove_dir_all(&temp_dir).await;
    }
    fs::create_dir_all(&temp_dir).await?;

    let fetch_result = fetch_fn(temp_dir.clone()).await;

    match fetch_result {
        Ok(()) => {
            fs::rename(&temp_dir, &target_dir)
                .await
                .map_err(|e| LazyGetError::AtomicRename {
                    from: temp_dir,
                    to: target_dir.clone(),
                    source: e,
                })?;
            Ok(target_dir)
        }
        Err(e) => {
            let _ = fs::remove_dir_all(&temp_dir).await;
            Err(LazyGetError::Fetch(e))
        }
    }
}

/// Forcely refetches artifact.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub async fn async_refetch<P, F, Fut>(
    cache_dir: P,
    artifact_id: &str,
    fetch_fn: F,
) -> Result<PathBuf>
where
    P: AsRef<Path>,
    F: FnOnce(PathBuf) -> Fut,
    Fut: std::future::Future<
        Output = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>,
    >,
{
    let target_dir = cache_dir.as_ref().join(artifact_id);
    if target_dir.exists() {
        fs::remove_dir_all(&target_dir).await?;
    }
    async_fetch(cache_dir, artifact_id, fetch_fn).await
}
