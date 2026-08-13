use crate::parser::parse_nflags_str;
use std::env;

#[derive(Debug, Default, Clone)]
pub struct SandboxConfig {
    pub enabled: bool,
    pub allow_net: bool,
    pub rw_paths: Vec<String>,
    pub ro_paths: Vec<String>,
    pub rox_paths: Vec<String>,
}

impl SandboxConfig {
    pub fn from_args(all_super_args: &[String]) -> Self {
        let enabled = all_super_args.iter().any(|a| a == "--sandbox");
        let allow_net = all_super_args.iter().any(|a| a == "--net");

        let mut rw_paths = Vec::new();
        let mut ro_paths = Vec::new();
        let mut rox_paths = Vec::new();

        for arg in all_super_args {
            if let Some(paths) = arg.strip_prefix("--rw=") {
                rw_paths.extend(paths.split(',').map(|p| p.trim().to_string()));
            }
            if let Some(paths) = arg.strip_prefix("--ro=") {
                ro_paths.extend(paths.split(',').map(|p| p.trim().to_string()));
            }
            if let Some(paths) = arg.strip_prefix("--rox=") {
                rox_paths.extend(paths.split(',').map(|p| p.trim().to_string()));
            }
        }

        SandboxConfig {
            enabled,
            allow_net,
            rw_paths,
            ro_paths,
            rox_paths,
        }
    }

    pub fn build_prefix(&self) -> Vec<String> {
        if !self.enabled {
            return Vec::new();
        }

        let mut cmd = Vec::new();

        if cfg!(target_os = "macos") {
            cmd.push("sandbox-exec".to_string());
            let mut sbpl = String::from("(version 1) (allow default)");
            if !self.allow_net {
                sbpl.push_str(" (deny network*)");
            }
            if self.rw_paths.is_empty() {
                sbpl.push_str(" (deny file-write*)");
            }
            cmd.push("-p".to_string());
            cmd.push(sbpl);
        } else {
            // Linux landrun (Landlock LSM)
            cmd.push("landrun".to_string());

            if let Ok(override_opts) = env::var("SUPER_COMMA_LANDRUN_OVERRIDE") {
                cmd.extend(parse_nflags_str(&override_opts));
            } else {
                cmd.push("--add-exec".to_string());
                cmd.push("--rox".to_string());
                cmd.push("/nix/store".to_string());
                cmd.push("--ro".to_string());
                cmd.push("/etc".to_string());

                // Dev devices (essential for TTY, /dev/null, stdio)
                for dev in [
                    "/dev/null",
                    "/dev/zero",
                    "/dev/full",
                    "/dev/tty",
                    "/dev/pts",
                ] {
                    if std::path::Path::new(dev).exists() {
                        cmd.push("--rw".to_string());
                        cmd.push(dev.to_string());
                    }
                }

                // Preserve essential environment variables
                let env_vars = [
                    "PATH",
                    "HOME",
                    "USER",
                    "SHELL",
                    "TERM",
                    "COLORTERM",
                    "LANG",
                    "XDG_CONFIG_HOME",
                    "XDG_DATA_HOME",
                    "XDG_RUNTIME_DIR",
                ];
                for var in env_vars {
                    if std::env::var_os(var).is_some() {
                        cmd.push("--env".to_string());
                        cmd.push(var.to_string());
                    }
                }
            }

            if self.allow_net {
                cmd.push("--unrestricted-network".to_string());
            }
            for path in &self.rw_paths {
                cmd.push("--rw".to_string());
                cmd.push(path.clone());
            }
            for path in &self.ro_paths {
                cmd.push("--ro".to_string());
                cmd.push(path.clone());
            }
            for path in &self.rox_paths {
                cmd.push("--rox".to_string());
                cmd.push(path.clone());
            }

            // Additive custom options from SUPER_COMMA_LANDRUN_FLAGS (or legacy SUPER_COMMA_LANDRUN_OPTIONS)
            if let Ok(extra_flags) = env::var("SUPER_COMMA_LANDRUN_FLAGS")
                .or_else(|_| env::var("SUPER_COMMA_LANDRUN_OPTIONS"))
            {
                cmd.extend(parse_nflags_str(&extra_flags));
            }
        }

        cmd
    }
}
