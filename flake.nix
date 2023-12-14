{
  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
  let pkgs = nixpkgs.legacyPackages.x86_64-linux;
      deps = with pkgs; with xorg;
        [ xdotool libX11 libXcursor libXrandr libXinerama pango cairo libXft libXi
        libGL rustup cmake ];
  in
    {
      devShells.x86_64-linux.default = pkgs.mkShell
        {
          buildInputs = deps;
          shellHook = ''export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath deps}";'';
        };
    };
}