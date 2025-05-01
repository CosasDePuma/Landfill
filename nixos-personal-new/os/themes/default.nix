{ config, lib, ... }:
  let
    cfg = config.within;
  in {
    imports = [ ./cinnamo ];

    options.within = {
      theme = lib.mkOption {
        default = null;
        example = "cinnamo";
        type = with lib.types; nullOr (enum ["none" "cinnamo"]);
        description = ''
          Desktop environment to use. Theme and dependencies will be installed.
          Options are: `none`, `cinnamo`.
        '';
      };
    };

    config._private.theme = {
      cinnamo.enable = (cfg.theme == "cinnamo");
    };
  }