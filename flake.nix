{
  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";
  inputs.crane.url   = "github:ipetkov/crane";

  outputs = { self, nixpkgs, crane }:
  let pkgs = nixpkgs.legacyPackages.x86_64-linux;
      crane' = crane.lib.x86_64-linux;
      deps = with pkgs; with xorg;
        [ xdotool libX11 libXcursor libXrandr libXinerama pango cairo libXft libXi
        libGL cargo rustc cmake ];
  in
    {
      devShells.x86_64-linux.default = pkgs.mkShell
        {
          buildInputs = deps;
          shellHook = ''export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath deps}";'';
        };
    };
}