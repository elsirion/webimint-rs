{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flakebox = {
      url = "github:rustshop/flakebox";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flakebox, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
        flakeboxLib = flakebox.lib.${system} {
          config = {
            # Avoid auto generation of flakebox' ci workflows
            # @see https://github.com/rustshop/flakebox/blob/master/docs/nixos-options.md#configgithubciworkflows
            github.ci.workflows = { };
            # don't check newlines
            # @see https://github.com/rustshop/flakebox/blob/master/docs/nixos-options.md#configgitpre-committrailing_newline
            git.pre-commit.trailing_newline = false;
            # disable semgrep
            # @see https://github.com/rustshop/flakebox/blob/master/docs/nixos-options.md#configsemgrepenable
            semgrep.enable = false;
          };
        };
        rustSrc = flakeboxLib.filterSubPaths {
          root = builtins.path {
            name = "webimint";
            path = ./.;
          };
          paths = [ "Cargo.toml" "Cargo.lock" ".cargo" "src" ];
        };
        toolchainsWasm = (pkgs.lib.getAttrs
          [
            "default"
            "wasm32-unknown"
          ]
          (flakeboxLib.mkStdFenixToolchains { })
        );
        toolchainWasm = flakeboxLib.mkFenixMultiToolchain {
          toolchains = toolchainsWasm;
        };
        target = "wasm32-unknown-unknown";

        outputs = (flakeboxLib.craneMultiBuild { }) (craneLib':
          let
            craneLib = (craneLib'.overrideArgs {
              pname = "flexbox-multibuild";
              src = rustSrc;
            });
          in
          rec {
            workspaceDeps = craneLib.buildWorkspaceDepsOnly { };
            workspaceBuild =
              craneLib.buildWorkspace { cargoArtifacts = workspaceDeps; };
            webimint = craneLib.buildPackage { };
          });
      in
      {
        legacyPackages = outputs;
        devShells = flakeboxLib.mkShells {
          toolchain = toolchainWasm;
          packages = [ ];
          CC_wasm32_unknown_unknown = "${pkgs.llvmPackages_15.clang-unwrapped}/bin/clang-15";
          # -Wno-macro-redefined fixes ring building
          CFLAGS_wasm32_unknown_unknown = "-I ${pkgs.llvmPackages_15.libclang.lib}/lib/clang/15.0.7/include/ -Wno-macro-redefined";
          nativeBuildInputs = with pkgs; [
            trunk
            wasm-pack
            wasm-bindgen-cli
            nodejs
            nodePackages.tailwindcss
          ];
        };
      });
}
