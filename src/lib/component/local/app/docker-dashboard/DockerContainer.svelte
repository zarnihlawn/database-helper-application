<script lang="ts">
	import { DeleteSvg } from '$lib/asset/image/svg/delete-svg';
	import { MenuSvg } from '$lib/asset/image/svg/menu-svg';
	import { PlaySvg } from '$lib/asset/image/svg/play-svg';
	import { StopSvg } from '$lib/asset/image/svg/stop-svg';
	import { dockerContainersState } from '$lib/store/state/docker.state.svelte';
	import {
		deleteDockerContainer,
		startDockerContainer,
		stopDockerContainer
	} from '$lib/tool/docker.tool';

</script>

<section>
	{#if dockerContainersState.containers.length > 0}
		{#each dockerContainersState.containers as container, index}
			<li class="list-row">
				<div class="text-4xl font-thin tabular-nums opacity-30">
					{(index + 1).toString().padStart(2, '0')}
				</div>

				<div class="list-col-grow">
					<h2>
						{container.name}
						{#if container.status.startsWith('Exited')}
							<div class="tooltip tooltip-error" data-tip="{container.name} is stopped.">
								<span class="indicator-item status status-error mb-1 ml-1"></span>
							</div>
						{:else}
							<div class="tooltip tooltip-success" data-tip="{container.name} is running.">
								<span class="indicator-item status status-success mb-1 ml-1"></span>
							</div>
						{/if}
					</h2>

					<div class="text-xs font-semibold opacity-60">
						Base Image: {container.image}
					</div>
					<div class="tooltip tooltip-right">
						<div class="tooltip-content text-left">
							<p>Name: {container.name}</p>
							<p>Image: {container.image}</p>
							<p>Status: {container.status}</p>
							<p>State: {container.state}</p>
							<p>Created: {container.created}</p>
						</div>
						<div class="link-info cursor-pointer text-xs font-semibold opacity-60">
							More Info ...
						</div>
					</div>
				</div>
				{#if container.status.startsWith('Exited')}
					<button
						class="btn btn-square btn-ghost text-success"
						onclick={() => startDockerContainer(container.id)}
					>
						<div class="tooltip tooltip-left tooltip-success" data-tip="Start Container">
							{@html PlaySvg}
						</div>
					</button>
				{/if}
				{#if container.status.startsWith('Up')}
					<button
						class="btn btn-square btn-ghost text-warning"
						onclick={() => stopDockerContainer(container.id)}
					>
						<div class="tooltip tooltip-left tooltip-warning" data-tip="Stop Container">
							{@html StopSvg}
						</div>
					</button>
				{/if}
				<button
					class="btn btn-square btn-ghost text-error"
					onclick={() => deleteDockerContainer(container.id)}
				>
					<div class="tooltip tooltip-left tooltip-error" data-tip="Delete Container">
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
		<p class="text-error m-4">No Containers found Or Docker is not running</p>
	{/if}
</section>
