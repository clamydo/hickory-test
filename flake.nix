{
  description = "Static musl Rust binary via flake.nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    # Optional: using rust-overlay to get a more recent or custom Rust toolchain.
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        # Import nixpkgs and add the rust overlay.
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
      in
      {
        packages.myRustApp = pkgs.rustPlatform.buildRustPackage {
          pname = "myRustApp";
          version = "0.1.0";

          # The source of your project – adjust if needed.
          src = ./.;

          # Use cargoLock instead of cargoSha256
          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          # Specify the target to use musl.
          target = "x86_64-unknown-linux-musl";

          # Optional: pass extra build flags.
          buildFlags = [ "--target=x86_64-unknown-linux-musl" ];

          # Provide any native build dependencies your crate needs.
          nativeBuildInputs = [ pkgs.pkg-config ];

          meta = with pkgs.lib; {
            description = "A Rust application statically compiled with musl";
            license = licenses.mit;
            maintainers = [ ];
          };
        };

        # Set a default package for the flake.
        defaultPackage = self.packages.${system}.myRustApp;
      }
    );
}

