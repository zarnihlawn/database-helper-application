<script lang="ts">
	import CodingWorkspace from '$lib/component/local/app/workspace-dashboard/CodingWorkspace.svelte';
	import WorkspaceFileTree from '$lib/component/local/app/workspace-dashboard/FileTreeWorkspace.svelte';
	import WorkspaceMenu from '$lib/component/local/app/workspace-dashboard/MenuWorkspace.svelte';
	import type { DatasourceInterface } from '$lib/model/interface/schema.interface';
	import { invoke } from '@tauri-apps/api/core';

	let datasource: DatasourceInterface[] = [];

	invoke<DatasourceInterface[]>('get_datasource').then((data) => {
		datasource = data;
	});
</script>

<main class="m-2 flex gap-5">
	<div class="flex max-w-64 flex-col gap-4">
		<WorkspaceMenu {datasource} />
		<WorkspaceFileTree />
	</div>
	<div class="flex flex-1">
		<CodingWorkspace />
	</div>
</main>
