import type { RequestHandler } from "./$types"
import { startInfo } from "$lib/server/services"

export const POST: RequestHandler = async ({ request, url }) => {
	const info = startInfo()
	if (!info) return new Response("YAWSF host unavailable", { status: 503 })

	const body = (await request.json()) as { action?: string }
	if (body.action === "lock") {
		return fetch(new URL("session-lock/lock", info.host_api), {
			method: "POST",
			headers: {
				Authorization: `Bearer ${info.token}`,
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ url: new URL("/lock", url.origin).toString() }),
		})
	}
	if (body.action === "unlock") {
		return fetch(new URL("session-lock/unlock", info.host_api), {
			method: "POST",
			headers: { Authorization: `Bearer ${info.token}` },
		})
	}

	return new Response("Invalid session-lock action", { status: 400 })
}
