---
title: CLI
description: YAWSF command-line options.
---

```text
Usage: yawsf [OPTIONS]
```

## `--base-url <BASE_URL>`

URL of the shell backend. Defaults to `http://127.0.0.1:12551/`. YAWSF sends lifecycle callbacks to
this base URL and loads surface URLs supplied by the shell.

## `--bind <BIND>`

Address for the authenticated native host API. Defaults to `127.0.0.1:12550`.

## `--token <TOKEN>`

Bearer token required by the host API. YAWSF generates a random token when this option is omitted
and provides it to the shell through `POST /_start`.

## `--webapp-command <COMMAND>`

Starts the shell backend as a supervised child. The value supports shell-style argument quoting but
does not invoke a shell.

```sh
result/bin/yawsf --webapp-command "pnpm --dir example-shell dev"
```

YAWSF waits up to 30 seconds for the child to accept the startup callback. When YAWSF shuts down, it
sends the child `SIGTERM`, waits five seconds, then uses `SIGKILL` if needed. If the child exits first,
YAWSF also shuts down.
