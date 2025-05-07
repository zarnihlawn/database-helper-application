<script lang="ts">
	import CodingWorkspace from '$lib/component/local/app/workspace-dashboard/CodingWorkspace.svelte';
	import FileTreeWorkspace from '$lib/component/local/app/workspace-dashboard/FileTreeWorkspace.svelte';
	import MenuWorkspace from '$lib/component/local/app/workspace-dashboard/MenuWorkspace.svelte';
	import type {
		DatabaseConnectionInterface,
		DatasourceInterface
	} from '$lib/model/interface/schema.interface';
	import { invoke } from '@tauri-apps/api/core';

	let datasource: DatasourceInterface[] = [];
	let databaseConnection: DatabaseConnectionInterface[] = [];

	invoke<DatasourceInterface[]>('get_datasource').then((data) => {
		datasource = data;
	});

	invoke<DatabaseConnectionInterface[]>('get_database_connection').then(
		(data) => {
			databaseConnection = data;
		}
	);
</script>

<main class="m-2 flex gap-5">
	<div class="flex max-w-64 flex-col gap-4">
		<MenuWorkspace {datasource} {databaseConnection} />
		<FileTreeWorkspace {databaseConnection} {datasource} />
	</div>
	<div class="flex flex-1">
		<CodingWorkspace {datasource} />
	</div>
</main>
