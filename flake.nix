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

      rust-toolchain = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
      craneLib = (crane.mkLib pkgs).overrideToolchain rust-toolchain;

      buildInputs = [
        rust-toolchain
      ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
        # Additional darwin specific inputs can be set here
        pkgs.libiconv
      ];

      devDependencies = with pkgs; [
        nil
        just
        cargo-nextest
      ];
      src = craneLib.cleanCargoSource ./.;

      crate = craneLib.buildPackage {
        inherit src buildInputs;
  
        pname = "store-cli";
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
        };
      };

      # run with `nix shell`
      packages.default = crate;

      apps.default = flake-utils.lib.mkApp {
        drv = crate;
      };
    });

    
}
