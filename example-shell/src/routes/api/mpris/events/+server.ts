import type { RequestHandler } from "./$types"
import { services } from "$lib/server/services"

const encoder = new TextEncoder()

export const GET: RequestHandler = ({ request }) => {
	const mpris = services()?.mpris
	if (!mpris) return new Response("MPRIS unavailable", { status: 503 })

	let unsubscribe: () => void = () => {}
	const stream = new ReadableStream<Uint8Array>({
		start(controller) {
			unsubscribe = mpris.subscribe((players) => {
				controller.enqueue(
					encoder.encode(`event: players\ndata: ${JSON.stringify(players)}\n\n`),
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
