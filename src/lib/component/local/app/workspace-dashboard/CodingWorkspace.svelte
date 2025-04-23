<script lang="ts">
	import { MongoDBSvg } from '$lib/asset/image/svg/mongodb-svg';
	import { MySQLSvg } from '$lib/asset/image/svg/mysql-svg';
	import { PostgreSQLSvg } from '$lib/asset/image/svg/postgresql-svg';
	import { SQLiteSvg } from '$lib/asset/image/svg/sqlite-svg';
	import type {
		ContentTypeInterface,
		DatasourceInterface
	} from '$lib/model/interface/schema.interface';
	import QueryBlockActionsCodingWorkspace from './component/QueryBlockActionsCodingWorkspace.svelte';
	import { MariaDBSvg } from '$lib/asset/image/svg/mariadb-svg';
	import { MicrosoftSqlServerSvg } from '$lib/asset/image/svg/microsoft-sql-server-svg';

	import { PlusSvg } from '$lib/asset/image/svg/plus-svg';
	import { selectedDatabaseState } from '$lib/store/state/selectedDatabase.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import NewFileDialogWorkspace from '$lib/component/local/app/workspace-dashboard/dialog/NewFileDialogWorkspace.svelte';
	import { onMount } from 'svelte';
	import { EditSvg } from '$lib/asset/image/svg/edit-svg';
	import { DeleteSvg } from '$lib/asset/image/svg/delete-svg';

	let selectedDatabase = $derived(selectedDatabaseState.selectedDatabase);

	type QueryFile = {
		id: number;
		name: string;
		description: string;
	};

	type QueryFileCollection = {
		id: number;
		name: string;
		description: string;
	};

	let showNewFileDialog = $state(false);
	let fileCollection = $state<QueryFileCollection[]>([]);
	let selectedFile = $state<QueryFile | null>(null);

	async function getFileCollection() {
		fileCollection = await invoke<QueryFileCollection[]>(
			'get_file_collection',
			{
				databaseConnectionId: selectedDatabase?.id
			}
		);
		console.log('File Collection', fileCollection);
	}

	function handleAddNewFile() {
		showNewFileDialog = true;
	}

	function handleFileSelect(event: Event) {
		const select = event.target as HTMLSelectElement;
		const selectedCollection = fileCollection.find(
			(collection) => collection.name === select.value
		);
		selectedFile = selectedCollection || null;
	}

	let { datasource } = $props<{ datasource: DatasourceInterface[] }>();

	let contentType: ContentTypeInterface[] = $state([]);

	const svgMap: Record<string, string> = {
		SQLite: SQLiteSvg('size-20'),
		MySQL: MySQLSvg('size-20'),
		PostgreSQL: PostgreSQLSvg('size-20'),
		MongoDB: MongoDBSvg('size-20'),
		MariaDB: MariaDBSvg('size-20'),
		MSSQL: MicrosoftSqlServerSvg('size-20')
	};

	onMount(() => {
		getContentType();
		getFileCollection();
	});

	async function getContentType() {
		contentType = await invoke('get_content_type');
	}
</script>

<main class=" w-full">
	{#if selectedDatabase}
		{#each datasource as datasource}
			{#if selectedDatabase.datasource_id === datasource.id}
				<div class="card bg-base-200 flex flex-col gap-2 px-5 py-3 shadow-sm">
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
					<section class="flex w-full gap-2">
						<section class=" flex w-full gap-2">
							<button class="btn btn-primary" onclick={handleAddNewFile}>
								{@html PlusSvg('size-4')} New File
							</button>
							<button class="btn btn-warning">
								{@html EditSvg('size-4')} Edit File
							</button>
							<button class="btn btn-error">
								{@html DeleteSvg('size-4')} Delete File
							</button>
						</section>
						<section class="flex w-full">
							<select class="select w-full" onchange={handleFileSelect}>
								<option disabled selected>Pick a file ...</option>
								{#each fileCollection as collection}
									<option>{collection.name}</option>
								{/each}
							</select>
						</section>
					</section>
					{#if showNewFileDialog}
						<NewFileDialogWorkspace
							onClose={() => (showNewFileDialog = false)}
						/>
					{/if}
				</div>
			{/if}
		{/each}

		{#if selectedFile}
			<QueryBlockActionsCodingWorkspace {contentType} />
		{/if}
	{:else}
		No Database Has Been Selected ...
	{/if}
</main>
