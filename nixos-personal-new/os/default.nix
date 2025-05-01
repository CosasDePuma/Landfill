{ nixpkgs, extraModules ? [], ... }:
  let
    lib' = import ../lib { inherit nixpkgs; };
  in {
    nixosConfigurations = lib'.mkHost {
      inherit extraModules;
      hostname = "metaverse"; username = "architech";
    };
  }