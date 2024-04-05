{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};

      in rec {
        devShells.default =
          pkgs.mkShell {
            buildInputs = with pkgs; [ cargo rustc
              openssl
              pkg-config
              libsForQt5.qt5.qtbase
              libsForQt5.qt5.qtsvg

              slint-lsp
            ];
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
              libGL
              libxkbcommon
              fontconfig
              wayland
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
              libsForQt5.qt5.qtbase
              libsForQt5.qt5.qtsvg
            ]);
          };
      }
    );
}
