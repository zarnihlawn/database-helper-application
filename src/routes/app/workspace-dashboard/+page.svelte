<script lang="ts">
	import WorkspaceFileTree from '$lib/component/local/app/workspace-dashboard/FileTreeWorkspace.svelte';
	import WorkspaceMenu from '$lib/component/local/app/workspace-dashboard/MenuWorkspace.svelte';
	import type {
		datasourceAuthenticationTypeInterface,
		datasourceInterface
	} from '$lib/model/interface/schema.interface';
	import { invoke } from '@tauri-apps/api/core';

	let datasource: datasourceInterface[] = [];
	let datasourceAuthenticationType: datasourceAuthenticationTypeInterface[] = [];

	// invoke<datasourceInterface[]>('get_datasource').then((data) => {
	// 	datasource = data;
	// });

</script>

<main class="m-2">
	<div class="flex max-w-64 flex-col gap-4">
		<WorkspaceMenu {datasource}  />
		<WorkspaceFileTree />
	</div>
</main>
