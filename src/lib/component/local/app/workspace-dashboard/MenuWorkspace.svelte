<script lang="ts">
	import { AddSvg } from '$lib/asset/image/svg/add-svg';
	import { MenuSvg } from '$lib/asset/image/svg/menu-svg';
	import { RefreshSvg } from '$lib/asset/image/svg/refresh-svg';
	import { RemoveSvg } from '$lib/asset/image/svg/remove-svg';
	import type { datasourceAuthenticationTypeInterface } from '$lib/model/interface/schema.interface';
	import type { datasourceInterface } from '$lib/model/interface/schema.interface';
	import WorkspaceSelectDatasource from './dialog/SelectDatasourceWorkspace.svelte';

	let { datasource, datasourceAuthenticationType } = $props<{
		datasource: datasourceInterface[];
	}>();

	let showSelectionDialog = $state(false);

	function handleAddClick() {
		showSelectionDialog = true;
	}

	function handleDialogClose() {
		showSelectionDialog = false;
	}
</script>

<main class="flex w-full justify-between">
	<div class="join">
		<div class="tooltip tooltip-right tooltip-success" data-tip="Add a data source">
			<button class="btn text-success join-item" onclick={handleAddClick}>
				{@html AddSvg}
			</button>
		</div>
		<div class="tooltip tooltip-right tooltip-warning" data-tip="Refresh all data sources">
			<button class="btn text-warning join-item">
				{@html RefreshSvg}
			</button>
		</div>
		<div class="tooltip tooltip-right tooltip-error" data-tip="Remove a data source">
			<button class="btn text-error join-item">
				{@html RemoveSvg}
			</button>
		</div>
	</div>
	<div class="join">
		<div class="tooltip tooltip-right tooltip-info" data-tip="More Actions">
			<button class="btn text-info join-item">
				{@html MenuSvg}
			</button>
		</div>
	</div>
</main>

{#if showSelectionDialog}
	<WorkspaceSelectDatasource
		{datasource}
		{datasourceAuthenticationType}
		onClose={handleDialogClose}
	/>
{/if}
