<script lang="ts">
	import ErDiagramDialogWorkspace from '$lib/component/local/app/workspace-dashboard/dialog/ErDiagramDialogWorkspace.svelte';
	import { onMount } from 'svelte';

	let { x, y, show } = $props<{ x: number; y: number; show: boolean }>();

	let menuElement: HTMLDivElement;
	let showErDiagramDialog = $state(false);

	function handleClickOutside(event: MouseEvent) {
		if (menuElement && !menuElement.contains(event.target as Node)) {
			show = false;
		}
	}

	function handleContextMenu(event: MouseEvent) {
		event.preventDefault();
		event.stopPropagation();
	}

	onMount(() => {
		document.addEventListener('click', handleClickOutside);
		document.addEventListener('contextmenu', handleContextMenu);
		return () => {
			document.removeEventListener('click', handleClickOutside);
			document.removeEventListener('contextmenu', handleContextMenu);
		};
	});

	function generateErDiagram() {
		showErDiagramDialog = true;
		show = false;
	}
</script>

{#if show}
	<div
		bind:this={menuElement}
		class="context-menu"
		style="left: {x}px; top: {y}px;"
		role="menu"
		tabindex="0"
		oncontextmenu={handleContextMenu}
	>
		<button class="menu-item btn-primary" onclick={generateErDiagram}
			>Generate ER Diagram</button
		>
		<div class="menu-item">Invite Collaborators</div>
	</div>
{/if}

{#if showErDiagramDialog}
	<ErDiagramDialogWorkspace onClose={() => (showErDiagramDialog = false)} />
{/if}

<style>
	.context-menu {
		position: fixed;
		background: white;
		border: 1px solid #ccc;
		border-radius: 4px;
		padding: 8px 0;
		min-width: 150px;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
		z-index: 1000;
	}

	.menu-item {
		padding: 8px 16px;
		cursor: pointer;
	}

	.menu-item:hover {
		background-color: #f0f0f0;
	}
</style>
