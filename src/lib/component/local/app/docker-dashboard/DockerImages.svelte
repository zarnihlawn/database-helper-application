<script lang="ts">
	import { BuildSvg } from '$lib/asset/image/svg/build-svg';
	import { DeleteSvg } from '$lib/asset/image/svg/delete-svg';
	import { MenuSvg } from '$lib/asset/image/svg/menu-svg';
	import {
		dockerContainersState,
		dockerImagesState
	} from '$lib/store/state/docker.state.svelte';
	import { deleteDockerImage } from '$lib/tool/docker.tool';
	import { goToRoute } from '$lib/util/router.util';
	import DeleteImageDialog from './dialog/DeleteImageDialog.svelte';
	import DockerBuildContainerDialog from './dialog/DockerBuildContainerDialog.svelte';

	let showBuildDialog = $state(false);
	let selectedImage = $state<string | null>(null);

	let deleteImageDialog = $state(false);
	let deleteImageId = $state('');
	let deleteImageName = $state('');

	function handleDeleteImageDialog(imageId: string, imageName: string) {
		deleteImageId = imageId;
		deleteImageName = imageName;
		deleteImageDialog = true;
	}
	function handleDialogClose() {
		deleteImageDialog = false;
	}

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
	function goToDocumentationPage() {
		goToRoute('/app/documentation-dashboard/user-manual-documentation');
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
									<span class="indicator-item status status-success mb-1 ml-1"
									></span>
								</div>
							{:else}
								<div
									class="tooltip tooltip-error"
									data-tip="{image.repository} has never been used."
								>
									<span class="indicator-item status status-error mb-1 ml-1"
									></span>
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
						<div
							class="link-info cursor-pointer text-xs font-semibold opacity-60"
						>
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
					onclick={() =>
						handleDeleteImageDialog(image.image_id, image.repository)}
				>
					<div
						class="tooltip tooltip-left tooltip-error"
						data-tip="Delete Image"
					>
						{@html DeleteSvg('size-7')}
					</div>
				</button>
				<button
					class="btn btn-square btn-ghost text-info"
					onclick={goToDocumentationPage}
				>
					<div
						class="tooltip tooltip-left tooltip-info"
						data-tip="See More Actions"
					>
						{@html MenuSvg}
					</div>
				</button>
			</li>
		{/each}
	{:else}
		<p class="text-error m-4">No Images found Or Docker is not running</p>
	{/if}
</section>

{#if showBuildDialog && selectedImage}
	<DockerBuildContainerDialog
		image={selectedImage}
		onSuccess={handleBuildSuccess}
		onClose={handleBuildClose}
	/>
{/if}

{#if deleteImageDialog}
	<DeleteImageDialog
		imageId={deleteImageId}
		imageName={deleteImageName}
		onClose={handleDialogClose}
	/>
{/if}
