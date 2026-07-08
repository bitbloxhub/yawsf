import { readdir, readFile } from "node:fs/promises"

import type { RequestHandler } from "./$types"

interface BatteryStatus {
	capacity: number | null
	charging: boolean
}

export const GET: RequestHandler = async () => {
	const [battery, online] = await Promise.all([readBattery(), readOnline()])
	return Response.json({ battery, online }, { headers: { "Cache-Control": "no-store" } })
}

async function readBattery(): Promise<BatteryStatus> {
	try {
		const supplies = await readdir("/sys/class/power_supply")
		const battery = supplies.find((supply) => supply.startsWith("BAT"))
		if (!battery) return { capacity: null, charging: false }

		const [capacity, status] = await Promise.all([
			readFile(`/sys/class/power_supply/${battery}/capacity`, "utf8"),
			readFile(`/sys/class/power_supply/${battery}/status`, "utf8"),
		])
		return { capacity: Number.parseInt(capacity, 10), charging: status.trim() === "Charging" }
	} catch {
		return { capacity: null, charging: false }
	}
}

async function readOnline(): Promise<boolean> {
	try {
		const interfaces = await readdir("/sys/class/net")
		const states = await Promise.all(
			interfaces
				.filter((name) => name !== "lo")
				.map((name) => readFile(`/sys/class/net/${name}/operstate`, "utf8")),
		)
		return states.some((state) => state.trim() === "up")
	} catch {
		return false
	}
}
