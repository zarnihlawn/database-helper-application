<script lang="ts">
	import { DockerSvg } from '$lib/asset/image/svg/docker-svg';
	import { DocumentSvg } from '$lib/asset/image/svg/document-svg';
	import { WorkspaceSvg } from '$lib/asset/image/svg/workspace-svg';
	import { SettingsSvg } from '$lib/asset/image/svg/settings-svg';

	import { goToRoute } from '$lib/util/router.util';

	import { dockerStatusState } from '$lib/store/state/docker.state.svelte';

	let activeDock = $state(0);

	function navigateToThePage(index: number, route: string) {
		activeDock = index;
		goToRoute(route);
	}
</script>

<div class="main">
	<div class="dock dock-md">
		<button
			onclick={() => navigateToThePage(0, '/app/documentation-dashboard')}
			class={activeDock === 0 ? 'dock-active' : ''}
		>
			<div class="tooltip" data-tip="Documentation">
				{@html DocumentSvg}
			</div>
		</button>

		<button
			onclick={() => navigateToThePage(1, '/app/docker-dashboard')}
			class={activeDock === 1 ? 'dock-active' : ''}
		>
			{#if dockerStatusState.status}
				<div class="tooltip" data-tip="Docker Dashboard">
					<div class="indicator mt-2">
						<span class="indicator-item status status-success"></span>
						{@html DockerSvg}
					</div>
				</div>
			{:else}
				<div class="tooltip" data-tip="Docker Dashboard">
					<div class="indicator mt-2">
						<span class="indicator-item status status-error"></span>
						{@html DockerSvg}
					</div>
				</div>
			{/if}
		</button>

		<button
			onclick={() => navigateToThePage(2, '/app/workspace-dashboard')}
			class={activeDock === 2 ? 'dock-active' : ''}
		>
			<div class="tooltip" data-tip="Workspace">
				{@html WorkspaceSvg}
			</div>
		</button>

		<button
			onclick={() => navigateToThePage(3, '/app/settings-dashboard')}
			class={activeDock === 3 ? 'dock-active' : ''}
		>
			<div class="tooltip mr-1" data-tip="Settings">
				{@html SettingsSvg}
			</div>
		</button>
	</div>
</div>
