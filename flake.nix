{
  inputs = {
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs =
    {
      crane,
      flake-utils,
      nixpkgs,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
      in
      {
        packages.default = craneLib.buildTrunkPackage {
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter =
              path: type:
              (pkgs.lib.hasInfix "/public" path)
              || (pkgs.lib.hasSuffix ".html" path)
              || (pkgs.lib.hasSuffix ".css" path)
              || (craneLib.filterCargoSources path type);
          };

          nativeBuildInputs = (
            with pkgs;
            [
              binaryen
              tailwindcss
              trunk
              wasm-bindgen-cli
              wasm-tools
            ]
          );

          pname = "website";
          strictDeps = true;
          trunkIndexPath = "index.html";
          wasm-bindgen-cli = pkgs.wasm-bindgen-cli;
        };

        devShells.default = craneLib.devShell {
          packages = (
            with pkgs;
            [
              leptosfmt
              nodePackages.prettier
              rust-analyzer
            ]
          );
        };
      }
    );
}
