<script lang="ts">
	import { MariaDBSvg } from '$lib/asset/image/svg/mariadb-svg';
	import { MicrosoftSqlServerSvg } from '$lib/asset/image/svg/microsoft-sql-server-svg';
	import { MongoDBSvg } from '$lib/asset/image/svg/mongodb-svg';
	import { MySQLSvg } from '$lib/asset/image/svg/mysql-svg';
	import { PostgreSQLSvg } from '$lib/asset/image/svg/postgresql-svg';
	import { SQLiteSvg } from '$lib/asset/image/svg/sqlite-svg';
	import type { DatasourceInterface } from '$lib/model/interface/schema.interface';
	import { userState } from '$lib/store/state/user.state.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';
	import Alert from '$lib/component/library/daisy/Alert.svelte';

	let { datasource, onClose } = $props<{
		datasource: DatasourceInterface;
		onClose: () => void;
	}>();

	const svgMap: Record<string, string> = {
		SQLite: SQLiteSvg('size-20'),
		MySQL: MySQLSvg('size-20'),
		PostgreSQL: PostgreSQLSvg('size-20'),
		MongoDB: MongoDBSvg('size-20'),
		MariaDB: MariaDBSvg('size-20'),
		MSSQL: MicrosoftSqlServerSvg('size-20')
	};

	let connectionName = $state('');
	let url = $state('');
	let user = userState.user;

	// Add alert state variables
	let showAlert = $state(false);
	let alertType = $state<'success' | 'error'>('success');
	let alertMessage = $state('');

	// Function to show alert and auto-hide after 5 seconds
	function showAlertMessage(type: 'success' | 'error', message: string) {
		showAlert = true;
		alertType = type;
		alertMessage = message;

		setTimeout(() => {
			showAlert = false;
		}, 3000);
	}

	function getUrlConnectionPlaceholder(name: string): string {
		switch (name) {
			case 'SQLite':
				return 'Select SQLite database file...';
			case 'MySQL':
				return 'mysql://username:password@localhost:3306';
			case 'PostgreSQL':
				return 'postgres://username:password@localhost:5432';
			case 'MongoDB':
				return 'mongodb://username:password@localhost:27017';
			case 'MariaDB':
				return 'mysql://username:password@localhost:3306';
			case 'MSSQL':
				return 'mssql://username:password@localhost:1433';

			default:
				return '';
		}
	}

	function handleClose() {
		onClose();
	}

	async function handleSelectSQLiteFile() {
		if (datasource.name === 'SQLite') {
			const selected = await open({
				filters: [
					{
						name: 'SQLite Database',
						extensions: ['db', 'sqlite', 'sqlite3']
					}
				],
				multiple: false
			});
			if (typeof selected === 'string') {
				url = selected;
			}
		}
	}

	async function handleTestConnection(datasourceType: string) {
		if (url) {
			try {
				switch (datasourceType) {
					case 'SQLite':
						await invoke('test_sqlite_connection', { url: url });
						break;
					case 'MySQL':
						await invoke('test_mysql_connection', { url: url });
						break;
					case 'PostgreSQL':
						await invoke('test_postgres_connection', { url: url });
						break;
					case 'MongoDB':
						await invoke('test_mongo_connection', { url: url });
						break;
					case 'MariaDB':
						await invoke('test_maria_connection', { url: url });
						break;
					case 'MSSQL':
						await invoke('test_mssql_connection', { url: url });
						break;

					default:
						break;
				}
				showAlertMessage(
					'success',
					`Successfully connected to ${datasourceType} database`
				);
			} catch (error) {
				showAlertMessage(
					'error', // Changed from 'success' to 'error'
					`Failed to connect to ${datasourceType} database: ${error}` // Added error message
				);
			}
		} else if (datasourceType === 'SQLite') {
			showAlertMessage('error', 'Please select an SQLite database file.');
		} else {
      showAlertMessage('error', 'Please enter a valid database URL.'); // Added for other database types if URL is missing
    }
	}

	async function handleConnect(datasourceType: string) {
		if (url && connectionName.trim() !== '') {
			try {
				switch (datasourceType) {
					case 'SQLite':
						if (user?.id) {
							await invoke('save_sqlite_connection', {
								user_id: user.id,
								url: url,
								connectionName: connectionName
							});
						} else {
							await invoke('save_sqlite_connection', {
								user_id: null,
								url: url,
								connectionName: connectionName
							});
						}
						break;
					case 'MySQL':
						if (user?.id) {
							await invoke('save_mysql_connection', {
								user_id: user.id,
								url: url,
								connectionName: connectionName
							});
						} else {
							await invoke('save_mysql_connection', {
								user_id: null,
								url: url,
								connectionName: connectionName
							});
						}
						break;
					case 'PostgreSQL':
						if (user?.id) {
							await invoke('save_postgres_connection', {
								user_id: user.id,
								url: url,
								connectionName: connectionName
							});
						} else {
							await invoke('save_postgres_connection', {
								user_id: null,
								url: url,
								connectionName: connectionName
							});
						}
						break;
					case 'MongoDB':
						if (user?.id) {
							const result = await invoke('save_mongo_connection', {
								user_id: user.id,
								url: url,
								connectionName: connectionName
							});
						} else {
							const result = await invoke('save_mongo_connection', {
								user_id: null,
								url: url,
								connectionName: connectionName
							});
						}
						break;
					case 'MariaDB':
						if (user?.id) {
							await invoke('save_maria_connection', {
								user_id: user.id,
								url: url,
								connectionName: connectionName
							});
						} else {
							await invoke('save_maria_connection', {
								user_id: null,
								url: url,
								connectionName: connectionName
							});
						}

						break;
					case 'MSSQL':
						if (user?.id) {
							await invoke('save_mssql_connection', {
								user_id: user.id,
								url: url,
								connectionName: connectionName
							});
						} else {
							await invoke('save_mssql_connection', {
								user_id: null,
								url: url,
								connectionName: connectionName
							});
						}
						break;

					default:
						break;
				}

				showAlertMessage(
					'success',
					`Successfully connected to ${datasourceType} database`
				);
				// Close the dialog after successful connection
				setTimeout(() => {
					onClose();
				}, 1000);
			} catch (error) {
				showAlertMessage(
					'error',
					`Failed to connect to ${datasourceType} database: ${error}`
				);
			}
		} else if (datasourceType === 'SQLite') {
			showAlertMessage(
				'error',
				'Please select an SQLite database file and enter a connection name.'
			);
		} else {
			showAlertMessage(
				'error',
				'Please enter a connection name and valid database URL.'
			);
		}
	}
</script>

<div class="modal modal-open">
	<div class="modal-box flex h-[90vh] w-11/12 max-w-5xl flex-col">
		<h2 class=" flex items-center gap-2 text-2xl font-bold">
			Connect to
			{@html svgMap[datasource.name]}
			{datasource.name}
		</h2>

		<div class="my-1 flex-1 overflow-y-auto py-3">
			<form>
				<fieldset
					class="fieldset bg-base-200 border-base-300 rounded-box w-full border p-4"
				>
					<legend class="fieldset-legend">
						<h2 class="text-lg font-bold">Connection Details</h2>
					</legend>

					<label for="host" class="fieldset-label">Name</label>
					<input
						name="text"
						class="input w-full"
						bind:value={connectionName}
						placeholder="{datasource.name} connection 1"
					/>

					<label for="url" class="fieldset-label">URL</label>
					{#if datasource.name === 'SQLite'}
						<button
							class="btn btn-primary w-full"
							onclick={handleSelectSQLiteFile}>Select SQLite Database</button
						>
					{:else}
						<input
							name="text"
							class="input w-full"
							bind:value={url}
							placeholder={getUrlConnectionPlaceholder(datasource.name)}
						/>
						{#if url !== ''}
							<p class="fieldset-label ml-2">
								E.g. {getUrlConnectionPlaceholder(datasource.name)}
							</p>
						{/if}
					{/if}

					<div class="modal-action">
						<button
						class="btn btn-success"
						class:btn-disabled={showAlert}
						onclick={() => handleTestConnection(datasource.name)}

						>
						{#if showAlert}
						<span class="loading loading-spinner text-success"></span>
						{:else}
							Test Connection
							{/if}
						</button>
						<button class="btn btn-error" onclick={handleClose}>Cancel</button>
						<button
							class="btn btn-primary"
							class:btn-disabled={showAlert}
							onclick={() => handleConnect(datasource.name)}>
							{#if showAlert}
						<span class="loading loading-spinner text-primary"></span>
						{:else}
							Connect
							{/if}</button
						>
					</div>
				</fieldset>
			</form>
		</div>
		{#if showAlert}
			<Alert show={showAlert} type={alertType} message={alertMessage} />
			{/if}
	</div>
</div>

