{ pkgs ? import <nixos> { } }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    dioxus-cli
  ];
  shellHook = ''
  '';
}
