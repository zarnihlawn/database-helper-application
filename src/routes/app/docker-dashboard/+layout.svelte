<script lang="ts">
	import DockerStatus from '$lib/component/local/app/docker-dashboard/DockerStatus.svelte';
	import type { RouteInterface } from '$lib/model/interface/route.interface';
	import { goToRoute } from '$lib/util/router.util';

	let { data, children } = $props<{
		data: {
			routeDockerDashboard: RouteInterface[];
		};
	}>();

	let currentRoute = $state(
		data && data.routeDockerDashboard && data.routeDockerDashboard.length > 0
			? data.routeDockerDashboard[0].url
			: ''
	);
</script>

<main class="flex flex-col gap-5 px-5 py-5">
	<DockerStatus />
	<div class="tabs tabs-lift">
		{#each data.routeDockerDashboard as route}
			<button
				type="button"
				class="tab"
				class:tab-active={currentRoute === route.url}
				onclick={() => {
					goToRoute(route.url);
					currentRoute = route.url;
				}}
			>
				<input type="radio" name="my_tabs_4" checked={currentRoute === route.url} class="hidden" />
				{@html route.icon}
				{route.name}
			</button>
			<div class="tab-content bg-base-100 border-base-300">
				<ul class="list bg-base-100 rounded-box shadow-md">
					{@render children()}
				</ul>
			</div>
		{/each}
	</div>
</main>
