{ mkShell, pkgs }:
mkShell { packages = with pkgs; [ cargo rustc rust-analyzer rustfmt transcrypt ]; }