<script lang="ts">
	import { onMount } from "svelte"

	let time = $state("")
	let unlocking = $state(false)

	function updateTime() {
		time = new Intl.DateTimeFormat(undefined, {
			hour: "2-digit",
			minute: "2-digit",
		}).format(new Date())
	}
	async function unlock() {
		if (unlocking) return
		unlocking = true
		try {
			await window.fetch("/api/session-lock", {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify({ action: "unlock" }),
			})
		} finally {
			unlocking = false
		}
	}

	onMount(() => {
		updateTime()
		const timer = window.setInterval(updateTime, 1000)

		return () => window.clearInterval(timer)
	})
</script>

<svelte:head>
	<title>Locked</title>
</svelte:head>

<button class="lockscreen" onclick={() => void unlock()} disabled={unlocking}>
	<span class="lockscreen-content">
		<time class="lockscreen-clock" datetime={new Date().toISOString()}>{time}</time>
		<span>{unlocking ? "Unlocking…" : "Click anywhere to unlock"}</span>
	</span>
</button>

<style>
	:global(html),
	:global(body) {
		width: 100%;
		height: 100%;
		margin: 0;
		background: #11111b;
	}

	.lockscreen {
		display: flex;
		width: 100%;
		height: 100%;
		align-items: center;
		justify-content: center;
		border: 0;
		position: relative;
		overflow: hidden;
		background: #3b5b8a;
		color: #ffffff;
		font-family: "Fira Code", monospace;
		font-size: 1rem;
		cursor: pointer;
	}

	.lockscreen::before {
		position: absolute;
		inset: 0;
		background: radial-gradient(circle at center, #8c6bb1, #5c6fa3 38%, #3b5b8a 68%, #27406b);
		content: "";
		animation: color-shift 16s ease-in-out infinite;
	}

	.lockscreen-content {
		position: relative;
		z-index: 1;
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.5rem;
		border: 1px solid rgb(255 255 255 / 20%);
		border-radius: 1rem;
		background: rgb(17 17 27 / 68%);
		box-shadow: 0 12px 40px rgb(17 17 27 / 45%);
	}

	.lockscreen-clock {
		font-size: 2rem;
		font-weight: 500;
		font-variant-numeric: tabular-nums;
	}

	@keyframes color-shift {
		0%,
		100% {
			filter: hue-rotate(0deg) saturate(1.1);
		}

		50% {
			filter: hue-rotate(180deg) saturate(1.2);
		}
	}
</style>
