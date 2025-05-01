{ config, lib, ... }:
  let
    cfg = config.within;
  in {
    options.within = {
      network = {
        hostname = lib.mkOption {
          default = "nixos";
          example = "metaverse";
          type = lib.types.strMatching "^$|^[[:alnum:]]([[:alnum:]_-]{0,61}[[:alnum:]])?$";
          description = ''
            The hostname of the system.
          '';
        };

        networkmanager = lib.mkOption {
          default = true;
          example = false;
          type = lib.types.bool;
          description = ''
            Enable NetworkManager.
          '';
        };
      };
    };

    config = lib.mkMerge [
      {
        # Hostname
        networking.hostName = lib.mkDefault cfg.network.hostname;
      } (lib.mkIf cfg.network.networkmanager {
        # Network Manager
        networking.useDHCP = lib.mkDefault false;
        networking.networkmanager.enable = lib.mkDefault true;
        programs.nm-applet.enable = lib.mkDefault true;
      })
    ];
  }