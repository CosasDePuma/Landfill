{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs { inherit system; config.allowUnfree = true; };
        in with pkgs; {
          devShells.default = mkShell {
            buildInputs = [ ansible just nomad python3 yamllint ];
            shellHook = ''
              export PS1="╠ IAC ╣ $PS1"
              export NOMAD_ADDR="http://192.168.1.2:4646"

              # Aliases
              alias j="just"
            '';
          };
        }
      );
}
