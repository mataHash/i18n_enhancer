{
  description = "i18_enhancer";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/26.05";
  };

  outputs = {nixpkgs, ...}: let
    inherit (nixpkgs) lib;
    supportedSystems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    forAllSystems = f: lib.genAttrs supportedSystems (system: f system);
  in {
    devShells = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
          rust-analyzer
          clippy
          rustc
          figlet
          rustfmt
        ];

        shellHook = ''
          figlet "Rust running"
          export PATH="$HOME/.cargo/bin/:$PATH"
          if ! command -v rustlings &> /dev/null; then cargo install rustlings
          fi
        '';
      };
    });
  };
}
