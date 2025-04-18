<script lang="ts">
	import { ColumnSvg } from '$lib/asset/image/svg/column-svg';
	import { MongoDBSvg } from '$lib/asset/image/svg/mongodb-svg';
	import { MySQLSvg } from '$lib/asset/image/svg/mysql-svg';
	import { OracleSvg } from '$lib/asset/image/svg/oracle-svg';
	import { PostgreSQLSvg } from '$lib/asset/image/svg/postgresql-svg';
	import { SQLiteSvg } from '$lib/asset/image/svg/sqlite-svg';
	import { TableSvg } from '$lib/asset/image/svg/table-svg';
	import type { TableInfoInterface } from '$lib/model/interface/erd.interface';
	import type {
		DatabaseConnectionInterface,
		DatasourceInterface
	} from '$lib/model/interface/schema.interface';
	import { selectedDatabaseState } from '$lib/store/state/selectedDatabase.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type {
		DatabaseMetadata,
		TableInfo as PostgresTableInfo
	} from '$lib/model/interface/postgres.interface';

	let { databaseConnection, datasource } = $props<{
		databaseConnection: DatabaseConnectionInterface[];
		datasource: DatasourceInterface[];
	}>();

	const svgMap: Record<string, string> = {
		SQLite: SQLiteSvg('size-5'),
		MySQL: MySQLSvg('size-5'),
		PostgreSQL: PostgreSQLSvg('size-5'),
		MongoDB: MongoDBSvg('size-5'),
		Oracle: OracleSvg('size-5')
	};

	async function getSQLiteTablesAndColumns(url: string): Promise<TableInfoInterface[]> {
		try {
			const result: TableInfoInterface[] = await invoke('get_database_from_sqlite', {
				url: url
			});
			return result;
		} catch (error) {
			console.error('Error fetching SQLite info:', error);
			return []; // Return an empty array in case of error
		}
	}

	async function getPostgreSQLTablesAndColumns(url: string): Promise<TableInfoInterface[]> {
		try {
			const result = await invoke<DatabaseMetadata>('get_database_from_postgres', {
				url: url
			});
			console.log('PostgreSQL metadata:', result);
			// Transform the DatabaseMetadata into TableInfoInterface[]
			return result.tables.map((table: PostgresTableInfo) => ({
				name: table.name,
				columns: table.columns.map((column) => ({
					name: column.name,
					data_type: column.data_type
				}))
			}));
		} catch (error) {
			console.error('Error fetching PostgreSQL info:', error);
			return []; // Return an empty array in case of error
		}
	}

	function handleSelectedDatabase(database: DatabaseConnectionInterface) {
		selectedDatabaseState.selectedDatabase = database;
	}
</script>

<main class="menu menu-xs bg-base-200 rounded-box max-w-64 min-w-64 overflow-auto p-1 shadow-sm">
	{#if databaseConnection.length > 0}
		{#each databaseConnection as database}
			{#each datasource as source}
				{#if database.datasource_id === source.id}
					<button onclick={() => handleSelectedDatabase(database)}>
						<li>
							<details>
								<summary>
									{@html svgMap[source.name]}
									{database.connection_name}
								</summary>
								{#if source.name === 'SQLite'}
									{#await getSQLiteTablesAndColumns(database.url)}
										<p>Loading tables and columns...</p>
									{:then tableInfos}
										<ul>
											{#each tableInfos as table}
												<li>
													<details>
														<summary>
															{@html TableSvg('size-5 text-info')}
															{table.name}
														</summary>
														<ul>
															{#each table.columns as column}
																<li>
																	<div>
																		{@html ColumnSvg('size-5 text-info')}
																		{column.name} ({column.data_type})
																	</div>
																</li>
															{/each}
														</ul>
													</details>
												</li>
											{/each}
										</ul>
									{:catch error}
										<p class="text-error">Failed to load tables and columns: {error}</p>
									{/await}
								{:else if source.name === 'PostgreSQL'}
									{#await getPostgreSQLTablesAndColumns(database.url)}
										<p>Loading tables and columns...</p>
									{:then tableInfos}
										<ul>
											{#each tableInfos as table}
												<li>
													<details>
														<summary>
															{@html TableSvg('size-5 text-info')}
															{table.name}
														</summary>
														<ul>
															{#each table.columns as column}
																<li>
																	<div>
																		{@html ColumnSvg('size-5 text-info')}
																		{column.name} ({column.data_type})
																	</div>
																</li>
															{/each}
														</ul>
													</details>
												</li>
											{/each}
										</ul>
									{:catch error}
										<p class="text-error">Failed to load tables and columns: {error}</p>
									{/await}
								{/if}
							</details>
						</li>
					</button>
				{/if}
			{/each}
		{/each}
	{:else}
		<li class="text-error text-center">No Database Connection Found</li>
	{/if}
</main>
