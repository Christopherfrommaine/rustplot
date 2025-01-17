{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  pure = true;

  buildInputs = [
    pkgs.rustup
    pkgs.ffmpeg
    pkgs.python3
    pkgs.python3Packages.matplotlib
  ];
}
