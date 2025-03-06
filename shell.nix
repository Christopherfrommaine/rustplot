{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup
    pkgs.ffmpeg
    pkgs.python3
    pkgs.python3Packages.matplotlib
    pkgs.imagemagick
  ];

  shellHook = ''
    export RUST_BACKTRACE=1
    rustup toolchain install stable
    rustup default stable
  '';
}
