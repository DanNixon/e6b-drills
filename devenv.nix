{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  packages = [
    pkgs.rustup
    pkgs.treefmt
    pkgs.alejandra
  ];
}
