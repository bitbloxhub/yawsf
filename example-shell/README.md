# YAWSF example shell

A small SvelteKit desktop shell showing how a web app can drive native YAWSF surfaces.

The example creates one layer-shell topbar on each Niri output. The bars show live workspaces,
MPRIS media controls, battery and network status, and a clock. It also includes a session-lock
surface.

## Run it

From the repository root:

```sh
direnv allow
pnpm --dir example-shell install
nix run .#default -- --webapp-command "pnpm --dir example-shell dev"
```

`--webapp-command` makes YAWSF supervise the Vite process. YAWSF waits for the app to start, sends
its host URL and bearer token to `POST /_start`, and shuts down when the child process exits.
Defaults are `127.0.0.1:12550` for the YAWSF host API and `127.0.0.1:12551` for Vite.

Niri integration reads `NIRI_SOCKET`. MPRIS integration uses the current user's D-Bus session.
When either integration is unavailable, the corresponding service remains empty instead of
preventing the app from starting.

## How the example is organized

- `src/lib/server/services.ts` owns server-service startup and shutdown.
- `src/lib/server/services/niri.ts` consumes Niri IPC and publishes output/workspace changes.
- `src/lib/server/services/mpris.ts` consumes MPRIS over D-Bus.
- `src/lib/server/services/topbars.ts` reconciles Niri outputs with YAWSF layer-shell surfaces.
- `src/routes/api/` exposes browser-safe endpoints and event streams for those services.
- `src/lib/components/topbar/` contains self-contained topbar widgets.
- `src/routes/topbar` and `src/routes/lock` render the native surface contents.
- `src/routes/_start` and `src/routes/_quit` implement YAWSF lifecycle callbacks.

Code under `$lib/server` never runs in the browser. Components reach native integrations through
SvelteKit API routes, while shared response types live in `src/lib/types.ts`.

## Server-side reloading

Vite normally reloads Svelte components, but this example also owns long-lived server resources:
a Niri socket, D-Bus subscriptions, SSE listeners, and native topbar surfaces. Reloading a server
module without stopping its previous instance would duplicate those resources.

`vite.config.ts` therefore registers the custom `yawsf-services-hmr` plugin. During development it
watches `src/lib/server/services.ts` and every module under `src/lib/server/services/`. When one of
those files changes, the plugin:

1. Serializes the reload behind the previous service reload.
2. Invalidates `services.ts` in Vite's server module graph.
3. Loads it again with `server.ssrLoadModule`.

`services.ts` keeps YAWSF bootstrap details in a `globalThis` entry keyed by `Symbol.for`. On
reevaluation it stops the previous service graph before creating the replacement. This gives
server integrations an explicit HMR lifecycle instead of leaving sockets, subscriptions, or
surfaces from an old module instance running.

This plugin is example-specific infrastructure, not a general SvelteKit requirement.

## Useful commands

```sh
pnpm --dir example-shell format
pnpm --dir example-shell check
pnpm --dir example-shell lint
pnpm --dir example-shell build
```

To run the production build under YAWSF:

```sh
pnpm --dir example-shell build
cargo run --release -- --webapp-command "pnpm --dir example-shell start"
# or `nix run .#default --`
```
