{
  description = "Pumita's flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    home-manager = { url = "github:nix-community/home-manager"; inputs.nixpkgs.follows = "nixpkgs"; };

    # -----------------------
    #  Third-party modules
    # -----------------------
    
    # MineGRUB is a Minecraft GRUB theme.
    minegrub-theme = { url = "github:Lxtharia/minegrub-theme"; inputs.nixpkgs.follows = "nixpkgs"; };
  };

  outputs = { nixpkgs, ... } @ inputs:
    let
      extraModules = [
        inputs.home-manager.nixosModules.default
        inputs.minegrub-theme.nixosModules.default
      ];
      os' = import ./os { inherit nixpkgs extraModules; };
  in {
    inherit (os') nixosConfigurations;
  };
}
