{
  flake-file.inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crate2nix = {
      url = "github:nix-community/crate2nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-compat.follows = "";
      inputs.flake-parts.follows = "flake-parts";
      inputs.cachix.follows = "";
    };
  };

  perSystem =
    {
      pkgs,
      inputs',
      ...
    }:
    let
      cargoNix = import ../Cargo.nix;

      cargoWorkspace = pkgs.callPackage cargoNix {
        buildRustCrateForPkgs =
          pkgs:
          with pkgs;
          buildRustCrate.override {
            rustc = inputs'.fenix.packages.default.toolchain;
            cargo = inputs'.fenix.packages.default.toolchain;
            defaultCrateOverrides = pkgs.defaultCrateOverrides // {
              javascriptcore6-sys = _attrs: {
                nativeBuildInputs = [ pkgs.pkg-config ];
                buildInputs = [ pkgs.webkitgtk_6_0 ];
                propagatedBuildInputs = [ pkgs.webkitgtk_6_0 ];
              };
              webkit6-sys = _attrs: {
                nativeBuildInputs = [ pkgs.pkg-config ];
                buildInputs = [ pkgs.webkitgtk_6_0 ];
              };
              gtk4-layer-shell-sys = _attrs: {
                nativeBuildInputs = [ pkgs.pkg-config ];
                buildInputs = [ pkgs.gtk4-layer-shell ];
              };
              gtk4-session-lock-sys = _attrs: {
                nativeBuildInputs = [ pkgs.pkg-config ];
                buildInputs = [ pkgs.gtk4-layer-shell ];
              };
              yawsf = attrs: {
                nativeBuildInputs = (attrs.nativeBuildInputs or [ ]) ++ [
                  pkgs.autoPatchelfHook
                  pkgs.wrapGAppsHook4
                ];

                buildInputs = (attrs.buildInputs or [ ]) ++ [
                  pkgs.glib-networking
                  pkgs.gtk4-layer-shell
                  pkgs.gst_all_1.gstreamer
                  pkgs.gst_all_1.gst-plugins-base
                  pkgs.gst_all_1.gst-plugins-good
                  pkgs.gst_all_1.gst-plugins-bad
                  pkgs.gst_all_1.gst-plugins-ugly
                  pkgs.gst_all_1.gst-libav
                ];
              };
            };
          };
      };

      yawsfBuild = cargoWorkspace.rootCrate.build;
      exportOpenapiBuild = yawsfBuild.override {
        features = [
          "default"
          "export-openapi"
        ];
      };
    in
    {
      make-shells.default = {
        packages = [
          inputs'.fenix.packages.default.toolchain
          pkgs.rust-analyzer
          pkgs.cargo-expand
          inputs'.crate2nix.packages.default

          pkgs.pkg-config
          pkgs.gtk4
          pkgs.webkitgtk_6_0
          pkgs.gtk4-layer-shell
        ];
      };

      apps.export-openapi = {
        type = "app";
        program = "${exportOpenapiBuild}/bin/export-openapi";
      };

      packages.default = yawsfBuild;

      treefmt = {
        programs.rustfmt = {
          enable = true;
          package = inputs'.fenix.packages.default.rustfmt;
        };
        settings.global.excludes = [
          "Cargo.nix"
        ];
      };
    };
}
