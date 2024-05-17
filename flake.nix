{
  description = "Facildata";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane}:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          rust-overlay.overlays.default
        ];
      };

      inherit (pkgs) lib;

      rust-toolchain = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
      craneLib = (crane.mkLib pkgs).overrideToolchain rust-toolchain;

      buildInputs = [
        rust-toolchain
        pkgs.mold
      ];

      devDependencies = with pkgs; [
        nil
        just
      ];
      src = craneLib.cleanCargoSource ./.;

      crate = craneLib.buildPackage {
        inherit src;
        strictDeps = true;
        nativeBuildInputs = buildInputs;
        doCheck = false;
      };
    in
    {
      checks = {
        inherit crate;
      };
      # run with `nix develop`
      devShells = {
        default = with pkgs; mkShell {
          buildInputs = [ buildInputs devDependencies ];
          shellHook = ''
            export MOLD_PATH=${mold}/bin/mold
          '';
        };
      };

      # run with `nix shell`
      packages.default = crate;
    });

    
}
