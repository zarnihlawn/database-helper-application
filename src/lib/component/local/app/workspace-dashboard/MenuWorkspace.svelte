<script lang="ts">
	import { AddSvg } from '$lib/asset/image/svg/add-svg';
	import { MenuSvg } from '$lib/asset/image/svg/menu-svg';
	import { RefreshSvg } from '$lib/asset/image/svg/refresh-svg';
	import { RemoveSvg } from '$lib/asset/image/svg/remove-svg';
	import type {
		DatabaseConnectionInterface,
		DatasourceInterface
	} from '$lib/model/interface/schema.interface';
	import { goToRoute } from '$lib/util/router.util';
	import RemoveDatasourceWorkspace from './dialog/RemoveDatasourceWorkspace.svelte';
	import WorkspaceSelectDatasource from './dialog/SelectDatasourceWorkspace.svelte';
	import { invoke } from '@tauri-apps/api/core';

	let { datasource, databaseConnection } = $props<{
		datasource: DatasourceInterface[];
		databaseConnection: DatabaseConnectionInterface[];
	}>();

	let showRemoveDatasourceDialog = $state(false);

	let showSelectionDialog = $state(false);

	function handleAddClick() {
		showSelectionDialog = true;
	}

	function handleRefreshClick() {
		invoke('refresh_window');
	}

	function handleDialogClose() {
		showSelectionDialog = false;
		showRemoveDatasourceDialog = false;
	}

	function handleRemoveClick() {
		showRemoveDatasourceDialog = true;
	}
	function goToDocumentationPage() {
		goToRoute('/app/documentation-dashboard/user-manual-documentation');
	}
</script>

<main class="flex w-full justify-between">
	<div class="join">
		<div
			class="tooltip tooltip-right tooltip-success"
			data-tip="Add a data source"
		>
			<button class="btn text-success join-item" onclick={handleAddClick}>
				{@html AddSvg}
			</button>
		</div>
		<div
			class="tooltip tooltip-right tooltip-warning"
			data-tip="Refresh all data sources"
		>
			<button class="btn text-warning join-item" onclick={handleRefreshClick}>
				{@html RefreshSvg}
			</button>
		</div>
		<div
			class="tooltip tooltip-right tooltip-error"
			data-tip="Remove a data source"
		>
			<button class="btn text-error join-item" onclick={handleRemoveClick}>
				{@html RemoveSvg}
			</button>
		</div>
	</div>
	<div class="join">
		<div class="tooltip tooltip-right tooltip-info" data-tip="More Actions">
			<button class="btn text-info join-item" onclick={goToDocumentationPage}>
				{@html MenuSvg}
			</button>
		</div>
	</div>
</main>

{#if showSelectionDialog}
	<WorkspaceSelectDatasource {datasource} onClose={handleDialogClose} />
{/if}

{#if showRemoveDatasourceDialog}
	<RemoveDatasourceWorkspace
		{datasource}
		{databaseConnection}
		onClose={handleDialogClose}
	/>
{/if}
