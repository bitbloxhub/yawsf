import type { NiriService } from "./niri"

interface HostInfo {
	host_api: string
	token: string
}

export interface TopbarService {
	stop(): Promise<void>
}

export function startTopbars(info: HostInfo, shellUrl: string, niri: NiriService): TopbarService {
	const bars = new Set<string>()
	let reconciliation = Promise.resolve()
	const unsubscribe = niri.subscribe((outputs) => {
		reconciliation = reconciliation.then(() => reconcileBars(info, shellUrl, bars, outputs))
	})

	return {
		async stop() {
			unsubscribe()
			await reconciliation
			await Promise.allSettled([...bars].map((monitor) => closeBar(info, monitor)))
		},
	}
}

async function reconcileBars(
	info: HostInfo,
	shellUrl: string,
	bars: Set<string>,
	outputs: string[],
): Promise<void> {
	const activeMonitors = new Set(outputs)

	for (const monitor of outputs) {
		if (bars.has(monitor)) continue
		try {
			await upsertBar(info, shellUrl, monitor)
			bars.add(monitor)
		} catch (error) {
			console.warn("failed to upsert bar", error)
		}
	}

	for (const monitor of bars) {
		if (activeMonitors.has(monitor)) continue
		try {
			await closeBar(info, monitor)
			bars.delete(monitor)
		} catch (error) {
			console.warn("failed to close bar", error)
		}
	}
}

async function upsertBar(info: HostInfo, shellUrl: string, monitor: string): Promise<void> {
	const id = `example-topbar-${monitor}`
	await hostFetch(info, `layer-shell/${id}`, {
		method: "PUT",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify({
			id,
			url: new URL("/topbar", shellUrl).toString(),
			namespace: "example-topbar",
			layer: "top",
			anchors: { top: true, left: true, right: true, bottom: false },
			exclusiveZone: { mode: "auto" },
			margins: { top: 0, bottom: 0, left: 0, right: 0 },
			keyboardMode: "none",
			width: null,
			height: 31,
			monitor,
		}),
	})
}

async function closeBar(info: HostInfo, monitor: string): Promise<void> {
	await hostFetch(info, `layer-shell/example-topbar-${monitor}`, { method: "DELETE" })
}

async function hostFetch(info: HostInfo, path: string, init: RequestInit): Promise<Response> {
	const response = await fetch(new URL(path, info.host_api), {
		...init,
		headers: { Authorization: `Bearer ${info.token}`, ...init.headers },
	})
	if (!response.ok) throw new Error(`${init.method ?? "GET"} ${path} failed: ${response.status}`)
	return response
}
