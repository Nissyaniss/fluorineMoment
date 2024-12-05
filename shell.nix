{ pkgs ? import <nixos> { } }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    dioxus-cli
    pkg-config
    openssl
  ];
  shellHook = ''
  '';
}
