{
  perSystem = {
    config,
    self',
    pkgs,
    ...
  }: {
    devShells = {
      default = pkgs.mkShell {
        packages = with pkgs; [
          rustc
          cargo
          clippy
          rustfmt
          rust-analyzer
          nix-output-monitor
          self'.packages.default
        ];
        shellHook = config.pre-commit.installationScript;
      };
      try = pkgs.mkShell {
        packages = with pkgs; [
          nix-output-monitor
          self'.packages.default
        ];
      };
    };
  };
}
