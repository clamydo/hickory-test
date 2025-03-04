[![built with garnix](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fgarnix.io%2Fapi%2Fbadges%2Fclamydo%2Fhickory-test%3Fbranch%3Dmain)](https://garnix.io/repo/clamydo/hickory-test)
A statically compiled TLSA DNS resolver (for debugging purposes) and a playground for nix related tooling.

## Features

- Fully static binary built with musl libc (runs on any Linux distro)

## Building locally

```bash
# Build with Nix
nix build

# Run the binary
./result/bin/hickory-dns-test
```
