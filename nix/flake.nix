{
  description = "Nix dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.nightly.latest.complete;
        commonNativeBuildInputs = [ pkgs.pkg-config ];
        commonBuildInputs = [ pkgs.openssl ];
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "rewind-backend";
          src = ./..;
          cargoLock = { lockFile = ../Cargo.lock; };
          nativeBuildInputs = commonNativeBuildInputs;
          buildInputs = commonBuildInputs;
        };
        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = commonNativeBuildInputs;
            buildInputs = with pkgs;
              [ postgresql sqlx-cli sops yq ] ++ [ rust ] ++ commonBuildInputs;
            shellHook = ''
              export PATH=$PATH:$(pwd)/nix/shell

              if sops --config ./nix/.sops.yaml --decrypt ./nix/secrets.yaml >/dev/null 2>&1; then
                export DATABASE_URL=$(sops --config ./nix/.sops.yaml --decrypt ./nix/secrets.yaml | yq -r '.["db"]')
                echo "Loaded DATABASE_URL from SOPS"
              else
                echo "Could not decrypt secrets (GPG locked?)"
              fi

              echo "
              🐚 Rust dev shell ready!
              Run: cargo build / cargo test / etc.
              Available commands:
              - load_db_migration.sh
            '';
          };
        };
      });
}

