{
  # This is a Nix Flake that defines development environment for this project.
  #
  # Using Nix is NOT required to develop this project.
  #
  # Nix provides a declariative way to define reproducible development environment
  # and aims to remove the issue of "works on my machine".
  #
  # If you use Nix, you can install and activate all dependencies required to
  # develop on this project with:
  #
  # $ nix develop
  #
  # If in addition you have `direnv` installed, you can create `.envrc` file with
  # `use flake` content, so that development environment activates automatically
  # when the project root directory is entered.
  #
  # More information:
  # - Nix: https://nixos.org
  # - Nix flakes:  https://wiki.nixos.org/wiki/Flakes
  # - Direnv: https://direnv.net
  description = "The fossil game";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            (rustToolchain.override {
              extensions = [
                "rust-src"
                "rust-analyzer"
              ];
            })
            # Audio (Linux only)
            pkgs.alsa-lib
            # Cross Platform 3D Graphics API
            pkgs.vulkan-loader
            # For debugging around vulkan
            pkgs.vulkan-tools
            # Other dependencies
            pkgs.libudev-zero
            pkgs.pkg-config
            pkgs.wayland
            pkgs.libxkbcommon
          ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.wayland
            pkgs.libxkbcommon
            pkgs.vulkan-loader
          ];
        };
      }
    );
}
