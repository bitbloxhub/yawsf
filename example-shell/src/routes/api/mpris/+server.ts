import type { RequestHandler } from "./$types"
import { services } from "$lib/server/services"

const jsonHeaders = {
	"Content-Type": "application/json",
}

export const GET: RequestHandler = async () => {
	const mpris = services()?.mpris
	return new Response(JSON.stringify(mpris?.players() ?? []), {
		headers: jsonHeaders,
	})
}

export const POST: RequestHandler = async ({ request }) => {
	const mpris = services()?.mpris
	if (!mpris) {
		return new Response("MPRIS unavailable", { status: 503 })
	}

	const body = (await request.json()) as {
		action?: string
		serviceName?: string
	}
	if (!body.serviceName || !["playPause", "next", "previous"].includes(body.action ?? "")) {
		return new Response("Invalid MPRIS command", { status: 400 })
	}

	if (body.action === "playPause") {
		await mpris.playPause(body.serviceName)
	} else if (body.action === "next") {
		await mpris.next(body.serviceName)
	} else {
		await mpris.previous(body.serviceName)
	}

	return new Response(null, { status: 204 })
}
