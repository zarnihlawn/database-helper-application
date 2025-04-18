<script lang="ts">
	import { MongoDBSvg } from '$lib/asset/image/svg/mongodb-svg';
	import { MySQLSvg } from '$lib/asset/image/svg/mysql-svg';
	import { OracleSvg } from '$lib/asset/image/svg/oracle-svg';
	import { PostgreSQLSvg } from '$lib/asset/image/svg/postgresql-svg';
	import { SQLiteSvg } from '$lib/asset/image/svg/sqlite-svg';
	import type { DatasourceInterface } from '$lib/model/interface/schema.interface';
	import { userState } from '$lib/store/state/user.state.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';

	let { datasource, onClose } = $props<{
		datasource: DatasourceInterface;
		onClose: () => void;
	}>();

	const svgMap: Record<string, string> = {
		SQLite: SQLiteSvg('size-10'),
		MySQL: MySQLSvg('size-10'),
		PostgreSQL: PostgreSQLSvg('size-10'),
		MongoDB: MongoDBSvg('size-10'),
		Oracle: OracleSvg('size-10')
	};

	let connectionName = $state('');
	let url = $state('');
	let user = userState.user;

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
			case 'Oracle':
				return 'oracle://username:password@localhost:1521';
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
			switch (datasourceType) {
				case 'SQLite':
					await invoke('test_sqlite_connection', { url: url });
					break;
				case 'MySQL':
					break;
				case 'PostgreSQL':
					try {
						await invoke('test_postgres_connection', { url: url });
					} catch (error) {
						console.error('Connection error:', error);
					}
					break;
				case 'MongoDB':
					try {
						const result = await invoke('test_mongo_connection', { url: url });
						console.log('Connection result:', result);
					} catch (error) {
						console.error('Connection error:', error);
					}
					break;
				case 'Oracle':
					try {
						const result = await invoke('test_oracle_connection', { url: url });
						console.log('Connection result:', result);
					} catch (error) {
						console.error('Connection error:', error);
					}
					break;
				default:
					break;
			}
		} else if (datasourceType === 'SQLite') {
			alert('Please select an SQLite database file.');
		}
	}

	async function handleConnect(datasourceType: string) {
		if (url && connectionName.trim() !== '') {
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
						console.log(result);
					} else {
						const result = await invoke('save_mongo_connection', {
							user_id: null,
							url: url,
							connectionName: connectionName
						});
						console.log(result);
					}
					break;
				case 'Oracle':
					if (user?.id) {
						const result = await invoke('save_oracle_connection', {
							user_id: user.id,
							url: url,
							connectionName: connectionName
						});
					} else {
						const result = await invoke('save_oracle_connection', {
							user_id: null,
							url: url,
							connectionName: connectionName
						});
					}
					break;
				default:
					break;
			}
		} else if (datasourceType === 'SQLite') {
			alert('Please select an SQLite database file and enter a connection name.');
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
				<fieldset class="fieldset bg-base-200 border-base-300 rounded-box w-full border p-4">
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
						<button class="btn btn-primary w-full" onclick={handleSelectSQLiteFile}
							>Select SQLite Database</button
						>
					{:else}
						<input
							name="text"
							class="input w-full"
							bind:value={url}
							placeholder={getUrlConnectionPlaceholder(datasource.name)}
						/>
						{#if url !== ''}
							<p class="fieldset-label ml-2">E.g. {getUrlConnectionPlaceholder(datasource.name)}</p>
						{/if}
					{/if}

					<div class="modal-action">
						<button class="btn btn-success" onclick={() => handleTestConnection(datasource.name)}>
							Test Connection
						</button>
						<button class="btn btn-error" onclick={handleClose}>Cancel</button>
						<button class="btn btn-primary" onclick={() => handleConnect(datasource.name)}
							>Connect</button
						>
					</div>
				</fieldset>
			</form>
		</div>
	</div>
</div>
