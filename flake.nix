{
  description = "super-comma (,) - Fast nix shell & runner wrapper powered by nixpkgs-multiverse";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    devenv.url = "github:cachix/devenv";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devenv.flakeModule
      ];

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem = { config, self', inputs', pkgs, system, ... }: {
        packages = rec {
          comma = pkgs.stdenv.mkDerivation {
            pname = "super-comma";
            version = "1.0.0";
            src = ./.;
            nativeBuildInputs = [ pkgs.rustc pkgs.cargo ];
            buildPhase = ''
              cargo build --release --offline || cargo build --release
            '';
            installPhase = ''
              mkdir -p $out/bin
              cp target/release/super-comma $out/bin/comma
              ln -s comma $out/bin/,
              ln -s comma $out/bin/,s
              ln -s comma $out/bin/,v
            '';
          };
          default = comma;
        };

        apps.default = {
          type = "app";
          program = "${self'.packages.default}/bin/comma";
        };

        devenv.shells.default = {
          imports = [ ./devenv.nix ];
          packages = [ self'.packages.default ];
        };
      };
    };
}
