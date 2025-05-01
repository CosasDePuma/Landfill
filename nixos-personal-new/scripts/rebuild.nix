{ pkgs, ... }:
  pkgs.writeShellScriptBin "rebuild" ''
    # Set the flake path
    test -n "$FLAKE" || FLAKE="/etc/nixos"
    # Check the system configuration
    CONFIG="$(${pkgs.nettools}/bin/hostname)-$(${pkgs.coreutils-full}/bin/uname -m)"
    # Check which update method is available
    REBUILD="${pkgs.sudo}/bin/sudo ${pkgs.nixos-rebuild}/bin/nixos-rebuild switch --flake $FLAKE#$CONFIG $@"
    command -v nh >/dev/null 2>&1 && REBUILD="${pkgs.nh}/bin/nh os switch -H $CONFIG -- $@"
    # Check if flake is a git repository, add changes, switch to the new system, and restore changes
    ISGIT="$(${pkgs.git}/bin/git -C "$FLAKE" rev-parse --is-inside-work-tree >/dev/null 2>&1 ; echo "$?")"
    ${pkgs.coreutils-full}/bin/test "$ISGIT" -eq 0 && ${pkgs.git}/bin/git -C "$FLAKE" add "$FLAKE"
    eval "$REBUILD"
    ${pkgs.coreutils-full}/bin/test "$ISGIT" -eq 0 && ${pkgs.git}/bin/git -C "$FLAKE" restore "$FLAKE"
  ''