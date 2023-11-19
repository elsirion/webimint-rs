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
        CC_wasm32_unknown_unknown = "${pkgs.llvmPackages_15.clang-unwrapped}/bin/clang-15";
        # -Wno-macro-redefined fixes ring building
        CFLAGS_wasm32_unknown_unknown = "-I ${pkgs.llvmPackages_15.libclang.lib}/lib/clang/15.0.7/include/ -Wno-macro-redefined";
        nativeBuildInputs = with pkgs; [ 
          nodejs
          trunk
          nodePackages.tailwindcss 
        ];
      };
    });
}
