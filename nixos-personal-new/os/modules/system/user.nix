{ config, lib, ... }:
  let
    cfg = config.within;
  in {
    options.within = {
      user = {
        name = lib.mkOption {
          default = "user";
          example = "god";
          type = lib.types.singleLineStr;
          description = ''
            The name of the user.
          '';
        };
      };
    };

    config = {
      # User
      users.mutableUsers = lib.mkDefault true;
      users.users."${cfg.user.name}" = {
        isNormalUser = lib.mkDefault true;
        description = lib.mkDefault "Main user";
        useDefaultShell = lib.mkDefault true;
        createHome = lib.mkDefault true;
        home = lib.mkDefault "/home/${cfg.user.name}";
        group = lib.mkDefault "users";
        extraGroups = lib.mkDefault [ "docker" "networkmanager" "wheel" ];
        isSystemUser = lib.mkDefault false;
        initialPassword = lib.mkDefault "Changeme123!";
      };
      services.displayManager.autoLogin.user = "${cfg.user.name}";
    };
  }