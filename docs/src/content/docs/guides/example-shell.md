---
title: Example shell
description: Tour the included SvelteKit shell and its development lifecycle.
---

`example-shell/` is a reference SvelteKit implementation of a YAWSF shell. It demonstrates:

- one Niri topbar per connected output;
- live workspace state and workspace focus commands;
- MPRIS metadata, playback controls, and progress interpolation;
- battery and network status;
- a session-lock page;
- server-sent events between SvelteKit server services and browser components.

## Service boundaries

`src/lib/server/services.ts` owns the service graph. Niri IPC, MPRIS D-Bus integration, and topbar
reconciliation live in separate modules under `src/lib/server/services/`. Browser components never
import those implementations; they communicate through routes under `src/routes/api/`.

Topbar components own their own event streams, timers, and API actions. The topbar page only composes
those components and defines page-level styling.

## Server-side HMR

Niri sockets, D-Bus subscriptions, and native surfaces outlive a normal server-module evaluation.
The custom `yawsf-services-hmr` plugin in `vite.config.ts` gives these resources an explicit reload
lifecycle during development.

When `services.ts` or a module in `services/` changes, the plugin serializes reloads, invalidates the
service entry in Vite's server module graph, and reevaluates it with `server.ssrLoadModule`.
`services.ts` keeps bootstrap details in `globalThis`, stops the previous service graph, then starts
the replacement. This prevents old sockets and subscriptions from surviving an HMR update.

The plugin is specific to the example's long-lived server resources; it is not required for normal
Svelte component HMR.

## Validate the example

```sh
pnpm --dir example-shell format
pnpm --dir example-shell check
pnpm --dir example-shell lint
pnpm --dir example-shell build
```
