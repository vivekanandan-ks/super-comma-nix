use std::{env, os::unix::process::CommandExt, process::Command};

fn main() {
    let raw_args: Vec<String> = env::args().collect();
    let prog = env::args().next().unwrap_or_default();
    let is_shell = prog.ends_with(",s") || raw_args.get(1).map_or(false, |a| a == "-s");
    let is_ver = prog.ends_with(",v") || raw_args.get(1).map_or(false, |a| a == "-v");

    if raw_args.len() < 2 || raw_args[1] == "-h" || raw_args[1] == "--help" {
        println!("super-comma (,) - Usage: , <pkg_spec> [args...] | ,s <specs...> | ,v <pkg>");
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

    let args: Vec<String> = raw_args.iter().map(|a| a.replace(&['"', '\''][..], "")).collect();

    let resolve = |spec: &str, scope: Option<&str>| -> (String, String) {
        let (pkg, bin) = spec.split_once(':').unwrap_or((spec, ""));
        let (attr, base) = if pkg.starts_with("latest.") || pkg.starts_with("tip.") || pkg.starts_with("versions.") || pkg.split('.').next().unwrap_or("").chars().all(|c| c.is_ascii_digit()) || pkg.contains('-') {
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
    if start_idx >= args.len() {
        println!("Usage: ,s <pkg_spec1> [pkg_spec2...]");
        return;
    }

    let input_specs = if is_shell { &args[start_idx..] } else { &args[start_idx..start_idx + 1] };

    let targets: Vec<(String, String)> = input_specs
        .iter()
        .flat_map(|arg| match arg.split_once('=') {
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
            None => vec![resolve(arg.trim(), None)],
        })
        .collect();

    if targets.is_empty() {
        println!("Error: No valid package specs provided.");
        return;
    }

    let mut nix = Command::new("nix");

    if is_shell {
        nix.arg("shell").args(targets.iter().map(|t| &t.0));
    } else {
        let (target_uri, bin_override) = &targets[0];
        let extra_args = if start_idx + 1 < raw_args.len() { &raw_args[start_idx + 1..] } else { &[] };

        if !bin_override.is_empty() {
            nix.arg("shell").arg(target_uri).arg("-c").arg(bin_override).args(extra_args);
        } else {
            nix.arg("run").arg(target_uri).arg("--").args(extra_args);
        }
    }

    let _ = nix.exec();
}
