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

### Development Shell (`devenv` & `nix develop`)

You can enter the dev environment using **either `devenv` or `nix develop`**:

```bash
git clone 
# enter the repo directory
cd super-comma-nix

# Option 1: devenv shell
devenv shell

# Option 2: nix develop
nix develop
```

---

### How to Use super-comma-nix
Prerequisites: Nix installed in ur system

# U can install it in any linux distro, Mac, WSL with this:

```bash
nix profile install github:vivekanandan-ks/super-comma-nix
```

# Want to try without installing?

```bash
nix develop github:vivekanandan-ks/super-comma-nix
```
# Home manager and NixOS

Add this in flake.nix:

```nix
inputs = {
  super-comma-nix.url = "github:vivekanandan-ks/super-comma-nix";
};
```
And then add the package `super-comma-nix` according to ur setup in nix os and home-manager , etc.

---

# Customize:
U can customize the default flake used by this project by passing a custom flake URL to the environment variable `SUPER_COMMA_FLAKE`.
The default value is: `github:fzakaria/nixpkgs-multiverse`


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
I'm a rust noob, so of course AI did the heavy lifting, but I made sure I read the code fully for any screwups by AI. The whole project is just 100 lines of rust code(less than this readme line you are reading so far), so might take few minutes to verify I guess. But where I might be lacking? here are few:
1) Syntactic sugars
2) Better inbuilt libraries
etc etc
I made sure the project starts small with very small codebase to verify even if it's AI generated and verified. So please feel free to suggest features in the issue tracker. As I learn more rust on the way, i'll continuously improve the project as well.


---


### Inspirations and Goals of this project:
Nix shell and nix run commands are plenty helpful already, but they require verbose writing depending on different shells like bash, nu, fish etc. So I got inspired by the comma nix project and thought I'll improve the experience further. The main design is this: simple by default but powerful when needed. So the goals of this project is as simple as making it easy to use nix shell and nix run commands without writing verbose commands. And of course additional functionalities on the way without compromising simplicity as much as possible.
