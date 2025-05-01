{ config, lib, ... }:
  let
    cfg = config.within;
  in {
    options.within = {
      sound = lib.mkOption {
        default = "none";
        example = "pipewire";
        type = with lib.types; nullOr (enum [ "none" "pulseaudio" "pipewire" ]);
        description = ''
          Sound system to use. Options are: `none`, `pulseaudio`, `pipewire`.
        '';
      };
    };

    config = let
      disabled = cfg.sound == null || cfg.sound == "none";
    in {
      hardware.pulseaudio.enable = lib.mkDefault (cfg.sound == "pulseaudio");

      services.pipewire = lib.mkIf (cfg.sound == "pipewire") {
        enable = lib.mkDefault true;
        alsa.enable = lib.mkDefault true;
        alsa.support32Bit = lib.mkDefault true;
        pulse.enable = lib.mkDefault true;
        jack.enable = lib.mkDefault true;
        wireplumber.enable = lib.mkDefault true;
      };

      security.rtkit.enable = lib.mkDefault (!disabled);
    };
}