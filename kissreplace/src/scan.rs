use crate::valid::is_valid_var_name;
use std::collections::HashSet;

pub fn extract_vars(content: &str) -> HashSet<String> {
    let mut out = HashSet::new();
    let mut pos = 0;
    while pos < content.len() {
        let rest = &content[pos..];
        let at = match rest.find('@') {
            Some(i) => i,
            None => break,
        };
        let after_at = &rest[at + 1..];
        let mut search_pos = 0;
        let mut matched = None;
        while let Some(close) = after_at[search_pos..].find('@') {
            let actual = search_pos + close;
            let cand = &after_at[..actual];
            if is_valid_var_name(cand) {
                matched = Some((cand.to_string(), at + 1 + actual + 1));
                break;
            }
            search_pos = actual + 1;
        }
        if let Some((name, advance)) = matched {
            out.insert(name);
            pos += advance;
        } else {
            pos += at + 1;
        }
    }
    out
}
