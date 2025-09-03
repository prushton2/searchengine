{
  description = "search engine";

  inputs = {
    nixpkgs.url = "https://github.com/NixOS/nixpkgs/archive/refs/tags/25.05.tar.gz";
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        name = "search engine";
        packages = with pkgs; [
          cargo
          rustc
          openssl
          pkg-config
        ];
      };
    };
}