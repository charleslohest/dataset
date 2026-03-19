{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      overlays = [ (import rust-overlay) ];
      system = "x86_64-linux";
  	  pkgs = import nixpkgs {inherit system overlays;};
		  python = pkgs.python310.withPackages (pyPkgs: with pyPkgs; [
		  ]);
      rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
        extensions = [ "rust-src" "rust-analyzer" "rustfmt" ];
      });
    in
      {
        devShells.${system}.default = pkgs.mkShell {
          packages = with pkgs; [
            gcc
            glibc

	          llvmPackages.clang-tools
            gdb
            bear
            gnumake	
            

            rust

          ];
        };
      };
}
