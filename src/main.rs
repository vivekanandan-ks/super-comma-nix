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
    let is_shell = prog.ends_with(",s") || raw_args.get(1).map_or(false, |a| a == "-s");
    let is_ver = prog.ends_with(",v") || raw_args.get(1).map_or(false, |a| a == "-v");

    if raw_args.len() < 2 || raw_args.iter().any(|a| a == "-h" || a == "--help") {
        println!("super-comma (,) - Usage: , [nixflags='...'] <pkg_spec> [args...] | ,s [nixflags='...'] <specs...> | ,v <pkg>");
        return;
    }

    let flake = env::var("SUPER_COMMA_FLAKE").unwrap_or_else(|_| "github:fzakaria/nixpkgs-multiverse".into());

    if is_ver {
        let pkg_idx = if raw_args[1] == "-v" { 2 } else { 1 };
        if pkg_idx >= raw_args.len() {
            println!("Usage: ,v <package_name> (e.g. ,v python3)");
            return;
        }
        let pkg = raw_args[pkg_idx].replace(&['"', '\''][..], "");
        println!("Available versions for {}:", pkg);
        let expr = format!("f: builtins.concatStringsSep \"\\n\" (f.${{builtins.currentSystem}}.versionsOf \"{}\")", pkg);
        let target = format!("{}#multiverse", flake);
        let _ = Command::new("nix").args(["eval", "--raw", "--impure", "--apply", &expr, &target]).status();
        return;
    }

    let resolve = |spec: &str, scope: Option<&str>| -> (String, String) {
        let (pkg, bin) = spec.split_once(':').unwrap_or((spec, ""));
        let (attr, _base) = if pkg.starts_with("latest.") || pkg.starts_with("tip.") || pkg.starts_with("versions.") || pkg.split('.').next().unwrap_or("").chars().all(|c| c.is_ascii_digit()) || pkg.contains('-') {
            (pkg.to_string(), pkg.split('.').last().unwrap_or(pkg).to_string())
        } else if let Some((p, v)) = pkg.split_once('.') {
            (format!("versions.{}.\"{}\"", p, v), p.to_string())
        } else {
            let rel = scope.unwrap_or("latest");
            (format!("{}.{}", rel, pkg), pkg.to_string())
        };
        (format!("{}#{}", flake, attr), bin.to_string())
    };

    let start_idx = if is_shell { if raw_args[1] == "-s" { 2 } else { 1 } } else { 1 };
    if start_idx >= raw_args.len() {
        println!("Usage: ,s [nixflags='...'] <pkg_spec1> [pkg_spec2...]");
        return;
    }

    let mut nix_flags: Vec<String> = Vec::new();
    if let Ok(env_flags) = env::var("SUPER_COMMA_NIXFLAGS").or_else(|_| env::var("SUPER_COMMA_NIX_FLAGS")) {
        nix_flags.extend(parse_nflags_str(&env_flags));
    }

    let cli_remaining = &raw_args[start_idx..];
    let mut input_specs: Vec<String> = Vec::new();
    let mut extra_args: Vec<String> = Vec::new();

    if is_shell {
        for arg in cli_remaining {
            if let Some(val) = arg.strip_prefix("nixflags=").or_else(|| arg.strip_prefix("nixflag=")) {
                let clean_val = val.trim_matches(|c| c == '\'' || c == '"');
                nix_flags.extend(parse_nflags_str(clean_val));
            } else {
                input_specs.push(arg.clone());
            }
        }
    } else {
        let mut found_target = false;
        for arg in cli_remaining {
            if let Some(val) = arg.strip_prefix("nixflags=").or_else(|| arg.strip_prefix("nixflag=")) {
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
            println!("Usage: ,s [nixflags='...'] <pkg_spec1> [pkg_spec2...]");
        } else {
            println!("Usage: , [nixflags='...'] <pkg_spec> [args...]");
        }
        return;
    }

    let targets: Vec<(String, String)> = input_specs
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
                                    if let Some((attr_name, custom_bin)) = bin_override.split_once(':') {
                                        (format!("{}#{}", hash_part, attr_name), custom_bin.to_string())
                                    } else {
                                        (raw_uri.to_string(), String::new())
                                    }
                                } else {
                                    (raw_uri.to_string(), String::new())
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

    let mut nix = Command::new("nix");

    if is_shell {
        nix.arg("shell");
        nix.args(&nix_flags);
        nix.args(targets.iter().map(|t| &t.0));
    } else {
        let (target_uri, bin_override) = &targets[0];

        if !bin_override.is_empty() {
            nix.arg("shell");
            nix.args(&nix_flags);
            nix.arg(target_uri).arg("-c").arg(bin_override).args(&extra_args);
        } else {
            nix.arg("run");
            nix.args(&nix_flags);
            nix.arg(target_uri).arg("--").args(&extra_args);
        }
    }

    let _ = nix.exec();
}

