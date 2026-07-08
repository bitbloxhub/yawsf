<script lang="ts">
	import { onMount } from "svelte"
	import { Button } from "bits-ui"
	import Icon from "@iconify/svelte"
	import type { NiriWorkspace } from "$lib/types"

	let workspaces = $state<NiriWorkspace[]>([])

	async function lockSession() {
		await fetch("/api/session-lock", {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify({ action: "lock" }),
		})
	}

	async function focusWorkspace(index: number) {
		await fetch("/api/niri", {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify({ index }),
		})
	}

	onMount(() => {
		const workspacesSource = new EventSource("/api/niri/events")
		workspacesSource.addEventListener("workspaces", (event) => {
			workspaces = JSON.parse((event as MessageEvent<string>).data)
		})

		return () => workspacesSource.close()
	})
</script>

<section
	class="widget workspace-controls is-flex is-align-items-center has-background-dark"
	aria-label="Workspace controls"
>
	<Button.Root
		class="lock-button button is-dark is-rounded"
		aria-label="Lock session"
		onclick={() => void lockSession()}
	>
		<Icon icon="mdi:lock" width="15" height="15" />
	</Button.Root>
	<nav class="is-flex" aria-label="Workspaces">
		{#each workspaces as workspace (workspace.id)}
			<Button.Root
				class={`workspace-button button is-rounded ${workspace.is_focused ? "is-link" : "is-dark"}`}
				aria-label={`Workspace ${workspace.name ?? workspace.idx}`}
				aria-pressed={workspace.is_focused}
				onclick={() => void focusWorkspace(workspace.idx)}
			>
				{workspace.name ?? workspace.idx}
			</Button.Root>
		{/each}
	</nav>
</section>

<style>
	.widget {
		border-radius: 9999px;
		line-height: 25px;
	}

	.workspace-controls {
		gap: 0.25rem;
		margin-right: 0.25rem;
	}

	.workspace-controls :global(.button) {
		height: 25px;
		min-height: 25px;
		font-size: 13.5px;
	}

	.workspace-controls :global(.lock-button.button) {
		width: 25px;
		min-width: 25px;
		padding: 0;
	}

	.workspace-controls :global(.workspace-button.button) {
		padding: 0 0.5rem;
	}
</style>
