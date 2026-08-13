# super-comma (,) - Instant Nix Runner (Rust)

`super-comma` is an ultra-fast, zero-dependency Nix command runner written in **Rust** and powered directly by **[nixpkgs-multiverse](https://github.com/fzakaria/nixpkgs-multiverse)**.

---

### All-in-One Usage Examples

#### 1. Interactive Shell Mode (`,s`):
Combine unpinned packages, release-channel pins, exact versions, and custom Flake URIs in a **single command**:

```bash
,s hello 26.05=cowsay,lolcat python3."3.8.9" f=github:ksv/repo1,gitlab:ksv/repo#pack1
```

**What this loads into your shell `$PATH`**:
- `latest.hello` (Unpinned package)
- `26.05.cowsay` & `26.05.lolcat` (Release channel pinned)
- `versions.python3."3.8.9"` (Exact version pinned)
- `github:ksv/repo1` & `gitlab:ksv/repo#pack1` (Direct custom Flake URIs)

---

#### 2. Direct Execution Mode (`,`):
Runs binaries directly using `nix run` (automatically resolves `meta.mainProgram` like `rg` for `ripgrep`):

```bash
# Execute latest package with CLI flags
, ripgrep -i "pattern" --color=always

# Execute specific version
, python3."3.8.9" --version

# Execute pinned release with binary override
, 26.05=ripgrep:rg -i "pattern"
# if a program have more than one binary u can mention it like :<binary_name> after the  package name
# like in the above example

# Execute direct custom Flake URI
, f=github:ksv/repo1#tool --help
```

---

#### 3. Version Query Mode (`,v`):
List all available historical versions of any package dynamically for your machine's architecture:

```bash
,v python3
```

---

### Contributing to the Development:
Setup guide? Not required!

It's nix era, we dont do that here :-)

You can enter the dev environment with just 1 command:

```bash
devenv shell #  run this command inside the cloned repo

# or

nix develop --impure # impure since I integrated devenv in flake and it requires env like PWD etc
```

---

### How to Use super-comma-nix
Prerequisites: Nix installed in ur system

# Want to try without installing?

```bash
nix develop github:vivekanandan-ks/super-comma-nix#try
# and then try the project inside the temporary shell
```

# U can install it in any linux distro, Mac, WSL with this:

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
And then add the package `super-comma` according to ur setup in nixos and home-manager , etc.

---

# Customize:
- **Default Flake URI**: Customize the default flake by passing a custom flake URL to `SUPER_COMMA_FLAKE`. (Default: `github:fzakaria/nixpkgs-multiverse`).
- **Nix Flags (CLI)**: Pass flags directly to `nix shell` or `nix run` via `nixflags='...'` (single or repeated):
  ```bash
  # Inline flags for nix shell / nix run
  ,s nixflags='--impure --refresh' hello 26.05=cowsay

  # Complex options with substituters or extra arguments
  ,s nixflags='--option substituters "https://cache.nixos.org https://mycache.org"' hello

  # Multiple nixflags parameters
  ,s nixflags='--impure' nixflags='--extra-substituters https://cache.org' hello
  ```
- **Nix Flags (Environment Variable)**: Set persistent default flags across all runs via `SUPER_COMMA_NIXFLAGS`:
  ```bash
  export SUPER_COMMA_NIXFLAGS="--extra-experimental-features nix-command"
  ```
- **Output Command Mode (`-o` / `--output`)**: Inspect the exact `nix` command line built by `super-comma` without executing it:
  ```bash
  , -o ripgrep -i "pattern"
  # Outputs: nix run github:fzakaria/nixpkgs-multiverse#latest.ripgrep -- -i pattern

  ,s -o nixflags='--impure' hello 26.05=cowsay
  # Outputs: nix shell --impure github:fzakaria/nixpkgs-multiverse#latest.hello github:fzakaria/nixpkgs-multiverse#26.05.cowsay
  ```



---

### Implementation & Configuration

- [`src/main.rs`](file:///home/ksvnixospc/Documents/super-comma-nix/src/main.rs) *(Rust runner, zero external dependencies)*
- [`devenv.nix`](file:///home/ksvnixospc/Documents/super-comma-nix/devenv.nix) *(devenv module configuration)*
- [`flake.nix`](file:///home/ksvnixospc/Documents/super-comma-nix/flake.nix) *(flake-parts + devenv.flakeModule)*
- [`Cargo.toml`](file:///home/ksvnixospc/Documents/super-comma-nix/Cargo.toml)

---

Roadmap:
1) Support for nix-output-monitor
2) Isolations for the commands like: network, filesystem etc
This will make trying out new programs worry free like without internet connection, etc etc
Of course would have to integrate more cool projects from the nix ecosystem for ultra experience.

---

### Note:
I'm a rust noob, so of course AI did the heavy lifting, but I made sure I read the code fully for any screwups by AI. The codebase isnt big, so might take short for u to verify I guess. But where I might be lacking? here are few:
1) Syntactic sugars
2) Better inbuilt libraries usage
etc etc
I made sure the project starts small with very small codebase to verify even if it's AI generated and verified. So please feel free to suggest features in the issue tracker. As I learn rust on the way, i'll continuously improve the project as well.

---


### Inspirations and Goals of this project:
Nix shell and nix run commands are plenty helpful already, but they require verbose writing depending on different shells like bash, nu, fish etc. So I got inspired by the comma nix project and thought I'll improve the experience further. The main design is this: simple by default but powerful when needed. So the goals of this project is as simple as making it easy to use nix shell and nix run commands without writing verbose commands. And of course additional functionalities on the way without compromising simplicity as much as possible.
