---
title: Build a shell
description: Connect a web backend to YAWSF and create native surfaces.
---

A YAWSF shell is an HTTP application. It serves the pages displayed inside native surfaces and
implements lifecycle callbacks used by the host.

## Implement lifecycle callbacks

Implement the shell webhooks documented in the [Host API reference](../../reference/host-api/). The
startup callback supplies the host API URL and bearer token. Keep both in server-only state; browser
code should call your backend rather than receiving host credentials directly.

## Use the host API

After bootstrap, make host API requests from server-only code with the supplied bearer token. Keep
the token out of browser state and expose only shell-specific actions to your frontend.

See the [Host API reference](../../reference/host-api/) for the complete interactive OpenAPI
document, request schemas, and response details.

## Shut down cleanly

When YAWSF supervises the shell with `--webapp-command`, exiting either process shuts down both.
Externally managed shells can use the shutdown webhook documented in the Host API reference.
