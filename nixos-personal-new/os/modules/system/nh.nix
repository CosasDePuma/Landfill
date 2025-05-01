{ config, lib, username ? "user", ... }:
  let
    cfg = config.within;
  in {
    options.within ={
      nh = lib.mkOption {
        default = null;
        example = /home/user/.config/nixos;
        type = with lib.types; nullOr path;
        description = ''
          Enable the Nix Helper.
        '';
      };
    };

    config = lib.mkIf (cfg.nh != null) (let
        gcEnabled = builtins.hasAttr "gc" cfg && cfg.gc != null;
      in lib.mkMerge [
        {
          # Nix Helper
          programs.nh.enable = lib.mkDefault true;
          programs.nh.flake = lib.mkDefault cfg.nh;
        } (lib.mkIf gcEnabled {
          # Garbage collector
          nix.gc.automatic = lib.mkForce false;
          programs.nh.clean.enable = lib.mkDefault true;
          programs.nh.clean.dates = lib.mkDefault cfg.gc;
          programs.nh.clean.extraArgs = lib.mkDefault "--keep 5 --keep-since 7d";
        })
      ]);
  }