{ ... }: with builtins; {
    imports =
      map (module: ./. + "/${module}")
        (filter (x: x != "default.nix")
          (attrNames (readDir ./.)));
}