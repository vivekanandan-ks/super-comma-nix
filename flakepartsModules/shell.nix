{
  perSystem = { self', pkgs, ... }: {
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
