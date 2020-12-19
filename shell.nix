let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/4521bc61c2332f41e18664812a808294c8c78580.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  crust = (nixpkgs.latest.rustChannels.nightly.rust.override { extensions = [ "rust-src" ]; });
in
with nixpkgs;
stdenv.mkDerivation {
  name = "catz";
  buildInputs = [ crust ];
  RUST_SRC_PATH = "${crust}/lib/rustlib/src/rust/src";
}
