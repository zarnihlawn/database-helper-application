<script lang="ts">
	import type { RouteInterface } from '$lib/model/interface/route.interface';
	import { goToRoute } from '$lib/util/router.util';

	let { data, children } = $props<{
		data: {
			routeSettingsDashboard: RouteInterface[];
		};
	}>();

	let currentRoute = $state(
		data && data.routeSettingsDashboard && data.routeSettingsDashboard.length > 0
			? data.routeSettingsDashboard[0].url
			: ''
	);

</script>

<div class="tabs tabs-lift">
	{#each data.routeSettingsDashboard as route}
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
