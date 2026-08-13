mod parser;
mod resolver;
mod sandbox;

use parser::{get_super_flags, parse_nflags_str};
use resolver::resolve;
use sandbox::SandboxConfig;

use std::env;
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    let raw_args: Vec<String> = env::args().collect();
    let prog = env::args().next().unwrap_or_default();
    let all_super_args = get_super_flags(&raw_args);

    let is_shell = prog.ends_with(",s")
        || all_super_args.iter().skip(1).any(|a| a == "-s")
        || raw_args.get(1).map_or(false, |a| a == "s" || a == ",s");
    let is_ver = prog.ends_with(",v")
        || all_super_args.iter().skip(1).any(|a| a == "-v")
        || raw_args.get(1).map_or(false, |a| a == "v" || a == ",v");

    if raw_args.len() < 2 || all_super_args.iter().any(|a| a == "-h" || a == "--help") {
        println!(
            "super-comma (,) - Usage: , [--sandbox] [--net] [--rw=...] [--ro=...] [--nom] [-o] [nixflags='...'] <pkg_spec> [args...] | ,s ... | ,v <pkg>"
        );
        return;
    }

    let output_only = all_super_args.iter().any(|a| a == "-o" || a == "--output");
    let is_nom = all_super_args.iter().any(|a| a == "--nom");
    let sandbox_cfg = SandboxConfig::from_args(&all_super_args);
    let flake = env::var("SUPER_COMMA_FLAKE")
        .unwrap_or_else(|_| "github:fzakaria/nixpkgs-multiverse".into());

    if is_ver {
        let pkg_args: Vec<String> = raw_args
            .iter()
            .skip(1)
            .filter(|a| {
                *a != "-o"
                    && *a != "--output"
                    && *a != "--nom"
                    && *a != "-v"
                    && *a != "--sandbox"
                    && *a != "--net"
                    && !a.starts_with("--rw=")
                    && !a.starts_with("--ro=")
                    && !a.starts_with("--rox=")
            })
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
            println!("{}", format_cmd(&cmd_tokens));
            return;
        }

        println!("Available versions for {}:", pkg);
        let _ = Command::new(&cmd_tokens[0]).args(&cmd_tokens[1..]).status();
        return;
    }

    let mut nix_flags: Vec<String> = Vec::new();
    if let Ok(env_flags) =
        env::var("SUPER_COMMA_NIXFLAGS").or_else(|_| env::var("SUPER_COMMA_NIX_FLAGS"))
    {
        nix_flags.extend(parse_nflags_str(&env_flags));
    }

    let start_idx = if raw_args.len() > 1
        && (raw_args[1] == "s" || raw_args[1] == ",s" || raw_args[1] == "v" || raw_args[1] == ",v")
    {
        2
    } else {
        1
    };
    let cli_remaining = &raw_args[start_idx..];
    let mut input_specs: Vec<String> = Vec::new();
    let mut extra_args: Vec<String> = Vec::new();

    let is_sandbox_flag = |arg: &str| {
        arg == "-o"
            || arg == "--output"
            || arg == "--nom"
            || arg == "-s"
            || arg == "--sandbox"
            || arg == "--net"
            || arg.starts_with("--rw=")
            || arg.starts_with("--ro=")
            || arg.starts_with("--rox=")
    };

    if is_shell {
        let mut in_extra_args = false;
        for arg in cli_remaining {
            if in_extra_args {
                extra_args.push(arg.clone());
                continue;
            }
            if arg == "--" {
                in_extra_args = true;
                continue;
            }
            if is_sandbox_flag(arg) {
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
            if is_sandbox_flag(arg) {
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
            println!(
                "Usage: ,s [--sandbox] [--net] [--rw=...] [--nom] [-o] [nixflags='...'] <specs...> [-- <cmd> [args...]]"
            );
        } else {
            println!(
                "Usage: , [--sandbox] [--net] [--rw=...] [--nom] [-o] [nixflags='...'] <pkg_spec> [args...]"
            );
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
                            .map(|p| resolve(p, Some(&clean_prefix), &flake))
                            .collect()
                    }
                }
                None => vec![resolve(clean_arg.trim(), None, &flake)],
            }
        })
        .collect();

    if targets.is_empty() {
        println!("Error: No valid package specs provided.");
        return;
    }

    let main_exec = if is_nom { "nom" } else { "nix" };
    let mut cmd_tokens = vec![main_exec.to_string()];
    let sandbox_prefix = sandbox_cfg.build_prefix();

    if is_shell {
        cmd_tokens.push("shell".into());
        cmd_tokens.extend(nix_flags);
        cmd_tokens.extend(targets.iter().map(|t| t.0.clone()));

        if !extra_args.is_empty() || sandbox_cfg.enabled {
            cmd_tokens.push("-c".into());
            if sandbox_cfg.enabled {
                cmd_tokens.extend(sandbox_prefix);
            }
            if !extra_args.is_empty() {
                cmd_tokens.extend(extra_args);
            } else {
                cmd_tokens.push(env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into()));
            }
        }
    } else {
        let (target_uri, bin_override, default_bin_name) = &targets[0];
        let exec_bin = if !bin_override.is_empty() {
            bin_override
        } else {
            default_bin_name
        };

        if sandbox_cfg.enabled || is_nom || !bin_override.is_empty() {
            cmd_tokens.push("shell".into());
            cmd_tokens.extend(nix_flags);
            cmd_tokens.push(target_uri.clone());
            cmd_tokens.push("-c".into());

            if sandbox_cfg.enabled {
                cmd_tokens.extend(sandbox_prefix);
                cmd_tokens.push(exec_bin.clone());
            } else {
                cmd_tokens.push(exec_bin.clone());
            }
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
        println!("{}", format_cmd(&cmd_tokens));
        return;
    }

    let mut nix = Command::new(&cmd_tokens[0]);
    nix.args(&cmd_tokens[1..]);
    let _ = nix.exec();
}

fn format_cmd(tokens: &[String]) -> String {
    tokens
        .iter()
        .map(|tok| {
            if tok.contains(' ') || tok.contains('\t') || tok.contains('\n') || tok.is_empty() {
                format!("\"{}\"", tok)
            } else {
                tok.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
