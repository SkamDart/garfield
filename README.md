# garfield

Your guide to functional programming using CATegory theory concepts in Rust.

## Dev Enviornment
This uses [nix](https://github.com/NixOS/nix), [nixpkgs](https://github.com/NixOS/nixpkgs), and [mozilla-rust-overlay](https://github.com/mozilla/nixpkgs-mozilla/blob/master/rust-overlay.nix) to manage Rust and it's dependencies. Though, this will run on a nightly rust that has the generic associated traits feature enabled.

## Test
```
$ nix-shell --run "cargo test"
```
