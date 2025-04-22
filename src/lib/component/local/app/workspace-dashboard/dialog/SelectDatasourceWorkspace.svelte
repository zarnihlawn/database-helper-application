<script lang="ts">
	import { MariaDBSvg } from '$lib/asset/image/svg/mariadb-svg';
	import { MicrosoftSqlServerSvg } from '$lib/asset/image/svg/microsoft-sql-server-svg';
	import { MongoDBSvg } from '$lib/asset/image/svg/mongodb-svg';
	import { MySQLSvg } from '$lib/asset/image/svg/mysql-svg';
	import { PostgreSQLSvg } from '$lib/asset/image/svg/postgresql-svg';
	import { SQLiteSvg } from '$lib/asset/image/svg/sqlite-svg';
	import type { DatasourceInterface } from '$lib/model/interface/schema.interface';
	import FieldConnectingDatasourceWorkspace from './FieldConnectingDatasourceWorkspace.svelte';

	let { datasource, onClose } = $props<{
		datasource: DatasourceInterface[];
		onClose: () => void;
	}>();

	const svgMap: Record<string, string> = {
		SQLite: SQLiteSvg('size-20'),
		MySQL: MySQLSvg('size-20'),
		PostgreSQL: PostgreSQLSvg('size-20'),
		MongoDB: MongoDBSvg('size-20'),
		MariaDB: MariaDBSvg('size-20'),
		MSSQL: MicrosoftSqlServerSvg('size-20'),
	};

	let selectedDatasource: DatasourceInterface | null = $state(null);
	let showFieldConnectingDatasourceDialog = $state(false);

	function handleClose() {
		onClose();
	}

	function handleOpenFieldConnectingDatasourceDialog(datasource: DatasourceInterface) {
		selectedDatasource = datasource;
		showFieldConnectingDatasourceDialog = true;
	}
</script>

<div class="modal modal-open">
	<div class="modal-box flex h-[90vh] w-11/12 max-w-5xl flex-col">
		<h2 class="text-2xl font-bold">Select A Data Source</h2>

		<div class="my-1 flex-1 overflow-y-auto py-3">
			<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
				{#each datasource as datasource}
					<div class="card bg-base-100 flex p-5 shadow-sm">
						<div class="card-image flex items-center justify-center">
							{@html svgMap[datasource.name]}
						</div>
						<div class="card-body flex">
							<h2 class="card-title flex justify-center text-2xl">{datasource.name}</h2>
							<div class="text-xs font-semibold opacity-60">
								{datasource.description}
							</div>
							<button
								class="btn btn-primary flex justify-center"
								onclick={() => handleOpenFieldConnectingDatasourceDialog(datasource)}
							>
								Connect
							</button>
						</div>
					</div>
				{/each}
			</div>
		</div>

		<div class="modal-action">
			<button class="btn btn-error" onclick={handleClose}>Cancel</button>
		</div>
	</div>
</div>

{#if showFieldConnectingDatasourceDialog && selectedDatasource}
	<FieldConnectingDatasourceWorkspace
		datasource={selectedDatasource}
		onClose={() => {
			showFieldConnectingDatasourceDialog = false;
			selectedDatasource = null;
		}}
	/>
{/if}
