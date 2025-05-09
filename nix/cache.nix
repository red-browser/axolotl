{ pkgs }:
{
  enable = true;
  name = "axolotl-cache";
  key = "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY=";
  
  paths = [
    "/nix/store"
    "/var/lib/cachix"
    "${pkgs.rustToolchain}"
  ];
}
