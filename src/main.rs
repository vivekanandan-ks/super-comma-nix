use std::{env, os::unix::process::CommandExt, process::Command};

fn parse_nflags_str(s: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = ' ';

    for c in s.chars() {
        match c {
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = c;
            }
            c if in_quotes && c == quote_char => {
                in_quotes = false;
            }
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    args.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        args.push(current);
    }
    args
}

fn main() {
    let raw_args: Vec<String> = env::args().collect();
    let prog = env::args().next().unwrap_or_default();

    let mut super_flags = Vec::new();
    if let Ok(env_sflags) = env::var("SUPER_COMMA_FLAGS") {
        super_flags.extend(parse_nflags_str(&env_sflags));
    }

    let all_super_args: Vec<String> = raw_args
        .iter()
        .cloned()
        .chain(super_flags.into_iter())
        .collect();

    let is_shell = prog.ends_with(",s") || all_super_args.iter().skip(1).any(|a| a == "-s");
    let is_ver = prog.ends_with(",v") || all_super_args.iter().skip(1).any(|a| a == "-v");

    if raw_args.len() < 2 || all_super_args.iter().any(|a| a == "-h" || a == "--help") {
        println!(
            "super-comma (,) - Usage: , [--nom] [-o] [nixflags='...'] <pkg_spec> [args...] | ,s [--nom] [-o] [nixflags='...'] <specs...> | ,v <pkg>"
        );
        return;
    }

    let output_only = all_super_args.iter().any(|a| a == "-o" || a == "--output");
    let is_nom = all_super_args.iter().any(|a| a == "--nom");
    let flake = env::var("SUPER_COMMA_FLAKE")
        .unwrap_or_else(|_| "github:fzakaria/nixpkgs-multiverse".into());

    if is_ver {
        let pkg_args: Vec<String> = raw_args
            .iter()
            .skip(1)
            .filter(|a| *a != "-o" && *a != "--output" && *a != "--nom" && *a != "-v")
            .cloned()
            .collect();
        if pkg_args.is_empty() {
            println!("Usage: ,v [-o] <package_name> (e.g. ,v python3)");
            return;
        }
        let pkg = pkg_args[0].replace(&['"', '\''][..], "");
        let expr = format!(
            "f: builtins.concatStringsSep \"\\n\" (f.${{builtins.currentSystem}}.versionsOf \"{}\")",
            pkg
        );
        let target = format!("{}#multiverse", flake);

        let cmd_tokens = vec![
            "nix".to_string(),
            "eval".to_string(),
            "--raw".to_string(),
            "--impure".to_string(),
            "--apply".to_string(),
            expr,
            target,
        ];

        if output_only {
            let formatted = cmd_tokens
                .iter()
                .map(|tok| {
                    if tok.contains(' ')
                        || tok.contains('\t')
                        || tok.contains('\n')
                        || tok.is_empty()
                    {
                        format!("\"{}\"", tok)
                    } else {
                        tok.clone()
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            println!("{}", formatted);
            return;
        }

        println!("Available versions for {}:", pkg);
        let _ = Command::new(&cmd_tokens[0]).args(&cmd_tokens[1..]).status();
        return;
    }

    let resolve = |spec: &str, scope: Option<&str>| -> (String, String, String) {
        let (pkg, bin) = spec.split_once(':').unwrap_or((spec, ""));
        let (attr, base) = if pkg.starts_with("latest.")
            || pkg.starts_with("tip.")
            || pkg.starts_with("versions.")
            || pkg
                .split('.')
                .next()
                .unwrap_or("")
                .chars()
                .all(|c| c.is_ascii_digit())
        {
            (
                pkg.to_string(),
                pkg.split('.').last().unwrap_or(pkg).to_string(),
            )
        } else if let Some((p, v)) = pkg.split_once('.') {
            (format!("versions.{}.\"{}\"", p, v), p.to_string())
        } else {
            let rel = scope.unwrap_or("latest");
            (format!("{}.{}", rel, pkg), pkg.to_string())
        };
        (format!("{}#{}", flake, attr), bin.to_string(), base)
    };

    let mut nix_flags: Vec<String> = Vec::new();
    if let Ok(env_flags) =
        env::var("SUPER_COMMA_NIXFLAGS").or_else(|_| env::var("SUPER_COMMA_NIX_FLAGS"))
    {
        nix_flags.extend(parse_nflags_str(&env_flags));
    }

    let cli_remaining = &raw_args[1..];
    let mut input_specs: Vec<String> = Vec::new();
    let mut extra_args: Vec<String> = Vec::new();

    if is_shell {
        for arg in cli_remaining {
            if arg == "-o" || arg == "--output" || arg == "--nom" || arg == "-s" {
                continue;
            }
            if let Some(val) = arg
                .strip_prefix("nixflags=")
                .or_else(|| arg.strip_prefix("nixflag="))
            {
                let clean_val = val.trim_matches(|c| c == '\'' || c == '"');
                nix_flags.extend(parse_nflags_str(clean_val));
            } else {
                input_specs.push(arg.clone());
            }
        }
    } else {
        let mut found_target = false;
        for arg in cli_remaining {
            if arg == "-o" || arg == "--output" || arg == "--nom" {
                continue;
            }
            if let Some(val) = arg
                .strip_prefix("nixflags=")
                .or_else(|| arg.strip_prefix("nixflag="))
            {
                let clean_val = val.trim_matches(|c| c == '\'' || c == '"');
                nix_flags.extend(parse_nflags_str(clean_val));
            } else if !found_target {
                input_specs.push(arg.clone());
                found_target = true;
            } else {
                extra_args.push(arg.clone());
            }
        }
    }

    if input_specs.is_empty() {
        if is_shell {
            println!("Usage: ,s [--nom] [-o] [nixflags='...'] <pkg_spec1> [pkg_spec2...]");
        } else {
            println!("Usage: , [--nom] [-o] [nixflags='...'] <pkg_spec> [args...]");
        }
        return;
    }

    let targets: Vec<(String, String, String)> = input_specs
        .iter()
        .flat_map(|arg| {
            let clean_arg = arg.replace(&['"', '\''][..], "");
            match clean_arg.split_once('=') {
                Some((prefix, list)) => {
                    let clean_prefix = prefix.replace(&['"', '\''][..], "");
                    if clean_prefix == "f" || clean_prefix == "flake" {
                        list.split(',')
                            .map(|p| p.trim())
                            .filter(|p| !p.is_empty())
                            .map(|raw_uri| {
                                if let Some((hash_part, bin_override)) = raw_uri.split_once('#') {
                                    if let Some((attr_name, custom_bin)) =
                                        bin_override.split_once(':')
                                    {
                                        (
                                            format!("{}#{}", hash_part, attr_name),
                                            custom_bin.to_string(),
                                            attr_name.to_string(),
                                        )
                                    } else {
                                        (
                                            raw_uri.to_string(),
                                            String::new(),
                                            bin_override.to_string(),
                                        )
                                    }
                                } else {
                                    (raw_uri.to_string(), String::new(), String::new())
                                }
                            })
                            .collect()
                    } else {
                        list.split(',')
                            .map(|p| p.trim())
                            .filter(|p| !p.is_empty())
                            .map(|p| resolve(p, Some(&clean_prefix)))
                            .collect()
                    }
                }
                None => vec![resolve(clean_arg.trim(), None)],
            }
        })
        .collect();

    if targets.is_empty() {
        println!("Error: No valid package specs provided.");
        return;
    }

    let main_exec = if is_nom { "nom" } else { "nix" };
    let mut cmd_tokens = vec![main_exec.to_string()];

    if is_shell {
        cmd_tokens.push("shell".into());
        cmd_tokens.extend(nix_flags);
        cmd_tokens.extend(targets.iter().map(|t| t.0.clone()));
    } else {
        let (target_uri, bin_override, default_bin_name) = &targets[0];
        let exec_bin = if !bin_override.is_empty() {
            bin_override
        } else {
            default_bin_name
        };

        if is_nom || !bin_override.is_empty() {
            cmd_tokens.push("shell".into());
            cmd_tokens.extend(nix_flags);
            cmd_tokens.push(target_uri.clone());
            cmd_tokens.push("-c".into());
            cmd_tokens.push(exec_bin.clone());
            cmd_tokens.extend(extra_args);
        } else {
            cmd_tokens.push("run".into());
            cmd_tokens.extend(nix_flags);
            cmd_tokens.push(target_uri.clone());
            cmd_tokens.push("--".into());
            cmd_tokens.extend(extra_args);
        }
    }

    if output_only {
        let formatted = cmd_tokens
            .iter()
            .map(|tok| {
                if tok.contains(' ') || tok.contains('\t') || tok.is_empty() {
                    format!("\"{}\"", tok)
                } else {
                    tok.clone()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", formatted);
        return;
    }

    let mut nix = Command::new(&cmd_tokens[0]);
    nix.args(&cmd_tokens[1..]);
    let _ = nix.exec();
}
