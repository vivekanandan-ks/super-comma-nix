{inputs, ...}: {
  imports = [inputs.treefmt-nix.flakeModule];

  perSystem = {
    treefmt = {
      projectRootFile = "flake.nix";
      flakeFormatter = true;

      programs = {
        alejandra.enable = true; # Nix formatting
        rustfmt.enable = true; # Rust formatting
        taplo.enable = true; # TOML formatting (Cargo.toml)
        prettier.enable = true; # Markdown formatting (README.md)
      };

      settings.global.excludes = [
        "flake.lock"
        "target/*"
      ];
    };
  };
}
