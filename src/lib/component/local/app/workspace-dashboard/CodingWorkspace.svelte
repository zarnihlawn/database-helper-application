<script lang="ts">
	import { MongoDBSvg } from '$lib/asset/image/svg/mongodb-svg';
	import { MySQLSvg } from '$lib/asset/image/svg/mysql-svg';
	import { OracleSvg } from '$lib/asset/image/svg/oracle-svg';
	import { PostgreSQLSvg } from '$lib/asset/image/svg/postgresql-svg';
	import { SQLiteSvg } from '$lib/asset/image/svg/sqlite-svg';
	import type {
		ContentTypeInterface,
		DatasourceInterface
	} from '$lib/model/interface/schema.interface';
	import { selectedDatabaseState } from '$lib/store/state/selectedDatabase.svelte';
	import { onMount } from 'svelte';
	import QueryBlockActionsCodingWorkspace from './component/QueryBlockActionsCodingWorkspace.svelte';
	import { invoke } from '@tauri-apps/api/core';

	let { datasource } = $props<{ datasource: DatasourceInterface[] }>();

	let selectedDatabase = $derived(selectedDatabaseState.selectedDatabase);
	let contentType: ContentTypeInterface[] = $state([]);

	const svgMap: Record<string, string> = {
		SQLite: SQLiteSvg('size-20'),
		MySQL: MySQLSvg('size-20'),
		PostgreSQL: PostgreSQLSvg('size-20'),
		MongoDB: MongoDBSvg('size-20'),
		Oracle: OracleSvg('size-20')
	};

	onMount(() => {
		getContentType();
	});

	async function getContentType() {
		contentType = await invoke('get_content_type');
	}
</script>

<main class=" w-full">
	{#if selectedDatabase}
		{#each datasource as datasource}
			{#if selectedDatabase.datasource_id === datasource.id}
				<div class="card bg-base-200 px-5 py-3 shadow-sm">
					<div class="flex gap-10">
						{@html svgMap[datasource.name]}
						<div class="flex flex-col">
							<div>Connection Name : {selectedDatabase.connection_name}</div>
							<div class="text-sm opacity-60">
								Database : {datasource.name}
							</div>
							<div class="text-sm opacity-60">
								URL : {selectedDatabase.url}
							</div>
						</div>
					</div>
				</div>
			{/if}
		{/each}

		<QueryBlockActionsCodingWorkspace {contentType} />
	{:else}
		No Database Has Been Selected ...
	{/if}
</main>
