<script lang="ts">
  import { deleteDockerContainer } from "$lib/tool/docker.tool";

  let {containerId, containerName, onClose} = $props<{
    containerId: string;
    containerName: string;
    onClose: () => void;
  }>();

  let fieldInput = $state('');

  function handleContainerDelete() {
    deleteDockerContainer(containerId);
    handleClose();
  }

  function handleClose() {
    onClose();
  }
</script>

<div class="modal modal-open">
	<div class="modal-box">
		<h3 class="text-lg font-bold">Delete Container: {containerName}</h3>
		

    <p class="label-text mb-5">Type in '{containerName}' to confirm deleting.</p>
		<div class="form-control w-full">
			<input
				type="text"
        bind:value={fieldInput}
				placeholder="Enter container name"
				class="input input-bordered w-full"
			/>
		</div>
		<div class="modal-action">
			<button class="btn btn-error" class:btn-disabled={containerName !== fieldInput} onclick={handleContainerDelete}>Delete</button>
			<button class="btn btn-primary" onclick={handleClose}>Cancel</button>
		</div>
	</div>
</div>
