<script lang="ts">
	import { selectedDatabaseState } from '$lib/store/state/selectedDatabase.svelte';
	import { invoke } from '@tauri-apps/api/core';

	let fileName = $state('');
	let fileDescription = $state('');
	let databaseConnectionId = $derived(
		selectedDatabaseState.selectedDatabase?.id
	);

	let { onClose } = $props<{
		onClose: () => void;
	}>();

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
		<h2 class="text-2xl font-bold">New File</h2>

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
