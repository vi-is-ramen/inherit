use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

pub const INHERIT_IGNORE_FILE: &str = ".inherignore";

/// A wrapper around `.inherignore` that uses gitignore-compatible syntax.
pub struct InheritIgnore {
    inner: Gitignore,
}

impl InheritIgnore {
    /// Creates a filter from `.inherignore` in the specified directory.
    /// If the file does not exist, returns an empty filter (ignores nothing).
    pub fn load(root: &Path) -> Self {
        let file = root.join(INHERIT_IGNORE_FILE);
        if !file.exists() {
            return Self {
                inner: Gitignore::empty(),
            };
        }
        let mut builder = GitignoreBuilder::new(root);
        let _ = builder.add(file);
        let inner = builder.build().unwrap_or_else(|_| Gitignore::empty());
        Self { inner }
    }

    /// Returns `true` if the path should be ignored.
    /// The path must be **relative** to the template root.
    pub fn is_ignored(&self, relative_path: &Path, is_dir: bool) -> bool {
        let m = self.inner.matched(relative_path, is_dir);
        m.is_ignore()
    }
}

/// Always ignored Inherit system artifacts.
pub const ALWAYS_IGNORE: &[&str] = &["Inherit.toml", INHERIT_IGNORE_FILE, ".git"];
