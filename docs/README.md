# YAWSF documentation

Starlight site for Yet Another Wayland Shell Framework (YAWSF) user and API documentation.

From the repository root:

```sh
pnpm --dir docs install
pnpm --dir docs dev
```

The development server runs at `http://127.0.0.1:24540`. Documentation pages live in
`src/content/docs/`; navigation is configured in `astro.config.mjs`.

The Host API page embeds Scalar using the checked-in `public/openapi.json`. Regenerate the YAWSF Host
API contract after host API changes:

```sh
pnpm --dir docs generate:yawsf-host-api
pnpm --dir docs format
```

Build the static site with:

```sh
pnpm --dir docs build
```
