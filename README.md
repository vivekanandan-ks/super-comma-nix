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
cd /home/ksvnixospc/Documents/super-comma-nix

# Option 1: devenv shell
devenv shell

# Option 2: nix develop
nix develop
```

---

### Implementation & Configuration

- [`src/main.rs`](file:///home/ksvnixospc/Documents/super-comma-nix/src/main.rs) *(Rust runner, zero external dependencies)*
- [`devenv.nix`](file:///home/ksvnixospc/Documents/super-comma-nix/devenv.nix) *(devenv module configuration)*
- [`flake.nix`](file:///home/ksvnixospc/Documents/super-comma-nix/flake.nix) *(flake-parts + devenv.flakeModule)*
- [`Cargo.toml`](file:///home/ksvnixospc/Documents/super-comma-nix/Cargo.toml)
