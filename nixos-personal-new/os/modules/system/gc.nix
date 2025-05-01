{ config, lib, ... }:
  let
    cfg = config.within;
  in {
    options.within = {
      gc = lib.mkOption {
        default = "daily";
        example = "weekly";
        type = with lib.types; nullOr singleLineStr;
        description = ''
          Enable the Garbage Collector.
          The value is the frequency of the garbage collection.
          Set to `null` to disable it.
        '';
      };
    };

    config = lib.mkIf (cfg.gc != null) {
      # Garbage collector
      nix.gc.automatic = lib.mkDefault true;
      nix.gc.persistent = lib.mkDefault true;
      nix.gc.dates = lib.mkDefault cfg.gc;
      nix.gc.options = lib.mkDefault "--delete-older-than 7d";

      # Optimizer
      nix.settings.auto-optimise-store = lib.mkDefault true;
      nix.optimise.automatic = lib.mkDefault true;
      nix.optimise.dates = lib.mkDefault [ cfg.gc ];

      # Clean on boot
      boot.tmp.cleanOnBoot = lib.mkDefault true;
    };
}