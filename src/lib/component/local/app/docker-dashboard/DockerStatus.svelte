<script lang="ts">
	import { dockerStatusState } from '$lib/store/state/docker.state.svelte';
	import { restartDocker, startDocker, stopDocker } from '$lib/tool/docker.tool';

	let isStarting = $state(false);
	let isStopping = $state(false);
	let isRestarting = $state(false);

	async function handleStartDocker() {
		isStarting = true;
		await startDocker();
		isStarting = false;
	}

	async function handleStopDocker() {
		isStopping = true;
		await stopDocker();
		isStopping = false;
	}

	async function handleRestartDocker() {
		isRestarting = true;
		await restartDocker();
		isRestarting = false;
	}
</script>

<section class="card bg-base-100 w-full shadow-sm">
	<div class="card-body">
		<h2 class="card-title">
			Docker status
			{#if dockerStatusState.status}
				<span class="indicator-item status status-success"></span>
			{:else}
				<span class="indicator-item status status-error"></span>
			{/if}
		</h2>
		<p>
			Docker is {dockerStatusState.status
				? 'running in the background'
				: 'not running in the background or not installed. Please check your installation.'}
		</p>
		<div class="card-actions justify-end">
			<button class="btn" onclick={handleRestartDocker} disabled={isRestarting}>
				{#if isRestarting}
					<span class="loading loading-spinner text-neutral"></span>
				{:else}
					Restart Docker
				{/if}
			</button>

			{#if dockerStatusState.status}
				<button class="btn btn-error" onclick={handleStopDocker} disabled={isStopping}>
					{#if isStopping}
						<span class="loading loading-spinner text-error"></span>
					{:else}
						Stop Docker
					{/if}
				</button>
			{:else}
				<button class="btn btn-success" onclick={handleStartDocker} disabled={isStarting}>
					{#if isStarting}
						<span class="loading loading-spinner text-success"></span>
					{:else}
						Start Docker
					{/if}
				</button>
			{/if}
		</div>
	</div>
</section>
