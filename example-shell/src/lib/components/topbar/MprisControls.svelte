<script lang="ts">
	import { onMount } from "svelte"
	import { Button } from "bits-ui"
	import Icon from "@iconify/svelte"
	import type { MprisPlayer } from "$lib/types"

	let now = $state(Date.now())
	let playerUpdatedAt = $state(Date.now())
	let players = $state<MprisPlayer[]>([])
	let activePlayer = $derived(players[0])
	let progress = $derived.by(() => {
		if (!activePlayer?.length) return 0
		const elapsed =
			activePlayer.playbackStatus === "Playing" ? (now - playerUpdatedAt) * 1000 : 0
		return Math.min((activePlayer.position + elapsed) / activePlayer.length, 1)
	})

	async function controlPlayer(action: "playPause" | "next" | "previous") {
		if (!activePlayer) return
		await fetch("/api/mpris", {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify({ action, serviceName: activePlayer.serviceName }),
		})
	}

	onMount(() => {
		const playersSource = new EventSource("/api/mpris/events")
		playersSource.addEventListener("players", (event) => {
			players = JSON.parse((event as MessageEvent<string>).data)
			playerUpdatedAt = Date.now()
		})
		const timer = window.setInterval(() => {
			now = Date.now()
		}, 1000)

		return () => {
			window.clearInterval(timer)
			playersSource.close()
		}
	})
</script>

{#if activePlayer}
	<section
		class="widget mpris-widget is-flex is-align-items-center has-background-dark has-text-light"
		aria-label="Media controls"
	>
		<div class="mpris-art" style={`--progress: ${progress}`}>
			{#if activePlayer.artUrl}
				<img src={activePlayer.artUrl} alt="" />
			{:else}
				<Icon icon="mdi:music" width="20" height="20" />
			{/if}
		</div>
		<Button.Root
			class="mpris-button button is-ghost has-text-light"
			aria-label="Previous track"
			disabled={!activePlayer.canGoPrevious}
			onclick={() => void controlPlayer("previous")}
		>
			<Icon icon="mdi:skip-previous" width="16" height="16" />
		</Button.Root>
		<Button.Root
			class="mpris-button button is-ghost has-text-light"
			aria-label={activePlayer.playbackStatus === "Playing" ? "Pause" : "Play"}
			onclick={() => void controlPlayer("playPause")}
		>
			<Icon
				icon={activePlayer.playbackStatus === "Playing" ? "mdi:pause" : "mdi:play"}
				width="16"
				height="16"
			/>
		</Button.Root>
		<Button.Root
			class="mpris-button button is-ghost has-text-light"
			aria-label="Next track"
			disabled={!activePlayer.canGoNext}
			onclick={() => void controlPlayer("next")}
		>
			<Icon icon="mdi:skip-next" width="16" height="16" />
		</Button.Root>
		<span class="mpris-title" title={`${activePlayer.artist} — ${activePlayer.title}`}>
			<span>{activePlayer.artist || activePlayer.title || activePlayer.identity}</span>
		</span>
	</section>
{/if}

<style>
	.widget {
		border-radius: 9999px;
		line-height: 25px;
	}

	.mpris-widget {
		gap: 0.25rem;
		padding: 0 0.5rem;
	}

	.mpris-widget :global(.mpris-button.button) {
		width: 21px;
		min-width: 21px;
		height: 25px;
		min-height: 25px;
		padding: 0;
	}

	.mpris-art {
		position: relative;
		display: grid;
		place-items: center;
		width: 21px;
		height: 21px;
		flex: none;
		padding: 2px;
		border-radius: 50%;
		background: conic-gradient(#4c8dff calc(var(--progress) * 1turn), #6b7280 0);
	}

	.mpris-art img {
		width: 100%;
		height: 100%;
		border-radius: 50%;
		object-fit: cover;
	}

	.mpris-title {
		max-width: 12rem;
		overflow: hidden;
		white-space: nowrap;
	}
</style>
