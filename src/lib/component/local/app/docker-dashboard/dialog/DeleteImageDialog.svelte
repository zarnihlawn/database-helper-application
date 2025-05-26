<script lang="ts">
	import {
		deleteDockerContainer,
		deleteDockerImage
	} from '$lib/tool/docker.tool';

	let { imageId, imageName, onClose } = $props<{
		imageId: string;
		imageName: string;
		onClose: () => void;
	}>();

	let fieldInput = $state('');

	function handleContainerDelete() {
		deleteDockerImage(imageId);
		handleClose();
	}

	function handleClose() {
		onClose();
	}
</script>

<div class="modal modal-open">
	<div class="modal-box">
		<h3 class="text-lg font-bold">Delete Image: {imageName}</h3>

		<p class="label-text mb-5">
			Type in '{imageName}' to confirm deleting. Note: Only unused image can be
			deleted.
		</p>
		<div class="form-control w-full">
			<input
				type="text"
				bind:value={fieldInput}
				placeholder="Enter container name"
				class="input input-bordered w-full"
			/>
		</div>

		<p></p>

		<div class="modal-action">
			<button
				class="btn btn-error"
				class:btn-disabled={imageName !== fieldInput}
				onclick={handleContainerDelete}>Delete</button
			>
			<button class="btn btn-primary" onclick={handleClose}>Cancel</button>
		</div>
	</div>
</div>
