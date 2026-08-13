{inputs, ...}: {
  perSystem = {
    self',
    pkgs,
    ...
  }: {
    packages = rec {
      super-comma = pkgs.stdenv.mkDerivation {
        pname = "super-comma";
        version = "1.0.0";
        src = inputs.gitignore.lib.gitignoreSource ../.;
        nativeBuildInputs = [pkgs.rustc pkgs.cargo];
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

    apps.default = {
      type = "app";
      program = "${self'.packages.default}/bin/super-comma";
    };
  };
}
