use crate::ignore::{InheritIgnore, ALWAYS_IGNORE};
use ignore::Walk;
use std::collections::HashSet;
use std::path::Path;

pub fn collect_variables(
    source_dir: &Path,
    ignore: &InheritIgnore,
) -> std::io::Result<HashSet<String>> {
    let mut vars = HashSet::new();

    for entry in Walk::new(source_dir) {
        let entry = entry.map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
        let path = entry.path();
        let rel = match path.strip_prefix(source_dir) {
            Ok(r) if r.as_os_str().is_empty() => continue,
            Ok(r) => r,
            Err(_) => continue,
        };

        let rel_str = rel.to_string_lossy();
        if ALWAYS_IGNORE
            .iter()
            .any(|&x| rel_str == x || rel_str.starts_with(&format!("{x}/")))
        {
            continue;
        }

        let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        if ignore.is_ignored(rel, is_dir) {
            continue;
        }

        if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            continue;
        }

        if let Ok(content) = std::fs::read_to_string(path) {
            for name in kissreplace::scan::extract_vars(&content) {
                vars.insert(name);
            }
        }
    }

    Ok(vars)
}
