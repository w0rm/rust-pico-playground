{
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs;
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, rust-overlay, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit overlays system; };
        rust-bin = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          targets = [ "thumbv6m-none-eabi" ];
          extensions = [ "rust-src" ];
        });
      in
      {
        devShell = with pkgs; mkShell {
          buildInputs = [
            rust-bin
            rust-analyzer
            rustfmt
            flip-link
            elf2uf2-rs
          ];
        };
      }
    );
}
