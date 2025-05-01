{ config, lib, ... }:
  let
    cfg = config.within;
  in {
    options.within = {
      languages = lib.mkOption {
        default = "us";
        example = ["es" "us"];
        type = with lib.types; let languages = enum ["es" "us"]; in either languages (nonEmptyListOf languages);
        description = ''
          The default language of the system, the locale and the keyboard layout.
          More than one language can be set, but the first one will be the default.
          Available options are: `es`, `us`.
        '';
      };

      timezone = lib.mkOption {
        default = null;
        example = "Europe/Madrid";
        type = with lib.types; nullOr singleLineStr;
        description = ''
          The timezone of the system.
          If null, the default is `UTC` and can be set imperatively using `timedatectl set-timezone`.
        '';
      };
    };

    config = let
        primary_language = if builtins.isList cfg.languages then builtins.head cfg.languages else cfg.languages;
        locale = if primary_language == "es" then "es_ES.UTF-8" else "en_US.UTF-8";
      in {
        # Time
        time.timeZone = cfg.timezone;

        # Language
        i18n.defaultLocale = lib.mkDefault locale;
        i18n.extraLocaleSettings = {
          LANGUAGE = lib.mkDefault locale;
          LC_ALL = lib.mkDefault locale;
          LC_ADDRESS = lib.mkDefault locale;
          LC_IDENTIFICATION = lib.mkDefault locale;
          LC_MEASUREMENT = lib.mkDefault locale;
          LC_MONETARY = lib.mkDefault locale;
          LC_NAME = lib.mkDefault locale;
          LC_NUMERIC = lib.mkDefault locale;
          LC_PAPER = lib.mkDefault locale;
          LC_TELEPHONE = lib.mkDefault locale;
          LC_TIME = lib.mkDefault locale;
        };

        # Keyboard
        services.xserver.xkb.layout = lib.mkDefault (if builtins.isList cfg.languages then builtins.concatStringsSep "," cfg.languages else cfg.languages);
        services.xserver.xkb.variant = lib.mkDefault ",";
        services.xserver.xkb.options = lib.mkDefault "grp:alt_shift_toggle";
      };
  }