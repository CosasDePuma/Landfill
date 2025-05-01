{ config, lib, pkgs, ... }:
  let
    cfg = config._private.theme.cinnamo;
  in {
    options._private.theme.cinnamo.enable = lib.mkEnableOption "cinnamoroll desktop environment";

    config = lib.mkIf cfg.enable {
      environment.variables."EDITOR" = "${pkgs.neovim}/bin/nvim";
      environment.variables."TERMINAL" = "${pkgs.kitty}/bin/kitty";
      within.desktop.awesomevm.enable = lib.mkDefault true;
      within.desktop.awesomevm.rc = lib.mkDefault (builtins.readFile ./awesome.rc);
    };
  }