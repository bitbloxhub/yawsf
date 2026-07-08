---
title: Getting started
description: Build YAWSF and run the included SvelteKit shell.
---

## Requirements

YAWSF runs on Wayland and needs a compositor that supports the protocols used by the surfaces you
create. Layer-shell windows require `wlr-layer-shell`; session locking requires
`ext-session-lock-v1`.

The repository provides a Nix development shell with Rust, Node.js, pnpm, GTK, WebKitGTK, and the
other native dependencies.

## Run the example shell

From the repository root:

```sh
direnv allow
pnpm --dir example-shell install
nix run .#default -- --webapp-command "pnpm --dir example-shell dev"
```

YAWSF listens on `127.0.0.1:12550`. Vite listens on `127.0.0.1:12551`.

The example reads `NIRI_SOCKET` to create one topbar for each Niri output. It also connects to the
current user's D-Bus session for MPRIS media players. Missing integrations remain empty instead of
preventing startup.

## What happens at startup

1. YAWSF starts the command supplied through `--webapp-command`.
2. It waits for the shell backend to accept HTTP requests.
3. YAWSF sends its host API URL and generated bearer token to the shell's `POST /_start` callback.
4. The shell uses those credentials to create native surfaces through the host API.
5. If either process exits, YAWSF shuts down the other process.

The included shell stores the bootstrap payload, starts its Niri and MPRIS services, and reconciles
Niri outputs with layer-shell topbars.

## Inspect the host API

While YAWSF is running, open `http://127.0.0.1:12550/scalar`. The interactive API reference includes
the current bearer token and documents every host endpoint.

Next, read [Build a shell](./build-a-shell/) to implement the lifecycle in another web framework.
