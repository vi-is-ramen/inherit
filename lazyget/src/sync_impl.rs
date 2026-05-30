use crate::error::{LazyGetError, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Fetches artifact with local cache.
///
/// If directory `cache_dir/artifact_id` already exists returns its path;
/// otherwise creates temp dir, calles `fetch_fn` and atomicly moves it.
pub fn fetch<P, F>(cache_dir: P, artifact_id: &str, fetch_fn: F) -> Result<PathBuf>
where
    P: AsRef<Path>,
    F: FnOnce(&Path) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>,
{
    let cache_dir = cache_dir.as_ref();
    let target_dir = cache_dir.join(artifact_id);

    if target_dir.exists() {
        return Ok(target_dir);
    }

    fs::create_dir_all(cache_dir).map_err(|e| LazyGetError::CacheCreate {
        path: cache_dir.to_path_buf(),
        source: e,
    })?;

    let temp_dir = cache_dir.join(format!(".{}-tmp", artifact_id));
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&temp_dir)?;

    let fetch_result = fetch_fn(&temp_dir);

    match fetch_result {
        Ok(()) => {
            fs::rename(&temp_dir, &target_dir).map_err(|e| LazyGetError::AtomicRename {
                from: temp_dir.clone(),
                to: target_dir.clone(),
                source: e,
            })?;
            Ok(target_dir)
        }
        Err(e) => {
            let _ = fs::remove_dir_all(&temp_dir);
            Err(LazyGetError::Fetch(e))
        }
    }
}

/// Forcely refetches artifact.
pub fn refetch<P, F>(cache_dir: P, artifact_id: &str, fetch_fn: F) -> Result<PathBuf>
where
    P: AsRef<Path>,
    F: FnOnce(&Path) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>,
{
    let target_dir = cache_dir.as_ref().join(artifact_id);
    if target_dir.exists() {
        fs::remove_dir_all(&target_dir)?;
    }
    fetch(cache_dir, artifact_id, fetch_fn)
}
