import type { Handle, ServerInit } from "@sveltejs/kit"
import { services, startInfo } from "$lib/server/services"

export const init: ServerInit = async () => {
	process.on("sveltekit:shutdown", () => {
		const info = startInfo()
		if (!info) return

		void fetch(`${info.host_api}/quit`, {
			method: "POST",
			headers: { Authorization: `Bearer ${info.token}` },
		}).catch(() => undefined)
	})
}

export const handle: Handle = async ({ event, resolve }) => {
	event.locals.start_info = startInfo()
	event.locals.services = services()

	const response = await resolve(event)
	if (response.headers.get("Content-Type")?.startsWith("text/html")) {
		response.headers.set("Cache-Control", "no-cache")
	}
	return response
}
