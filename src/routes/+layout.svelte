<script lang="ts">
	import '../app.css';
	import AppToolBar from '$lib/component/global/AppToolBar.svelte';
	import AppDock from '$lib/component/global/AppDock.svelte';
	import DockerStalker from '$lib/store/stalker/DockerStalker.svelte';
	import { getThemeFromLocalStorage } from '$lib/store/safe/local-storage/theme.local-storage.svelte';
	import { getFontFromLocalStorage } from '$lib/store/safe/local-storage/font.local-storage.svelte';
	import { getUserFromLocalStorage } from '$lib/store/safe/local-storage/user.local-storage.svelte';
	// import { RenderScan } from 'svelte-render-scan';

	let { children } = $props();

	// Check Update

	// Local Storage
	getUserFromLocalStorage();
	getThemeFromLocalStorage();
	getFontFromLocalStorage();
</script>

<!-- Render Scan -->
<!-- <RenderScan /> -->

<!-- Stalkers -->
<DockerStalker />

<div class="app">
	<div class="tool-bar">
		<AppToolBar />
	</div>
	<main>
		{@render children()}
	</main>
	<div class="dock">
		<AppDock />
	</div>
</div>

<style>
	.app {
		display: flex;
		flex-direction: column;
		height: 100vh;
		position: relative;
	}
	.tool-bar {
		position: sticky;
		top: 0;
		z-index: 10;
		background: var(--background);
	}
	main {
		flex: 1;
		overflow-y: auto;
		position: relative;
	}
	.dock {
		position: sticky;
		bottom: 0;
		z-index: 10;
		background: var(--background);
	}
</style>
