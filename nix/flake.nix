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
              [ postgresql openssl pkg-config ] ++ [ rust ];
            shellHook = ''
              export PATH=$PATH:$(pwd)/nix/shell
              echo "
              🐚 Rust dev shell ready!
              Run: cargo build / cargo test / etc.
              Available commands:
              - reload_db_struct.sh
              - run_db.sh"
            '';
          };
        };
      });
}

