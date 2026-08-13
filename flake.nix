{
  description = "super-comma (,) - Fast nix shell & runner wrapper powered by nixpkgs-multiverse";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem = { self', pkgs, ... }: {
        packages = rec {
          super-comma = pkgs.stdenv.mkDerivation {
            pname = "super-comma";
            version = "1.0.0";
            src = inputs.gitignore.lib.gitignoreSource ./.;
            nativeBuildInputs = [ pkgs.rustc pkgs.cargo ];
            buildPhase = ''
              cargo build --release --offline || cargo build --release
            '';
            installPhase = ''
              mkdir -p $out/bin
              cp target/release/super-comma $out/bin/super-comma
              ln -s super-comma $out/bin/,
              ln -s super-comma $out/bin/,s
              ln -s super-comma $out/bin/,v
            '';
          };
          default = super-comma;
          try = default;
        };

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

        apps.default = {
          type = "app";
          program = "${self'.packages.default}/bin/super-comma";
        };
      };
    };
}
