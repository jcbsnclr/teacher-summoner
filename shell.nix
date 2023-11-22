{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    tokei
  ];

  RUST_BACKTRACE = 1;
  RUST_LOG = "debug";
}
