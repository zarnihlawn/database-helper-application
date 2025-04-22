<script lang="ts">
	import { ColumnSvg } from '$lib/asset/image/svg/column-svg';
	import { MongoDBSvg } from '$lib/asset/image/svg/mongodb-svg';
	import { MySQLSvg } from '$lib/asset/image/svg/mysql-svg';
	import { PostgreSQLSvg } from '$lib/asset/image/svg/postgresql-svg';
	import { SQLiteSvg } from '$lib/asset/image/svg/sqlite-svg';
	import { TableSvg } from '$lib/asset/image/svg/table-svg';
	import { FolderSvg } from '$lib/asset/image/svg/folder-svg';
	import type { CollectionInfo, TableInfoInterface } from '$lib/model/interface/erd.interface';
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
	import { FileSvg } from '$lib/asset/image/svg/file-svg';
	import { MariaDBSvg } from '$lib/asset/image/svg/mariadb-svg';
	import { MicrosoftSqlServerSvg } from '$lib/asset/image/svg/microsoft-sql-server-svg';
	import { SchemaSvg } from '$lib/asset/image/svg/schema-svg';
	import type { DatabaseInfo } from '$lib/model/interface/mssql.interface';
	import { DatabaseSvg } from '$lib/asset/image/svg/database-svg';

	let { databaseConnection, datasource } = $props<{
		databaseConnection: DatabaseConnectionInterface[];
		datasource: DatasourceInterface[];
	}>();

	const svgMap: Record<string, string> = {
		SQLite: SQLiteSvg('size-5'),
		MySQL: MySQLSvg('size-5'),
		PostgreSQL: PostgreSQLSvg('size-5'),
		MongoDB: MongoDBSvg('size-5'),
		MariaDB: MariaDBSvg('size-5'),
		MSSQL: MicrosoftSqlServerSvg('size-5')
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

	async function getMssqlTablesAndColumns(
		url: string
	): Promise<
		{ database_name: string; schemas: { schema_name: string; tables: TableInfoInterface[] }[] }[]
	> {
		try {
			const result = await invoke<DatabaseInfo[]>('get_database_from_mssql', {
				url: url
			});

			console.log('MSSQL metadata:', result);

			// Transform the data to group tables by database and then by schema
			const databaseInfos: {
				database_name: string;
				schemas: { schema_name: string; tables: TableInfoInterface[] }[];
			}[] = [];

			for (const database of result) {
				const schemas: { schema_name: string; tables: TableInfoInterface[] }[] = [];
				for (const schema of database.schemas) {
					const tables: TableInfoInterface[] = schema.tables.map(
						(table: { name: string; columns: { name: string; data_type: string }[] }) => ({
							name: table.name,
							columns: table.columns.map((column: { name: string; data_type: string }) => ({
								name: column.name,
								data_type: column.data_type
							}))
						})
					);
					schemas.push({ schema_name: schema.name, tables: tables });
				}
				databaseInfos.push({ database_name: database.name, schemas: schemas });
			}

			return databaseInfos;
		} catch (error) {
			console.error('Error fetching MSSQL info:', error);
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

	async function getMongoDBCollections(
		url: string
	): Promise<{ database_name: string; collections: string[] }[]> {
		try {
			const result = await invoke<CollectionInfo[]>('get_database_from_mongo', {
				url: url
			});

			// Group collections by database name
			const groupedCollections = result.reduce(
				(acc, curr) => {
					const existingDb = acc.find((db) => db.database_name === curr.database_name);
					if (existingDb) {
						existingDb.collections.push(curr.collection_name);
					} else {
						acc.push({
							database_name: curr.database_name,
							collections: [curr.collection_name]
						});
					}
					return acc;
				},
				[] as { database_name: string; collections: string[] }[]
			);

			return groupedCollections;
		} catch (error) {
			console.error('Error fetching MongoDB info:', error);
			return [];
		}
	}

	async function getMySQLTablesAndColumns(
		url: string
	): Promise<{ schema_name: string; tables: TableInfoInterface[] }[]> {
		try {
			const result = await invoke<
				Record<
					string,
					Record<
						string,
						{ tables: { name: string; columns: { name: string; data_type: string }[] }[] }
					>
				>
			>('get_database_from_mysql', {
				url: url
			});

			console.log('MySQL metadata:', result);

			// Transform the data to group tables by schema
			const schemaInfos: { schema_name: string; tables: TableInfoInterface[] }[] = [];
			for (const [schema, schemaData] of Object.entries(result)) {
				const tables: TableInfoInterface[] = [];
				for (const [_, dbObjects] of Object.entries(schemaData)) {
					for (const table of dbObjects.tables) {
						tables.push({
							name: table.name,
							columns: table.columns.map((column) => ({
								name: column.name,
								data_type: column.data_type
							}))
						});
					}
				}
				schemaInfos.push({
					schema_name: schema,
					tables: tables
				});
			}

			return schemaInfos;
		} catch (error) {
			console.error('Error fetching MySQL info:', error);
			return [];
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
																		{column.name}
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
								{:else if source.name === 'MongoDB'}
									{#await getMongoDBCollections(database.url)}
										<p>Loading collections...</p>
									{:then groupedCollections}
										<ul>
											{#each groupedCollections as group}
												<li>
													<details>
														<summary>
															{@html FolderSvg('size-5 text-success')}
															{group.database_name}
														</summary>
														<ul>
															{#each group.collections as collection}
																<li>
																	<div>
																		{@html FileSvg('size-5 text-success')}
																		{collection}
																	</div>
																</li>
															{/each}
														</ul>
													</details>
												</li>
											{/each}
										</ul>
									{:catch error}
										<p class="text-error">Failed to load collections: {error}</p>
									{/await}
								{:else if source.name === 'MySQL'}
									{#await getMySQLTablesAndColumns(database.url)}
										<p>Loading tables and columns...</p>
									{:then schemaInfos}
										<ul>
											{#each schemaInfos as schema}
												<li>
													<details>
														<summary>
															{@html SchemaSvg('size-5 text-warning')}
															{schema.schema_name}
														</summary>
														<ul>
															{#each schema.tables as table}
																<li>
																	<details>
																		<summary>
																			{@html TableSvg('size-5 text-warning')}
																			{table.name}
																		</summary>
																		<ul>
																			{#each table.columns as column}
																				<li>
																					<div>
																						{@html ColumnSvg('size-5 text-warning')}
																						{column.name} ({column.data_type})
																					</div>
																				</li>
																			{/each}
																		</ul>
																	</details>
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
								{:else if source.name === 'MariaDB'}
									{#await getMySQLTablesAndColumns(database.url)}
										<p>Loading tables and columns...</p>
									{:then schemaInfos}
										<ul>
											{#each schemaInfos as schema}
												<li>
													<details>
														<summary>
															{@html SchemaSvg('size-5 text-warning')}
															{schema.schema_name}
														</summary>
														<ul>
															{#each schema.tables as table}
																<li>
																	<details>
																		<summary>
																			{@html TableSvg('size-5 text-warning')}
																			{table.name}
																		</summary>
																		<ul>
																			{#each table.columns as column}
																				<li>
																					<div>
																						{@html ColumnSvg('size-5 text-warning')}
																						{column.name} ({column.data_type})
																					</div>
																				</li>
																			{/each}
																		</ul>
																	</details>
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
								{:else if source.name === 'MSSQL'}
									{#await getMssqlTablesAndColumns(database.url)}
										<p>Loading tables and columns...</p>
									{:then databaseInfos}
										{#each databaseInfos as dbInfo (dbInfo.database_name)}
											<ul>
												<li>
													<details>
														<summary>
															{@html DatabaseSvg('size-5 text-warning')}
															{dbInfo.database_name}
														</summary>
														<ul>
															{#each dbInfo.schemas as schema (schema.schema_name)}
																<li>
																	<details>
																		<summary>
																			{@html SchemaSvg('size-5 text-warning')}
																			{schema.schema_name}
																		</summary>
																		<ul>
																			{#each schema.tables as table (table.name)}
																				<li>
																					<details>
																						<summary>
																							{@html TableSvg('size-5 text-warning')}
																							{table.name}
																						</summary>
																						<ul>
																							{#each table.columns as column (column.name)}
																								<li>
																									<div>
																										{@html ColumnSvg('size-5 text-warning')}
																										{column.name} ({column.data_type})
																									</div>
																								</li>
																							{/each}
																						</ul>
																					</details>
																				</li>
																			{/each}
																		</ul>
																	</details>
																</li>
															{/each}
														</ul>
													</details>
												</li>
											</ul>
										{/each}
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
