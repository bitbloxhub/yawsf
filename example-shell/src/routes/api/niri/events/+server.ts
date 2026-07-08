import type { RequestHandler } from "./$types"
import { services } from "$lib/server/services"

const encoder = new TextEncoder()

export const GET: RequestHandler = ({ request }) => {
	const niri = services()?.niri
	if (!niri) return new Response("Niri unavailable", { status: 503 })

	let unsubscribe: () => void = () => {}
	const stream = new ReadableStream<Uint8Array>({
		start(controller) {
			unsubscribe = niri.subscribeWorkspaces((workspaces) => {
				controller.enqueue(
					encoder.encode(`event: workspaces\ndata: ${JSON.stringify(workspaces)}\n\n`),
				)
			})
			request.signal.addEventListener("abort", unsubscribe, { once: true })
		},
		cancel() {
			unsubscribe()
		},
	})

	return new Response(stream, {
		headers: { "Cache-Control": "no-cache", "Content-Type": "text/event-stream" },
	})
}
