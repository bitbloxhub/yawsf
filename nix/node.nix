{
  perSystem =
    {
      pkgs,
      ...
    }:
    {
      make-shells.default = {
        packages = [
          pkgs.nodejs_latest
          pkgs.pnpm_11
        ];

        shellHook = ''
          export PATH=$PATH:$(pwd)/example-shell/node_modules/.bin/
        '';
      };
    };
}
