{
  inputs = {
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

  outputs = { self, flake-utils, nixpkgs, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustBin = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };

        build_inputs = with pkgs; [ ];

        native_build_inputs = with pkgs; [
          binaryen
          cargo-auditable
          nodePackages.prettier
          pkg-config
          tailwindcss
          trunk
          wasm-bindgen-cli
        ];

        code = pkgs.callPackage ./. {
          inherit pkgs system build_inputs native_build_inputs;
        };
      in rec {
        packages = code // {
          all = pkgs.symlinkJoin {
            name = "all";
            paths = with code; [ web ];
          };

          default = packages.all;
          override = packages.all;
          overrideDerivation = packages.all;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs;
            [ cargo-watch leptosfmt rust-analyzer rustBin ] ++ build_inputs;
          nativeBuildInputs = native_build_inputs;

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath build_inputs;
        };
      });
}
