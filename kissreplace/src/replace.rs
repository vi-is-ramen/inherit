use crate::*;
use std::path::PathBuf;

pub trait KissReplace {
    /// Replace `@VAR@` in single string
    fn replace_str(&self, input: &str) -> String;

    /// Immutable string replacement
    fn replace(&self, sources: Vec<String>) -> Vec<String> {
        sources.into_iter().map(|s| self.replace_str(&s)).collect()
    }

    /// Mutable string replacement (in-place, more alloc-efficient)
    fn replace_mut(&self, sources: &mut Vec<String>) {
        for s in sources.iter_mut() {
            *s = self.replace_str(s);
        }
    }

    /// Replace `@VAR@` in paths
    fn replace_paths(&self, paths: Vec<PathBuf>) -> Vec<PathBuf> {
        paths
            .into_iter()
            .map(|p| {
                let s = p.to_string_lossy();
                PathBuf::from(self.replace_str(&s))
            })
            .collect()
    }
}

impl KissReplace for Variables {
    fn replace_str(&self, input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        let mut remaining = input;

        while let Some(at_pos) = remaining.find('@') {
            result.push_str(&remaining[..at_pos]);
            let after_at = &remaining[at_pos + 1..];

            let mut found_valid = false;
            let mut search_pos = 0;

            while let Some(close_at_pos) = after_at[search_pos..].find('@') {
                let actual_close_pos = search_pos + close_at_pos;
                let var_name = &after_at[..actual_close_pos];

                if crate::valid::is_valid_var_name(var_name) {
                    if let Some(value) = self.get(var_name) {
                        result.push_str(value);
                    } else {
                        result.push('@');
                        result.push_str(var_name);
                        result.push('@');
                    }
                    remaining = &after_at[actual_close_pos + 1..];
                    found_valid = true;
                    break;
                } else {
                    search_pos = actual_close_pos + 1;
                }
            }

            if !found_valid {
                result.push('@');
                remaining = after_at;
            }
        }

        result.push_str(remaining);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_vars(pairs: &[(&str, &str)]) -> Variables {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn test_basic_replacement() {
        let vars = make_vars(&[("NAME", "World"), ("PROJECT", "test")]);
        assert_eq!(
            vars.replace_str("Hello @NAME@, project @PROJECT@!"),
            "Hello World, project test!"
        );
    }

    #[test]
    fn test_missing_var_preserved() {
        let vars = make_vars(&[("NAME", "World")]);
        assert_eq!(
            vars.replace_str("@NAME@ and @MISSING@"),
            "World and @MISSING@"
        );
    }

    #[test]
    fn test_empty_at_at() {
        let vars = make_vars(&[("X", "Y")]);
        assert_eq!(vars.replace_str("@@X@@"), "@Y@");
    }

    #[test]
    fn test_invalid_names_preserved() {
        let vars = make_vars(&[("X", "Y")]);
        assert_eq!(
            vars.replace_str("@var-name@ and @123@ and @X@"),
            "@var-name@ and @123@ and Y"
        );
    }

    #[test]
    fn test_unclosed_at() {
        let vars = make_vars(&[("X", "Y")]);
        assert_eq!(vars.replace_str("hello @X and @X@"), "hello @X and Y");
    }

    #[test]
    fn test_consecutive_vars() {
        let vars = make_vars(&[("A", "1"), ("B", "2")]);
        assert_eq!(vars.replace_str("@A@@B@"), "12");
    }

    #[test]
    fn test_no_vars() {
        let vars: Variables = std::collections::HashMap::new();
        assert_eq!(vars.replace_str("no vars here"), "no vars here");
    }

    #[test]
    fn test_empty_string() {
        let vars: Variables = std::collections::HashMap::new();
        assert_eq!(vars.replace_str(""), "");
    }

    #[test]
    fn test_replace_paths() {
        let vars = make_vars(&[("PROJECT", "my_app")]);
        let paths = vec![
            PathBuf::from("src/@PROJECT@/main.rs"),
            PathBuf::from("config/@PROJECT@.toml"),
        ];
        let result = vars.replace_paths(paths);
        assert_eq!(result[0], PathBuf::from("src/my_app/main.rs"));
        assert_eq!(result[1], PathBuf::from("config/my_app.toml"));
    }

    #[test]
    fn test_replace_mut() {
        let vars = make_vars(&[("X", "Y")]);
        let mut v = vec!["@X@".to_string(), "plain".to_string()];
        vars.replace_mut(&mut v);
        assert_eq!(v, vec!["Y", "plain"]);
    }

    #[test]
    fn test_nested_looking() {
        let vars = make_vars(&[("A", "@B@"), ("B", "X")]);
        assert_eq!(vars.replace_str("@A@"), "@B@");
    }
}
