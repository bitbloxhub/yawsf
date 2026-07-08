import type { RequestHandler } from "./$types"
import { services } from "$lib/server/services"

export const GET: RequestHandler = async () => {
	return Response.json(services()?.niri.workspaces() ?? [])
}

export const POST: RequestHandler = async ({ request }) => {
	const index = ((await request.json()) as { index?: unknown }).index
	if (typeof index !== "number") return new Response("Workspace index required", { status: 400 })
	const niri = services()?.niri
	if (!niri) return new Response("Niri unavailable", { status: 503 })
	await niri.focusWorkspace(index)
	return new Response(null, { status: 204 })
}
