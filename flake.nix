{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain (_: rust);
      in
      {
        devShells.default = craneLib.devShell {
          RUST_LOG = "debug";
          RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
          DATABASE_URL = "postgresql://admin:admin@localhost:5432/gtfs_db";

          packages = with pkgs; [
            rustfmt
            rust-analyzer-unwrapped
            rustPackages.clippy
            rustup
            cargo-flamegraph
            rust-script
            pkg-config
            openssl
            protobuf_29
            sqlx-cli
            postgresql_17
          ];
        };
      }
    );
}
