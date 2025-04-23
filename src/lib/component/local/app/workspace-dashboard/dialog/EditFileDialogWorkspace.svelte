<script lang="ts">
	import type { QueryFileCollection } from '$lib/model/type/query-file-collection.type';
	import { selectedDatabaseState } from '$lib/store/state/selectedDatabase.svelte';
	import { invoke } from '@tauri-apps/api/core';

	let fileName = $state('');
	let fileDescription = $state('');
	let databaseConnectionId = $derived(
		selectedDatabaseState.selectedDatabase?.id
	);

	let { fileCollection, onClose } = $props<{
		fileCollection: QueryFileCollection[];
		onClose: () => void;
	}>();

	let selectedFile = $state<QueryFileCollection | null>(null);

	function handleClose() {
		onClose();
	}

	async function handleCreateFile() {
		if (fileName === '') {
			return;
		} else {
			try {
				const fileId = await invoke('create_file_for_database', {
					name: fileName,
					description: fileDescription
				});
				try {
					await invoke('store_file_with_database', {
						databaseConnectionId: databaseConnectionId,
						queryFileId: fileId
					});
				} catch (error) {
					console.error(error);
				}
			} catch (error) {
				console.error(error);
			}
		}
	}
</script>

<div class="modal modal-open">
	<div class="modal-box flex h-[50vh] w-11/12 max-w-5xl flex-col gap-3">
		<h2 class="text-2xl font-bold">Edit File</h2>

		<select class="select w-full" bind:value={selectedFile}>
			<option disabled selected>Pick a file ...</option>
			{#each fileCollection as collection}
				<option>{collection.name}</option>
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
			<button class="btn btn-primary" onclick={handleCreateFile}>Create</button>
			<button class="btn btn-error" onclick={handleClose}>Cancel</button>
		</div>
	</div>
</div>
