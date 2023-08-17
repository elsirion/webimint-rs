{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fedimint.url = "github:fedimint/fedimint";
  };

  outputs = { self, nixpkgs, flake-utils, fedimint, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
        target = "wasm32-unknown-unknown";
      in
    {
      devShells.default = pkgs.mkShell {
        inputsFrom = [ fedimint.devShells."${system}".crossWasm ];
        nativeBuildInputs = with pkgs; [ 
          nodejs
          trunk
          nodePackages.tailwindcss 
        ];
      };
    });
}
