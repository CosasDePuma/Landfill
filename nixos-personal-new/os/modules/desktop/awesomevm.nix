{ config, lib, pkgs, ... }: let cfg = config.within.desktop.awesomevm; in
  {
    options.within.desktop.awesomevm = lib.mkEnableOption "AwesomeVM";

    config = lib.mkIf cfg {
      # Xorg
      services.xserver.enable = lib.mkDefault true;
      services.libinput.enable = lib.mkDefault true;

      # Display manager
      services.displayManager.defaultSession = lib.mkDefault "none+awesome";

      # AwesomeVM
      services.xserver.windowManager.awesome.enable = lib.mkDefault true;
      services.xserver.windowManager.awesome.luaModules = lib.mkDefault (with pkgs.luaPackages; [ luarocks ]);
    };
  }