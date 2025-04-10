<script lang="ts">
	import { BuildSvg } from '$lib/asset/image/svg/build-svg';
	import { DeleteSvg } from '$lib/asset/image/svg/delete-svg';
	import { MenuSvg } from '$lib/asset/image/svg/menu-svg';
	import { dockerContainersState, dockerImagesState } from '$lib/store/state/docker.state.svelte';
	import { deleteDockerImage } from '$lib/tool/docker.tool';
	import DockerBuildContainerDialog from './dialog/DockerBuildContainerDialog.svelte';

	let showBuildDialog = $state(false);
	let selectedImage = $state<string | null>(null);

	function handleBuildClick(image: string) {
		selectedImage = image;
		showBuildDialog = true;
	}

	function handleBuildSuccess(message: string) {
		showBuildDialog = false;
		selectedImage = null;
	}

	function handleBuildClose() {
		showBuildDialog = false;
		selectedImage = null;
	}
</script>

<section>
	{#if dockerImagesState.images.length > 0}
		{#each dockerImagesState.images as image, index}
			<li class="list-row">
				<div class="text-4xl font-thin tabular-nums opacity-30">
					{(index + 1).toString().padStart(2, '0')}
				</div>

				<div class="list-col-grow">
					<div class="flex items-center gap-2">
						<h2>
							{image.repository}
							{#if dockerContainersState.containers.some( (container) => container.image.includes(image.repository) )}
								<div
									class="tooltip tooltip-success"
									data-tip="{image.repository} is being used by at least one container."
								>
									<span class="indicator-item status status-success mb-1 ml-1"></span>
								</div>
							{:else}
								<div
									class="tooltip tooltip-error"
									data-tip="{image.repository} has never been used."
								>
									<span class="indicator-item status status-error mb-1 ml-1"></span>
								</div>
							{/if}
						</h2>
					</div>
					<div class="text-xs font-semibold opacity-60">Size: {image.size}</div>
					<div class="tooltip tooltip-right">
						<div class="tooltip-content text-left">
							<div class="">
								<p>Repository: {image.repository}</p>
								<p>Tag: {image.tag}</p>
								<p>Image ID: {image.image_id}</p>
								<p>Created: {image.created}</p>
								<p>Size: {image.size}</p>
							</div>
						</div>
						<div class="link-info cursor-pointer text-xs font-semibold opacity-60">
							More Info ...
						</div>
					</div>
				</div>
				<button
					class="btn btn-square btn-ghost text-success"
					onclick={() => handleBuildClick(image.repository)}
				>
					<div
						class="tooltip tooltip-left tooltip-success"
						data-tip="Build Container From This Image"
					>
						{@html BuildSvg}
					</div>
				</button>
				<button
					class="btn btn-square btn-ghost text-error"
					onclick={() => deleteDockerImage(image.image_id)}
				>
					<div class="tooltip tooltip-left tooltip-error" data-tip="Delete Image">
						{@html DeleteSvg}
					</div>
				</button>
				<button class="btn btn-square btn-ghost text-info">
					<div class="tooltip tooltip-left tooltip-info" data-tip="See More Actions">
						{@html MenuSvg}
					</div>
				</button>
			</li>
		{/each}
	{:else}
		<div class="tab-content bg-base-100 border-base-300">
			<p class="text-error m-4">No Images found Or Docker is not running</p>
		</div>
	{/if}
</section>

{#if showBuildDialog && selectedImage}
	<DockerBuildContainerDialog
		image={selectedImage}
		onSuccess={handleBuildSuccess}
		onClose={handleBuildClose}
	/>
{/if}
