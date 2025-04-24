<script lang="ts">
	import { PlusSvg } from '$lib/asset/image/svg/plus-svg';
	import { selectedDatabaseState } from '$lib/store/state/selectedDatabase.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import NewFileDialogWorkspace from '../dialog/NewFileDialogWorkspace.svelte';
	import { onDestroy, onMount } from 'svelte';
	import { EditSvg } from '$lib/asset/image/svg/edit-svg';
	import { DeleteSvg } from '$lib/asset/image/svg/delete-svg';
	import EditFileDialogWorkspace from '../dialog/EditFileDialogWorkspace.svelte';
	import type { QueryFileInterface } from '$lib/model/interface/schema.interface';
	import { selectedFileState } from '$lib/store/state/selectedFile.svelte';

	let selectedDatabase = $derived(selectedDatabaseState.selectedDatabase);
	let showNewFileDialog = $state(false);
	let showEditFileDialog = $state(false);
	let fileCollection = $state<QueryFileInterface[]>([]);

	async function getFileCollection() {
		fileCollection = await invoke<QueryFileInterface[]>('get_file_collection', {
			databaseConnectionId: selectedDatabase?.id
		});
	}

	onMount(() => {
		getFileCollection();
	});

	function handleAddNewFile() {
		showNewFileDialog = true;
	}
	function handleEditFile() {
		showEditFileDialog = true;
	}
	function handleSelectedFile(file: QueryFileInterface) {
		selectedFileState.selectedFile = file;
		console.log(selectedFileState.selectedFile);
	}

	onDestroy(() => {
		selectedFileState.selectedFile = null;
	});
</script>

<main class="flex flex-row justify-between gap-3">
	<section class=" flex w-full gap-2">
		<button class="btn btn-primary" onclick={handleAddNewFile}>
			{@html PlusSvg('size-4')} New File
		</button>
		<button
			class="btn btn-warning"
			class:disabled={fileCollection.length === 0}
			onclick={handleEditFile}
		>
			{@html EditSvg('size-4')} Edit File
		</button>
		<button class="btn btn-error" class:disabled={fileCollection.length === 0}>
			{@html DeleteSvg('size-4')} Delete File
		</button>
	</section>
	<section class="flex w-full">
		<select
			class="select w-full"
			onchange={(e: Event) => {
				const target = e.target as HTMLSelectElement;
				const selectedFile = fileCollection.find(
					(file) => file.name === target.value
				);
				if (selectedFile) {
					handleSelectedFile(selectedFile);
				}
			}}
		>
			<option disabled selected>Pick a file ...</option>
			{#each fileCollection as collection}
				<option>{collection.name}</option>
			{/each}
		</select>
	</section>
</main>

{#if showNewFileDialog}
	<NewFileDialogWorkspace onClose={() => (showNewFileDialog = false)} />
{/if}

{#if showEditFileDialog}
	<EditFileDialogWorkspace
		{fileCollection}
		onClose={() => (showEditFileDialog = false)}
	/>
{/if}
