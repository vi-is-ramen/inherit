use anyhow::{bail, Result};
use inherit_core::Variables;
use inquire::{InquireError, Text};
use std::collections::HashMap;

pub fn prompt_for_variables(
    required: &std::collections::HashSet<String>,
    descriptions: &HashMap<String, String>,
    defaults: &HashMap<String, String>,
) -> Result<Variables> {
    let non_interactive = std::env::var("INHERIT_NON_INTERACTIVE").is_ok();
    let mut vars = Variables::new();
    let mut names: Vec<_> = required.iter().cloned().collect();
    names.sort();

    if names.is_empty() {
        return Ok(vars);
    }

    if !non_interactive {
        println!(
            "\n{}",
            console::style("Template requires the following variables:")
                .cyan()
                .bold()
        );
    }

    for name in names {
        let default = defaults.get(&name).map(|s| s.as_str());

        if non_interactive {
            if let Some(d) = default {
                vars.insert(name.clone(), d.to_string());
            } else {
                bail!(
                    "Missing required variable `{}` (no default) in non-interactive mode",
                    name
                );
            }
            continue;
        }

        let desc = descriptions
            .get(&name)
            .map(|s| s.as_str())
            .unwrap_or("Value");
        let temp = format!("{} ({})", name, desc);
        let mut prompt = Text::new(&temp);
        if let Some(d) = default {
            prompt = prompt
                .with_default(d)
                .with_help_message("Press Enter to use default");
        }

        let value = match prompt.prompt() {
            Ok(v) => v,
            Err(InquireError::OperationCanceled) | Err(InquireError::OperationInterrupted) => {
                bail!("Aborted by user");
            }
            Err(e) => return Err(e.into()),
        };

        vars.insert(name, value);
    }

    Ok(vars)
}

/// Requests a single value for the specified variable (for the `default for` command).
pub fn prompt_single_value(name: &str, current: Option<&str>) -> Result<String> {
    let temp = format!("Default value for {}", name);
    let mut prompt = Text::new(&temp);
    if let Some(c) = current {
        prompt = prompt.with_default(c);
    }
    match prompt.prompt() {
        Ok(v) => Ok(v),
        Err(InquireError::OperationCanceled) | Err(InquireError::OperationInterrupted) => {
            bail!("Aborted by user");
        }
        Err(e) => Err(e.into()),
    }
}
