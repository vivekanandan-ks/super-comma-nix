{ pkgs, config, inputs, ... }:

{
  # devenv configuration for super-comma
  packages = [
    pkgs.cargo
    pkgs.rustc
  ];

  languages.rust.enable = true;

  scripts.",".exec = ''cargo run -- "$@"'';
  scripts.",s".exec = ''cargo run -- -s "$@"'';
  scripts.",v".exec = ''cargo run -- -v "$@"'';

  enterShell = ''
    echo -e "\033[1;36mWelcome to super-comma (,) DevShell!\033[0m"
    echo -e "Commands available:"
    echo -e "  \033[1;32m, <pkg> [args...]\033[0m         Run package binary directly (uses nix run)"
    echo -e "  \033[1;32m,s <pkg1> [pkg2...]\033[0m      Open interactive nix shell with packages"
    echo -e "  \033[1;32m,v <pkg>\033[0m               List all historical versions of package"
  '';
}
