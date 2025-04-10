<script lang="ts">
	import { BluemarkSvg } from '$lib/asset/image/svg/bluemark-svg';
	import { PullSvg } from '$lib/asset/image/svg/pull-svg';
	import { SearchSvg } from '$lib/asset/image/svg/search-svg';
	import type { DockerHubImageInterface } from '$lib/model/interface/docker.interface';
	import { catchAllDockerImages, pullDockerImage, searchDockerImage } from '$lib/tool/docker.tool';

	let searchText = $state('');
	let searchResults = $state<DockerHubImageInterface[]>([]);
	let isSearching = $state(false);

	let pullingImage = $state<string | null>(null);
	let pullError = $state<string | null>(null);

	$effect(() => {
		if (searchText.length > 0) {
			isSearching = true;
			searchDockerImage(searchText)
				.then((results) => {
					searchResults = results;
					console.log('Search Results', searchResults);
				})
				.catch((error) => {
					console.error('Search failed:', error);
				})
				.finally(() => {
					isSearching = false;
				});
		} else {
			searchResults = [];
		}
	});

	async function handlePullImage(imageName: string) {
		pullingImage = imageName;
		pullError = null;
		try {
			await pullDockerImage(imageName);
		} catch (error) {
			pullError = error instanceof Error ? error.message : 'Failed to pull image';
		} finally {
			pullingImage = null;
		}
	}
</script>

<section>
	<label class="input w-full">
		{@html SearchSvg}
		<input
			type="search"
			bind:value={searchText}
			placeholder="Search Docker Hub images..."
			class="grow focus:ring-0"
		/>
	</label>

	{#if isSearching}
		<div class="my-10 flex h-full items-center justify-center">
			<div class="loading loading-spinner loading-lg"></div>
		</div>
	{:else if searchResults.length > 0}
		<ul class="list bg-base-100 rounded-box shadow-md">
			{#each searchResults as result, index}
				<li class="list-row">
					<div class="text-4xl font-thin tabular-nums opacity-30">
						{(index + 1).toString().padStart(2, '0')}
					</div>
					<div class="list-col-grow">
						<h2>
							{result.name}
							{#if result.official}
								<div
									class="tooltip tooltip-info text-info text-left"
									data-tip="Certified(Official) Image"
								>
									{@html BluemarkSvg}
								</div>
							{/if}
						</h2>
						<div class="text-xs font-semibold opacity-60">{result.description}</div>
						<div class="tooltip tooltip-right">
							<div class="tooltip-content text-left">
								<div class="flex flex-col gap-2">
									<p>Name: {result.name}</p>
									<p>Description: {result.description}</p>
									<p>Star Count: {result.star_count}</p>
									<p>Pull Count: {result.pull_count}</p>
									<p>Official: {result.official}</p>
								</div>
							</div>
							<div class="link-info cursor-pointer text-xs font-semibold opacity-60">
								More Info ...
							</div>
						</div>
					</div>
					<button
						class="btn btn-square btn-ghost text-success"
						onclick={() => handlePullImage(result.name)}
						disabled={pullingImage !== null}
					>
						<div class="tooltip tooltip-left tooltip-success" data-tip="Pull Image">
							{#if pullingImage === result.name}
								<span class="loading loading-spinner loading-xs"></span>
							{:else}
								{@html PullSvg('size-7')}
							{/if}
						</div>
					</button>
				</li>
			{/each}
		</ul>
	{:else}
		<p class="text-error m-4">No images found</p>
	{/if}

	{#if pullError}
		<div class="alert alert-error mt-4">
			<span>{pullError}</span>
		</div>
	{/if}
</section>
