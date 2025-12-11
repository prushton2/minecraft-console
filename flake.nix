{
  description = "mc console";

  inputs = {
    nixpkgs.url = "https://github.com/NixOS/nixpkgs/archive/refs/tags/25.11.tar.gz";
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        name = "mc console";
        packages = with pkgs; [
          cargo
          rustc
          openssl
          pkg-config
          rustup
          gnumake
        ];
      };
    }; /* rustup component add rust-src */
}

