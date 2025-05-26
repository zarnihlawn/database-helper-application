<script lang="ts">
	import type { QueryFileInterface } from "$lib/model/interface/schema.interface";
	import { invoke } from "@tauri-apps/api/core";

  let { fileCollection, onClose } = $props<{
    fileCollection: QueryFileInterface[];
    onClose: () => void;
  }>();

  let handleClose = () => {
    onClose();
  }

  let selectedFileId = $state(0);
  let selectedFileName = $state('');
  let fieldName = $state('');

  $effect(() => {
    const selected = fileCollection.find((collection: QueryFileInterface) => collection.id === selectedFileId);
    if (selected) {
      selectedFileName = selected.name;
    } else {
      selectedFileName = ''; // Reset if no file is selected or found
    }
  });
  
  async function handleDeleteFile() {
    if (selectedFileId && selectedFileName && fieldName === selectedFileName) {
      try {
        await invoke('delete_file_database', {
          id: selectedFileId
        });
        handleClose();
      } catch (error) {
        console.error("Error deleting file:", error);
      }
    } else {
      console.warn("Deletion aborted: Please select a file and retype the name correctly to confirm.");
    }
  }
</script>

<div class="modal modal-open">
	<div class="modal-box flex h-[50vh] w-11/12 max-w-5xl flex-col gap-3">
		<h2 class="text-2xl font-bold">Delete File</h2>

    <p>
      Select file 
    </p>
		<select class="select w-full" bind:value={selectedFileId}>
      <option disabled selected value={0}>Pick a file ...</option> {#each fileCollection as collection}
        <option value={collection.id}>{collection.name}</option>
      {/each}
    </select>

    <p>Retype the file name to confirm deleting: </p>
		<input
			type="text"
			placeholder="Type here"
			class="input w-full"
			bind:value={fieldName}
		/>

	

		<div class="modal-action">
      <button
        class="btn btn-primary"
        onclick={handleDeleteFile}
        disabled={!selectedFileId || fieldName !== selectedFileName}
      >
        Delete
      </button>
      <button class="btn btn-error" onclick={handleClose}>Cancel</button>
    </div>
	</div>
</div>
