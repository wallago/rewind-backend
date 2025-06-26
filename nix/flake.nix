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
      in {
        devShells = {
          default = pkgs.mkShell {
            buildInputs = with pkgs;
              [ postgresql openssl pkg-config sqlx-cli sops yq ] ++ [ rust ];
            shellHook = ''
              export PATH=$PATH:$(pwd)/nix/shell

              if [ -f ./nix/secrets.yaml ]; then
                export DATABASE_URL=$(sops --decrypt ./nix/secrets.yaml | yq -r '.["db"]')
                echo "Loaded secrets from SOPS into environment"
              else
                echo "Secrets not found!"
              fi

              echo "
              🐚 Rust dev shell ready!
              Run: cargo build / cargo test / etc.
              Available commands:
              - load_db.sh
              - run_db.sh"
            '';
          };
        };
      });
}

