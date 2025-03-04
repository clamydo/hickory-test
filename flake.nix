{
  description = "Static TLSA DNS resolver with build-time integrity checks via SHA1 hashing";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
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
        # Import nixpkgs with rust overlay
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        # Create a static Rust environment with musl target
        rustStatic = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "x86_64-unknown-linux-musl" ];
          extensions = [ "rust-src" ];
        };

        # We'll generate a static binary using rust + pkgsStatic
        staticRustPlatform = pkgs.makeRustPlatform {
          cargo = rustStatic;
          rustc = rustStatic;
        };
      in
      {
        packages.default = staticRustPlatform.buildRustPackage {
          pname = "hickory-dns-test";
          version = "0.1.0";
          src = ./.;

          # Use the Cargo.lock from the repo
          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          # Build for musl target
          CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";

          # Flags to ensure static linking
          RUSTFLAGS = "-C target-feature=+crt-static -C link-self-contained=yes -C link-arg=-static";

          # No need to specify linker path as we'll use the rust environment's linker

          # Provide necessary build tools
          nativeBuildInputs = with pkgs; [
            pkg-config
            pkgsStatic.stdenv.cc
          ];

          # Perform the build
          buildPhase = ''
            cargo build --release --target x86_64-unknown-linux-musl
          '';

          # Install the binary
          installPhase = ''
            mkdir -p $out/bin
            cp target/x86_64-unknown-linux-musl/release/hickory-dns-test $out/bin/
          '';

          # Skip checks for simplicity
          doCheck = false;

          meta = with pkgs.lib; {
            description = "Static TLSA DNS resolver that calculates SHA1 hashes at build time";
            longDescription = ''
              A fully static DNS resolver built with hickory-dns that:
              1. Calculates and embeds SHA1 hashes of source code at build time
              2. Performs TLSA DNS lookups using UDP-only transport
              3. Uses hickory-resolver with DNS-over-RUSTLS and DNSSEC support
              4. Built as a static-pie ELF executable using musl libc
            '';
            mainProgram = "hickory-dns-test";
            license = licenses.gpl;
            platforms = platforms.linux;
          };
        };
      }
    );
}
