# super-comma (,) - Instant Nix Runner (Rust)

`super-comma` is an ultra-fast, zero-dependency Nix command runner written in **Rust** and powered directly by **[nixpkgs-multiverse](https://github.com/fzakaria/nixpkgs-multiverse)**.

---

## ⚡ Fast Read / Quick Reference

### 1. Core Features & Modes (`,`, `,s`, `,v`)

| Mode                  | Command | Description                                                             | Example                                       |
| :-------------------- | :------ | :---------------------------------------------------------------------- | :-------------------------------------------- |
| **Direct Execution**  | `,`     | Runs binaries directly via `nix run` (auto-resolves `meta.mainProgram`) | `, ripgrep -i "pattern"`                      |
| **Interactive Shell** | `,s`    | Opens a Nix shell with multiple packages (use `-- <cmd>` for execution) | `,s hello cowsay` / `,s hello -- cowsay "hi"` |
| **Version Query**     | `,v`    | Dynamically lists all historical versions of a package for your system  | `,v python3`                                  |

---

### 2. Package Definition Syntaxes

Mix and match any of these package formats in `,` or `,s`:

| Spec Type               | Syntax            | Description                                              | Example                            |
| :---------------------- | :---------------- | :------------------------------------------------------- | :--------------------------------- |
| **Unpinned**            | `pkg`             | Uses latest unpinned package from channel                | `, hello`                          |
| **Release Channel Pin** | `26.05=pkg1,pkg2` | Pins packages to a specific NixOS release channel        | `,s 26.05=cowsay,lolcat`           |
| **Exact Version Pin**   | `pkg."version"`   | Pins to an exact historical version                      | `, python3."3.8.9"`                |
| **Binary Override**     | `pkg:binary`      | Overrides the binary executed from a package             | `, 26.05=ripgrep:rg -i "pat"`      |
| **Custom Flake URI**    | `f=uri#attr:bin`  | Direct custom Flake URI with optional `#attr` and `:bin` | `, f=github:ksv/repo1#tool --help` |

---

### 3. Core Project Flags & Environment Variables

#### Core CLI Flags

| Flag                  | Short            | Purpose                                                                         | Example                                  |
| :-------------------- | :--------------- | :------------------------------------------------------------------------------ | :--------------------------------------- |
| **Output / Dry-Run**  | `-o`, `--output` | Manifests and prints the exact `nix` or `nom` command line without executing it | `, -o ripgrep -i "pattern"`              |
| **Nix Pass-Through**  | `nixflags='...'` | Passes raw flags directly to the underlying `nix` CLI (quote-aware)             | `,s nixflags='--impure --refresh' hello` |
| **Command Delimiter** | `--`             | Delimits package specs from direct command execution in `,s` shell mode         | `,s hello cowsay -- cowsay "hello"`      |
| **Shell Mode**        | `-s`             | Flag alternative to `,s`                                                        | `, -s hello cowsay`                      |
| **Version Mode**      | `-v`             | Flag alternative to `,v`                                                        | `, -v python3`                           |

#### Core Environment Variables

| Variable               | Purpose                                                                      | Example                                                                   |
| :--------------------- | :--------------------------------------------------------------------------- | :------------------------------------------------------------------------ |
| `SUPER_COMMA_FLAKE`    | Overrides default flake URL (Default: `github:fzakaria/nixpkgs-multiverse`)  | `export SUPER_COMMA_FLAKE="github:myorg/multiverse"`                      |
| `SUPER_COMMA_FLAGS`    | Persistent default flags for `super-comma` (e.g. `--nom`, `--sandbox`, `-o`) | `export SUPER_COMMA_FLAGS="--sandbox"`                                    |
| `SUPER_COMMA_NIXFLAGS` | Persistent default flags passed to the underlying `nix` CLI                  | `export SUPER_COMMA_NIXFLAGS="--extra-experimental-features nix-command"` |

### 4. Integrations & Extensions

#### 4.1 `nix-output-monitor` (`--nom`)

| Feature / Flag         | Full Flag Syntax | Purpose                                                                        | Example                        |
| :--------------------- | :--------------- | :----------------------------------------------------------------------------- | :----------------------------- |
| **Nix Output Monitor** | `--nom`          | Integrates `nix-output-monitor` for colorful progress bars and tree build logs | `, --nom ripgrep -i "pattern"` |

> **Note on `--nom`:** Requires `nom` (`nix-output-monitor`) to be available in your system environment `$PATH`.

#### 4.2 Cross-Platform Sandboxing (`--sandbox`, `landrun` / `sandbox-exec`)

##### Sandbox Flags & Features

| Sub-Flag             | Full Flag Syntax          | Purpose                                                               | Example                               |
| :------------------- | :------------------------ | :-------------------------------------------------------------------- | :------------------------------------ |
| **Default Sandbox**  | `--sandbox`               | Enforces default-deny sandbox lockdown (blocks network & home writes) | `, --sandbox ripgrep -i "pattern"`    |
| **Unblock Network**  | `--sandbox --net`         | Unblocks network socket & DNS access within sandbox                   | `, --sandbox --net python3 script.py` |
| **Read-Write Paths** | `--sandbox --rw=<paths>`  | Unblocks write access to specified comma-separated paths              | `, --sandbox --rw=./,/tmp node.js`    |
| **Read-Only Paths**  | `--sandbox --ro=<paths>`  | Unblocks extra read-only paths inside sandbox                         | `, --sandbox --ro=/var/log ripgrep`   |
| **Executable Paths** | `--sandbox --rox=<paths>` | Unblocks extra read-only + executable paths inside sandbox            | `, --sandbox --rox=/opt/bin tool`     |

##### Sandbox Environment Variables

| Variable                       | Purpose                                                                | Example                                                                    |
| :----------------------------- | :--------------------------------------------------------------------- | :------------------------------------------------------------------------- |
| `SUPER_COMMA_LANDRUN_FLAGS`    | **Additive**: Extra custom flags appended on top of `landrun` defaults | `export SUPER_COMMA_LANDRUN_FLAGS="--env DISPLAY --env WAYLAND_DISPLAY"`   |
| `SUPER_COMMA_LANDRUN_OVERRIDE` | **Override**: Completely replaces all default `landrun` flags          | `export SUPER_COMMA_LANDRUN_OVERRIDE="--rox /nix/store --ro /etc --rw ./"` |

> **Note on Linux defaults:** By default, Linux sandboxing (`landrun`) automatically supplies safe defaults (`--add-exec`, `--rox /nix/store`, `--ro /etc`, essential `/dev` devices like `/dev/null`, `/dev/tty`, `/dev/pts`, `/dev/zero`, and core environment variables `$PATH`, `$HOME`, `$USER`, `$SHELL`, `$TERM`, `$LANG`). Use `SUPER_COMMA_LANDRUN_FLAGS` to **add extra flags** on top of these defaults, or `SUPER_COMMA_LANDRUN_OVERRIDE` to **completely replace** them. macOS uses native `/usr/bin/sandbox-exec`.

---

### 5. Comprehensive Use Cases & Examples

```bash
# 1. Quick run of latest package with CLI flags
, ripgrep -i "pattern" --color=always

# 2. Open multi-package shell with release channels and exact versions
,s hello 26.05=cowsay,lolcat python3."3.8.9" f=github:ksv/repo1

# 3. Dry-run inspection (-o) with custom nix flags
,s -o nixflags='--impure --refresh' hello 26.05=cowsay

# 4. Colorful build monitoring with nom
, --nom ripgrep -i "pattern"

# 5. Sandboxed execution with network, storage and more isolation
, --sandbox python3 script.py

# 6. Direct command execution in multi-package shell (with optional sandboxing)
,s --sandbox hello cowsay -- cowsay "Hello from multi-package shell"

# 7. Query all historical versions of a package
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

- [`src/main.rs`](file:///home/ksvnixospc/Documents/super-comma-nix/src/main.rs) _(Rust runner & CLI orchestration)_
- [`src/parser.rs`](file:///home/ksvnixospc/Documents/super-comma-nix/src/parser.rs) _(Quote-aware tokenizer & flag extractors)_
- [`src/resolver.rs`](file:///home/ksvnixospc/Documents/super-comma-nix/src/resolver.rs) _(Package spec & version resolver)_
- [`src/sandbox.rs`](file:///home/ksvnixospc/Documents/super-comma-nix/src/sandbox.rs) _(Cross-platform Landlock/Seatbelt sandbox engine)_
- [`flake.nix`](file:///home/ksvnixospc/Documents/super-comma-nix/flake.nix) _(flake-parts + nix Shell & build package)_
- [`Cargo.toml`](file:///home/ksvnixospc/Documents/super-comma-nix/Cargo.toml)

---

Roadmap:

1. `(Added!)` Support for nix-output-monitor
2. `(Added!)` Isolations for commands: network & filesystem sandboxing (`--sandbox`, `--net`, `--rw=...`)

---

### Note:

I'm a rust ultra noob, so of course AI did the heavy lifting. The codebase isn't big, so might take short for you to verify I guess (as the project is all about splitting and joiing strings toform the final command). Feel free to suggest features in the issue tracker.

---

### Inspirations and Goals of this project:

Nix shell and nix run commands are plenty helpful already, but they require verbose writing depending on different shells like bash, nu, fish etc. So I got inspired by the comma nix project and thought I'll improve the experience further. The main design is this: simple by default but powerful when needed. So the goals of this project is as simple as making it easy to use nix shell and nix run commands without writing verbose commands. And of course additional functionalities on the way without compromising simplicity as much as possible.
