{ config, lib, ... }:
  let
    cfg = config.within;
  in {
    options.within = {
      nix = {
        version = lib.mkOption {
          default = "unstable";
          example = "24.05";
          type = lib.types.str;
          description = ''
            NixOS version to use. Default is `unstable` (rolling release).
          '';
        };

        platform = lib.mkOption {
          default = "x86_64-linux";
          example = "aarch64-linux";
          type = lib.types.enum [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
          description = ''
            Nixpkgs host platform.
          '';
        };
      };
    };

    config = {
      system.stateVersion = lib.mkDefault cfg.nix.version;

      nixpkgs.config.allowUnfree = lib.mkDefault true;
      nixpkgs.hostPlatform = lib.mkDefault cfg.nix.platform;

      nix.enable = lib.mkDefault true;
      nix.channel.enable = lib.mkDefault true;
      nix.settings.allowed-users = lib.mkDefault [ "root" "@wheel" ];
      nix.settings.trusted-users = lib.mkDefault [ "root" "@wheel" ];
      nix.settings.experimental-features = lib.mkDefault [ "nix-command" "flakes" ];
    };
  }