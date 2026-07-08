<script lang="ts">
	import { onMount } from "svelte"
	import Icon from "@iconify/svelte"
	import type { SystemStatus } from "$lib/types"

	let time = $state("")
	let battery = $state<number | null>(null)
	let charging = $state(false)
	let online = $state(false)

	function updateTime() {
		time = new Intl.DateTimeFormat(undefined, {
			hour: "2-digit",
			minute: "2-digit",
			second: "2-digit",
		}).format(new Date())
	}

	async function updateSystem() {
		try {
			const response = await fetch("/api/system")
			if (!response.ok) return
			const status = (await response.json()) as SystemStatus
			battery = status.battery.capacity
			charging = status.battery.charging
			online = status.online
		} catch {
			// Keep last known system status while the server reloads.
		}
	}

	onMount(() => {
		updateTime()
		void updateSystem()
		const timer = window.setInterval(updateTime, 1000)
		const systemTimer = window.setInterval(() => void updateSystem(), 30_000)

		return () => {
			window.clearInterval(timer)
			window.clearInterval(systemTimer)
		}
	})
</script>

<section
	class="widget status-widget is-flex is-align-items-center has-background-dark"
	aria-label="System status"
>
	<span class="status-item" title={online ? "Online" : "Offline"}>
		<Icon icon={online ? "mdi:wifi" : "mdi:wifi-off"} width="14" height="14" />
		<span class="is-sr-only">{online ? "Online" : "Offline"}</span>
	</span>
	<span class="status-item">
		<Icon icon={charging ? "mdi:battery-charging" : "mdi:battery"} width="14" height="14" />
		{battery === null ? "AC" : `${battery}%`}
	</span>
	<time class="status-item" datetime={new Date().toISOString()}>{time}</time>
</section>

<style>
	.widget {
		border-radius: 9999px;
		line-height: 25px;
	}

	.status-widget {
		gap: 0.5rem;
		margin-left: 0.25rem;
		padding: 0 0.5rem;
	}

	.status-item {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
	}
</style>
