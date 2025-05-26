<script lang="ts">
	import type { QueryFileInterface } from '$lib/model/interface/schema.interface';
	import { selectedDatabaseState } from '$lib/store/state/selectedDatabase.svelte';
	import { invoke } from '@tauri-apps/api/core';

	let fileName = $state('');
	let fileDescription = $state(' ');
	let databaseConnectionId = $derived(
		selectedDatabaseState.selectedDatabase?.id
	);

	let { fileCollection, onClose } = $props<{
		fileCollection: QueryFileInterface[];
		onClose: () => void;
	}>();

	let selectedFileId = $state(0);


	function handleClose() {
		onClose();
	}

	async function handleEditFile() {
		if (fileName === '') {
			return;
		} else {
			if (selectedFileId) {
				try {
					console.log(
						'Selected File Id: ',
						selectedFileId)
					await invoke('edit_file_database', {
						id: selectedFileId,
						name: fileName,
						description: fileDescription
					});
					handleClose();
				} catch (error) {
					console.error(error);
				}
			}
		}
	}
</script>

<div class="modal modal-open">
	<div class="modal-box flex h-[50vh] w-11/12 max-w-5xl flex-col gap-3">
		<h2 class="text-2xl font-bold">Edit File</h2>

		<select class="select w-full" bind:value={selectedFileId}>
			<option disabled selected value={0}>Pick a file ...</option>
			{#each fileCollection as collection}
				<option value={collection.id}>{collection.name}</option>
			{/each}
		</select>

		<input
			type="text"
			placeholder="File Name"
			class="input w-full"
			bind:value={fileName}
		/>

		<fieldset class="fieldset">
			<textarea
				class="textarea h-24 w-full"
				placeholder="File Description"
				bind:value={fileDescription}
			></textarea>
			<div class="label">Optional</div>
		</fieldset>

		<div class="modal-action">
			<button class="btn btn-primary" onclick={handleEditFile}>Edit</button>
			<button class="btn btn-error" onclick={handleClose}>Cancel</button>
		</div>
	</div>
</div>
