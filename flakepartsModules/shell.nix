{
  perSystem = {
    config,
    self',
    pkgs,
    lib,
    ...
  }: {
    devShells = {
      default = pkgs.mkShell {
        packages = with pkgs;
          [
            rustc
            cargo
            clippy
            rustfmt
            rust-analyzer
            nix-output-monitor
            self'.packages.default
          ]
          ++ lib.optional (!pkgs.stdenv.isDarwin) pkgs.landrun;
        shellHook = config.pre-commit.installationScript;
      };
      try = pkgs.mkShell {
        packages = with pkgs;
          [
            nix-output-monitor
            self'.packages.default
          ]
          ++ lib.optional (!pkgs.stdenv.isDarwin) pkgs.landrun;
      };
    };
  };
}
