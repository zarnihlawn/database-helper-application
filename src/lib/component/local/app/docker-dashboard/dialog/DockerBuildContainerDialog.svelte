<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	let { image, onSuccess, onClose } = $props<{
		image: string;
		onSuccess: (message: string) => void;
		onClose: () => void;
	}>();

	let containerName = $state('');
	let isBuilding = $state(false);
	let error = $state<string | null>(null);

	async function handleBuild() {
		if (!containerName.trim()) {
			error = 'Container name is required';
			return;
		}

		isBuilding = true;
		error = null;

		try {
			const command = `docker run -d --name ${containerName} ${image}`;
			const output = await invoke('execute_shell_command', { command });
			if (output) {
				onSuccess('Container built successfully');
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to build container';
		} finally {
			isBuilding = false;
		}
	}

	function handleClose() {
		onClose();
	}
</script>

<div class="modal modal-open">
	<div class="modal-box">
		<h3 class="text-lg font-bold">Build Container</h3>
		<div class="py-4">
			<p class="text-sm opacity-70">
				Building container from image: <span class="font-semibold">{image}</span>
			</p>
		</div>

		<div class="form-control w-full">
			<span class="label-text">Container Name</span>
			<input
				type="text"
				bind:value={containerName}
				placeholder="Enter container name"
				class="input input-bordered w-full"
			/>
		</div>

		{#if error}
			<div class="alert alert-error mt-4">
				<span>{error}</span>
			</div>
		{/if}

		<div class="modal-action">
			<button class="btn" onclick={handleClose}>Cancel</button>
			<button class="btn btn-primary" onclick={handleBuild} disabled={isBuilding}>
				{#if isBuilding}
					<span class="loading loading-spinner loading-sm"></span>
					Building...
				{:else}
					Build Container
				{/if}
			</button>
		</div>
	</div>
</div>
