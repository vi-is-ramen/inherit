use anyhow::{bail, Result};

#[derive(Debug)]
pub enum Command {
    Generate {
        template: String,
        target: Option<String>,
    },
    AliasAdd {
        template: String,
        alias: String,
    },
    AliasList,
    AliasRemove {
        name: String,
    },
    DefaultSet {
        name: String,
    },
    DefaultList,
    DefaultUnset {
        name: String,
    },
    CacheList,
    CacheClean,
}

pub fn parse(args: Vec<String>) -> Result<Command> {
    if args.is_empty() {
        bail!(usage());
    }

    match args[0].as_str() {
        "alias" => match args.get(1).map(|s| s.as_str()) {
            Some("list") => Ok(Command::AliasList),
            Some("remove") if args.len() == 3 => Ok(Command::AliasRemove { name: args[2].clone() }),
            _ => bail!("Usage: inherit alias list | inherit alias remove <name>"),
        },
        "default" => match args.get(1).map(|s| s.as_str()) {
            Some("list") => Ok(Command::DefaultList),
            Some("for") if args.len() == 3 => Ok(Command::DefaultSet { name: args[2].clone() }),
            Some("unset") if args.len() == 3 => Ok(Command::DefaultUnset { name: args[2].clone() }),
            _ => bail!("Usage: inherit default list | inherit default for <var> | inherit default unset <var>"),
        },
        "cache" => match args.get(1).map(|s| s.as_str()) {
            Some("list") => Ok(Command::CacheList),
            Some("clean") => Ok(Command::CacheClean),
            _ => bail!("Usage: inherit cache list | inherit cache clean"),
        },
        _ => {
            // Fallback to generate/alias-add syntax
            let template = args[0].clone();
            let rest = &args[1..];
            match rest {
                [] => Ok(Command::Generate { template, target: None }),
                [kw, target] if kw == "to" => Ok(Command::Generate { template, target: Some(target.clone()) }),
                [kw1, kw2, alias] if kw1 == "to" && kw2 == "alias" => {
                    Ok(Command::AliasAdd { template, alias: alias.clone() })
                }
                _ => bail!(usage()),
            }
        }
    }
}

fn usage() -> &'static str {
    r#"Usage:
   inherit <user/repo> [to <dirname>]       # generate project
   inherit <user/repo> to alias <name>      # save alias
   inherit alias list                       # list aliases
   inherit alias remove <name>              # remove alias
   inherit default for <var>                # set default value
   inherit default list                     # list defaults
   inherit default unset <var>              # remove default
   inherit cache list                       # list cached templates
   inherit cache clean                      # clear template cache"#
}
