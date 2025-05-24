<script lang="ts">
	import ErDiagramDialogWorkspace from '$lib/component/local/app/workspace-dashboard/dialog/ErDiagramDialogWorkspace.svelte';
	import ShareDatabaseConnection from '$lib/component/local/app/workspace-dashboard/dialog/ShareDatabaseConnection.svelte';
	import type {
		DatabaseConnectionInterface,
		DatasourceInterface
	} from '$lib/model/interface/schema.interface';
	import { onMount, createEventDispatcher } from 'svelte';

	let { x, y, show, databaseConnection, datasource, databaseName } = $props<{
		x: number;
		y: number;
		show: boolean;
		databaseConnection: DatabaseConnectionInterface;
		datasource: DatasourceInterface[];
		databaseName?: string;
	}>();

	let erDiagramDialogDatabaseConnection = $state(databaseConnection);
	

	const dispatch = createEventDispatcher();
	let menuElement: HTMLDivElement;
	let showErDiagramDialog = $state(false);
	let showShareDatabaseConnection = $state(false);

	function handleClickOutside(event: MouseEvent) {
		if (menuElement && !menuElement.contains(event.target as Node)) {
			// Instead of directly modifying show, dispatch an event to the parent
			dispatch('close');
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
		dispatch('close');
	}

	function shareDatabaseConnection() {
		showShareDatabaseConnection = true;
		dispatch('close');
	}
</script>

{#if show}
	<div
		bind:this={menuElement}
		class="context-menu flex flex-col gap-2"
		style="left: {x}px; top: {y}px;"
		role="menu"
		tabindex="0"
		oncontextmenu={handleContextMenu}
	>
		<button class="menu-item btn-primary" onclick={generateErDiagram}
			>Generate ER Diagram</button
		>
		<button class="menu-item btn-primary" onclick={shareDatabaseConnection}
			>Share Database Connection</button
		>
		
	</div>
{/if}

{#if showErDiagramDialog}
	<ErDiagramDialogWorkspace
		onClose={() => (showErDiagramDialog = false)}
		databaseConnection={erDiagramDialogDatabaseConnection}
		{datasource}
		databaseName={databaseName}
	/>
{/if}

{#if showShareDatabaseConnection}
	<ShareDatabaseConnection
		onClose={() => (showShareDatabaseConnection = false)}
		databaseConnection={erDiagramDialogDatabaseConnection}
	/>
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
