<script lang="ts">
	import { resolve } from "$app/paths"
</script>

<svelte:head>
	<title>YAWSF Example Shell</title>
</svelte:head>

<main>
	<p class="eyebrow">YAWSF example shell</p>
	<h1>Native desktop surfaces, rendered with SvelteKit</h1>
	<p>
		This app demonstrates a Niri-aware topbar, MPRIS controls, system status, and a session lock
		surface backed by the YAWSF host API.
	</p>
	<nav aria-label="Example surfaces">
		<a href={resolve("/topbar")}>Open topbar</a>
		<a href={resolve("/lock")}>Preview lock screen</a>
	</nav>
	<p class="hint">Run this app through YAWSF to create native layer-shell surfaces.</p>
</main>

<style>
	:global(body) {
		margin: 0;
		background: #11111b;
		color: #e5e7eb;
		font-family: system-ui, sans-serif;
	}

	main {
		max-width: 44rem;
		margin: 0 auto;
		padding: 6rem 2rem;
	}

	.eyebrow {
		color: #7aa2f7;
		font-weight: 700;
		letter-spacing: 0.08em;
		text-transform: uppercase;
	}

	h1 {
		font-size: clamp(2rem, 6vw, 4rem);
		line-height: 1.05;
	}

	p {
		line-height: 1.6;
	}

	nav {
		display: flex;
		gap: 0.75rem;
		margin: 2rem 0;
	}

	a {
		padding: 0.75rem 1rem;
		border-radius: 0.5rem;
		background: #7aa2f7;
		color: #11111b;
		font-weight: 700;
		text-decoration: none;
	}

	.hint {
		color: #9ca3af;
	}
</style>
