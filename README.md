# Yet Another Wayland Shell Framework

YAWSF is a native Wayland host for desktop shells built with web technologies. It embeds web pages
in GTK/WebKit surfaces and exposes an authenticated local API for creating layer-shell windows and
controlling session locks.

Shell backends remain ordinary HTTP applications. They can use any frontend stack, own their native
integration logic, and ask YAWSF to display selected routes as compositor-managed surfaces.

## Features

- `wlr-layer-shell` windows for bars, panels, backgrounds, and overlays
- `ext-session-lock-v1` session-lock surfaces
- Supervised web backend startup and shutdown
- Authenticated host API with OpenAPI and Scalar reference
- Serialized layer-shell mutations per window ID
- SvelteKit example shell with Niri workspaces, MPRIS controls, and system status

## Run the example

Requirements: Linux, Wayland, and a compositor supporting the protocols used by the requested
surfaces. The included topbar integration also requires Niri.

```sh
direnv allow
pnpm --dir example-shell install
nix run .#default -- --webapp-command "pnpm --dir example-shell dev"
```

YAWSF starts and supervises Vite, sends host credentials to the shell's `POST /_start` callback,
then creates one topbar for each Niri output. The native host API defaults to
`http://127.0.0.1:12550/`; Vite defaults to `http://127.0.0.1:12551/`.

While YAWSF is running, its interactive API reference is available at
`http://127.0.0.1:12550/scalar`.

## Documentation

The Starlight documentation covers setup, shell lifecycle, the CLI, the example shell, and the
generated Host API reference.

```sh
pnpm --dir docs install
pnpm --dir docs dev
```

Open `http://127.0.0.1:24540/`, or start with [`docs/README.md`](docs/README.md). See
[`example-shell/README.md`](example-shell/README.md) for the example architecture and its custom
server-side HMR lifecycle.

## Build

Nix is the fully supported way to build YAWSF. Direct Cargo builds are semi-supported.

```sh
nix build .#default
```

## Development

```sh
nix fmt
nix flake check
pnpm --dir example-shell check
pnpm --dir example-shell lint
pnpm --dir docs format:check
pnpm --dir docs build
```

Regenerate the checked-in YAWSF Host API document after host API changes:

```sh
pnpm --dir docs generate:yawsf-host-api
pnpm --dir docs format
```
