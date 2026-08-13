{inputs, ...}: {
  imports = [inputs.git-hooks-nix.flakeModule];

  perSystem = {
    pre-commit = {
      check.enable = true; # Adds checks to `nix flake check`

      settings = {
        hooks = {
          # Formatting & Linting Integration
          treefmt.enable = true;
          clippy.enable = true;

          # Security & Secret Scanning
          detect-private-keys.enable = true;
          detect-aws-credentials.enable = true;
          ripsecrets.enable = true;

          # Repository Integrity & File Hygiene
          check-toml.enable = true;
          check-case-conflicts.enable = true;
          check-merge-conflicts.enable = true;
          check-symlinks.enable = true;
          check-added-large-files.enable = true;
          forbid-new-submodules.enable = true;
          end-of-file-fixer.enable = true;
          trim-trailing-whitespace.enable = true;
        };
      };
    };
  };
}
