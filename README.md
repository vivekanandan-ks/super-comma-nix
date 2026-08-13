# super-comma (,) - Instant Nix Runner (Rust)

`super-comma` is an ultra-fast, zero-dependency Nix command runner written in **Rust** and powered directly by **[nixpkgs-multiverse](https://github.com/fzakaria/nixpkgs-multiverse)**.

---

## ⚡ Fast Read / Quick Reference

### 1. Core Features & Modes (`,`, `,s`, `,v`)
| Mode | Command | Description | Example |
| :--- | :--- | :--- | :--- |
| **Direct Execution** | `,` | Runs binaries directly via `nix run` (auto-resolves `meta.mainProgram`) | `, ripgrep -i "pattern"` |
| **Interactive Shell** | `,s` | Opens a Nix shell session with multiple packages in `$PATH` | `,s hello cowsay lolcat` |
| **Version Query** | `,v` | Dynamically lists all historical versions of a package for your system | `,v python3` |

---

### 2. Package Definition Syntaxes
Mix and match any of these package formats in `,` or `,s`:

| Spec Type | Syntax | Description | Example |
| :--- | :--- | :--- | :--- |
| **Unpinned** | `pkg` | Uses latest unpinned package from channel | `, hello` |
| **Release Channel Pin** | `26.05=pkg1,pkg2` | Pins packages to a specific NixOS release channel | `,s 26.05=cowsay,lolcat` |
| **Exact Version Pin** | `pkg."version"` | Pins to an exact historical version | `, python3."3.8.9"` |
| **Binary Override** | `pkg:binary` | Overrides the binary executed from a package | `, 26.05=ripgrep:rg -i "pat"` |
| **Custom Flake URI** | `f=uri#attr:bin` | Direct custom Flake URI with optional `#attr` and `:bin` | `, f=github:ksv/repo1#tool --help` |

---

### 3. Project Flags & Purpose
| Flag | Short | Purpose | Example |
| :--- | :--- | :--- | :--- |
| **Output / Dry-Run** | `-o`, `--output` | Manifests and prints the exact `nix` or `nom` command line without executing it | `, -o ripgrep -i "pattern"` |
| **Nix Pass-Through** | `nixflags='...'` | Passes raw flags directly to the underlying `nix` CLI (quote-aware) | `,s nixflags='--impure --refresh' hello` |
| **Nix Output Monitor** | `--nom` | Integrates `nix-output-monitor` for colorful progress bars and tree build logs | `, --nom ripgrep -i "pattern"` |
| **Shell Mode** | `-s` | Flag alternative to `,s` | `, -s hello cowsay` |
| **Version Mode** | `-v` | Flag alternative to `,v` | `, -v python3` |

---

### 4. Environment Variables & Usage
| Variable | Purpose | Example |
| :--- | :--- | :--- |
| `SUPER_COMMA_FLAKE` | Overrides default flake URL (Default: `github:fzakaria/nixpkgs-multiverse`) | `export SUPER_COMMA_FLAKE="github:myorg/multiverse"` |
| `SUPER_COMMA_FLAGS` | Persistent default flags for `super-comma` (e.g. `--nom`, `-o`) | `export SUPER_COMMA_FLAGS="--nom"` |
| `SUPER_COMMA_NIXFLAGS` | Persistent default flags passed to the underlying `nix` CLI | `export SUPER_COMMA_NIXFLAGS="--extra-experimental-features nix-command"` |

---

### 5. Integrations
- **`nix-output-monitor` (`--nom`)**: Automatically pipes build output through `nom shell` for colorful, tree-based progress tracking and download stats:
  ```bash
  , --nom ripgrep -i "pattern"
  ,s --nom hello 26.05=cowsay
  ```

---

### 6. Comprehensive Use Cases & Examples

```bash
# 1. Quick run of latest package with CLI flags
, ripgrep -i "pattern" --color=always

# 2. Open multi-package shell with release channels and exact versions
,s hello 26.05=cowsay,lolcat python3."3.8.9" f=github:ksv/repo1

# 3. Dry-run inspection (-o) with custom nix flags
,s -o nixflags='--impure --refresh' hello 26.05=cowsay

# 4. Colorful build monitoring with nom
, --nom ripgrep -i "pattern"

# 5. Query all historical versions of a package
,v tdesktop
```

---

### How to Use super-comma-nix
Prerequisites: Nix installed in your system

# Want to try without installing?

```bash
nix develop github:vivekanandan-ks/super-comma-nix#try
# and then try the project inside the temporary shell
```

# You can install it in any linux distro, Mac, WSL with this:

```bash
nix profile install github:vivekanandan-ks/super-comma-nix
```

# Home manager and NixOS

Add this in flake.nix:

```nix
inputs = {
  super-comma-nix.url = "github:vivekanandan-ks/super-comma-nix";
};
```
And then add the package `super-comma` according to your setup in NixOS and Home Manager.

---

### Contributing to Development

You can enter the pure Nix dev environment with just 1 command:

```bash
nix develop # Enter pure Nix environment with Rust toolchain & super-comma binaries
```

---

### Implementation & Configuration

- [`src/main.rs`](file:///home/ksvnixospc/Documents/super-comma-nix/src/main.rs) *(Rust runner, zero external dependencies)*
- [`flake.nix`](file:///home/ksvnixospc/Documents/super-comma-nix/flake.nix) *(flake-parts + nix Shell & build package)*
- [`Cargo.toml`](file:///home/ksvnixospc/Documents/super-comma-nix/Cargo.toml)

---

Roadmap:
1) `(Added!)` Support for nix-output-monitor 
2) Isolations for the commands like: network, filesystem etc
This will make trying out new programs worry free like without internet connection, etc etc.
suggest more integrations to improve the experience
---

### Note:
I'm a rust noob, so of course AI did the heavy lifting. The codebase isn't big, so might take short for you to verify I guess (as the project is all about splitting and joiing strings toform the final command). Feel free to suggest features in the issue tracker.

---

### Inspirations and Goals of this project:
Nix shell and nix run commands are plenty helpful already, but they require verbose writing depending on different shells like bash, nu, fish etc. So I got inspired by the comma nix project and thought I'll improve the experience further. The main design is this: simple by default but powerful when needed. So the goals of this project is as simple as making it easy to use nix shell and nix run commands without writing verbose commands. And of course additional functionalities on the way without compromising simplicity as much as possible.
