<script lang="ts">
	import { PlusSvg } from '$lib/asset/image/svg/plus-svg';
	import { selectedDatabaseState } from '$lib/store/state/selectedDatabase.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import NewFileDialogWorkspace from '../dialog/NewFileDialogWorkspace.svelte';
	import { onMount } from 'svelte';

	let selectedDatabase = $derived(selectedDatabaseState.selectedDatabase);

	type QueryFile = {
		id: number;
		name: string;
		description: string;
	};

	type QueryFileCollection = {
		id: number;
		name: string;
		description: string;
	};

	let showNewFileDialog = $state(false);
	let fileCollection = $state<QueryFileCollection[]>([]);

	onMount(() => {
		// fileCollection = await invoke<QueryFileCollection[]>(
		// 	'get_file_collection',
		// 	{
		// 		database_connection_id: selectedDatabase?.id
		// 	}
		// );
	});

	function handleAddNewFile() {
		showNewFileDialog = true;
	}
</script>

<main
	class="bg-base-200 rounded-box my-3 flex flex-col justify-between p-3 shadow-sm"
>
	<section class=" flex h-full flex-col gap-2">
		<div class="flex flex-row gap-2">
			<div>Current File :</div>
			<select class="select w-full">
				<option disabled selected>Pick a file collection ...</option>
				{#each fileCollection as collection}
					<option>{collection.name}</option>
				{/each}
			</select>
		</div>
		<div class="tooltip tooltip-right" data-tip="Add a new query file">
			<button class="btn btn-primary" onclick={handleAddNewFile}>
				{@html PlusSvg('size-4')} New File
			</button>
		</div>
	</section>
</main>

{#if showNewFileDialog}
	<NewFileDialogWorkspace onClose={() => (showNewFileDialog = false)} />
{/if}
