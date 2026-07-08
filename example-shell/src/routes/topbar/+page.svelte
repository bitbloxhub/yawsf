<script lang="ts">
	import MprisControls from "$lib/components/topbar/MprisControls.svelte"
	import SystemStatus from "$lib/components/topbar/SystemStatus.svelte"
	import WorkspaceControls from "$lib/components/topbar/WorkspaceControls.svelte"
	import "bulma/css/bulma.min.css"
	import "@fontsource/fira-code/400.css"
	import "@fontsource/fira-code/500.css"
	import "@fontsource/fira-code/600.css"
	import "@fontsource/fira-code/700.css"
</script>

<svelte:head>
	<title>YAWSF Topbar</title>
</svelte:head>

<header
	class="topbar is-family-monospace is-flex is-align-items-center is-justify-content-space-between has-background-transparent has-text-light px-2 py-0"
>
	<WorkspaceControls />

	<div class="brand widget has-background-dark has-text-grey-light is-uppercase">YAWSF</div>

	<MprisControls />

	<SystemStatus />
</header>

<style>
	:global(html) {
		--bulma-body-background-color: transparent;
		--bulma-family-monospace: "Fira Code", monospace;
		background-color: transparent;
	}

	:global(body) {
		background-color: transparent;
	}

	.topbar {
		height: 31px;
		font-family: "Fira Code", monospace;
		font-size: 13.5px;
		font-weight: 500;
		font-variant-ligatures: none;
		line-height: 1;
		text-rendering: auto;
	}

	.topbar :global(*) {
		font-family: "Fira Code", monospace;
	}

	.brand {
		padding: 0 0.75rem;
		border-radius: 9999px;
		line-height: 25px;
	}
</style>
