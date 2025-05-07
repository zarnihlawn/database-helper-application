<script lang="ts">
	import { DeleteSvg } from '$lib/asset/image/svg/delete-svg';
	import { MariaDBSvg } from '$lib/asset/image/svg/mariadb-svg';
	import { MicrosoftSqlServerSvg } from '$lib/asset/image/svg/microsoft-sql-server-svg';
	import { MongoDBSvg } from '$lib/asset/image/svg/mongodb-svg';
	import { MySQLSvg } from '$lib/asset/image/svg/mysql-svg';
	import { PostgreSQLSvg } from '$lib/asset/image/svg/postgresql-svg';
	import { RemoveSvg } from '$lib/asset/image/svg/remove-svg';
	import { SQLiteSvg } from '$lib/asset/image/svg/sqlite-svg';
	import ConfirmDialog from '$lib/component/library/daisy/ConfirmDialog.svelte';
	import type {
		DatabaseConnectionInterface,
		DatasourceInterface
	} from '$lib/model/interface/schema.interface';
	import { invoke } from '@tauri-apps/api/core';

	let { datasource, databaseConnection, onClose } = $props<{
		datasource: DatasourceInterface[];
		databaseConnection: DatabaseConnectionInterface[];
		onClose: () => void;
	}>();

	let showConfirmRemovingDialog = $state(false);
	let confirmDialogKeyword = $state('');
	let selectedDatabaseConnectionId = $state(0);

	const svgMap: Record<string, string> = {
		SQLite: SQLiteSvg('size-11'),
		MySQL: MySQLSvg('size-11'),
		PostgreSQL: PostgreSQLSvg('size-11'),
		MongoDB: MongoDBSvg('size-11'),
		MariaDB: MariaDBSvg('size-11'),
		MSSQL: MicrosoftSqlServerSvg('size-11')
	};

	function handleShowConfirmRemovingDialog(id: number, url: string) {
		showConfirmRemovingDialog = true;
		confirmDialogKeyword = url;
		selectedDatabaseConnectionId = id;
	}

	function handleClose() {
		showConfirmRemovingDialog = false;
		onClose();
	}

	async function handleRemoveDatabaseConnection() {
		showConfirmRemovingDialog = false;
		console.log(
			'Selected Database Connection Id: ',
			selectedDatabaseConnectionId
		);
		const result = await invoke('remove_database_connection', {
			id: selectedDatabaseConnectionId
		});
		console.log('Result', result);
		onClose();
	}
</script>

<div class="modal modal-open">
	<div
		class="modal-box flex h-[70vh] w-11/12 max-w-5xl flex-col justify-between gap-3"
	>
		<h2 class="text-2xl font-bold">Remove a Database Connection</h2>

		<div class="body overflow-y-auto">
			<ul class="list bg-base-100 rounded-box shadow-md">
				{#each databaseConnection as databaseConnection, index}
					<li class="list-row">
						<div class="text-4xl font-thin tabular-nums opacity-30">
							{(index + 1).toString().padStart(2, '0')}
						</div>
						<div>
							{#each datasource as datasource}
								{#if databaseConnection.datasource_id === datasource.id}
									{@html svgMap[datasource.name]}
								{/if}
							{/each}
						</div>
						<div class="list-col-grow">
							<div>{databaseConnection.connection_name}</div>
							<div class="text-xs font-semibold opacity-60">
								{databaseConnection.url}
							</div>
						</div>
						<div
							class="tooltip tooltip-left tooltip-error"
							data-tip="delete this database connection"
						>
							<button
								class="btn btn-square btn-ghost text-error"
								onclick={() =>
									handleShowConfirmRemovingDialog(
										databaseConnection.id,
										databaseConnection.url
									)}
							>
								{@html RemoveSvg}
							</button>
						</div>
					</li>
				{/each}
			</ul>
		</div>

		<div class="modal-action justify-end">
			<button class="btn btn-error" onclick={handleClose}>Cancel</button>
		</div>
	</div>
</div>

{#if showConfirmRemovingDialog}
	<ConfirmDialog
		action="Remove"
		keyword={confirmDialogKeyword}
		onClose={handleClose}
		doAction={handleRemoveDatabaseConnection}
	/>
{/if}
