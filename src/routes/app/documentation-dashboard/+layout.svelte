<script lang="ts">
	import type { RouteInterface } from '$lib/model/interface/route.interface';
	import { goToRoute } from '$lib/util/router.util';

	const { data, children } = $props<{
		data: {
			routeDocumentationDashboard: RouteInterface[];
		};
	}>();

	let currentRoute = $state(
		data && data.routeDocumentationDashboard && data.routeDocumentationDashboard.length > 0
			? data.routeDocumentationDashboard[0].url
			: ''
	);
</script>

<div class="tabs tabs-lift">
	{#each data.routeDocumentationDashboard as route}
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
		<div class="tab-content bg-base-100 border-base-300 p-6">
			{@render children()}
		</div>
	{/each}
</div>
