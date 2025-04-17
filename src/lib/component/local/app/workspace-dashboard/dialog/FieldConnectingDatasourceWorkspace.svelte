<script lang="ts">
	import { MongoDBSvg } from '$lib/asset/image/svg/mongodb-svg';
	import { MySQLSvg } from '$lib/asset/image/svg/mysql-svg';
	import { OracleSvg } from '$lib/asset/image/svg/oracle-svg';
	import { PostgreSQLSvg } from '$lib/asset/image/svg/postgresql-svg';
	import { SQLiteSvg } from '$lib/asset/image/svg/sqlite-svg';
	import type {
		datasourceAuthenticationTypeInterface,
		datasourceInterface
	} from '$lib/model/interface/schema.interface';
	import { userState } from '$lib/store/state/user.state.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';

	let { datasource, datasourceAuthenticationType, onClose } = $props<{
		datasource: datasourceInterface;
		datasourceAuthenticationType: datasourceAuthenticationTypeInterface[];
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

	function getUrlConnectionPlaceholder(type: string): string {
		switch (type) {
			case 'SQLite':
				return 'Select SQLite database file...';
			case 'MySQL':
				return 'mysql://username:password@localhost:3306/database';
			case 'PostgreSQL':
				return 'postgres://username:password@localhost:5432/database';
			case 'MongoDB':
				return 'mongodb://localhost:27017/database';
			case 'Oracle':
				return 'oracle://username:password@localhost:1521/database';
			default:
				return '';
		}
	}

	function handleClose() {
		onClose();
	}

	async function handleSelectSQLiteFile() {
		if (datasource.type === 'SQLite') {
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
					break;
				case 'MongoDB':
					break;
				case 'Oracle':
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
					console.log(connectionName);
					if (user?.id) {
						console.log('Connecting with user: ', user.id);
						await invoke('save_sqlite_connection', {
							user_id: 2,
							connection_name: connectionName,
							url: url
						});
					} else {
						console.log('Connecting without user');
						await invoke('save_sqlite_connection', {
							user_id: null,
							connection_name: connectionName,
							url: url
						});
					}
					break;
				case 'MySQL':
					break;
				case 'PostgreSQL':
					break;
				case 'MongoDB':
					break;
				case 'Oracle':
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
			{@html svgMap[datasource.type]}
			{datasource.type}
		</h2>

		<div class="my-1 flex-1 overflow-y-auto py-3">
			<form>
				<fieldset class="fieldset bg-base-200 border-base-300 rounded-box w-full border p-4">
					<legend class="fieldset-legend">
						<h2 class="text-lg font-bold">Connection Details</h2>
					</legend>

					<label for="host" class="fieldset-label">Name</label>
					<input
						type="text"
						class="input w-full"
						bind:value={connectionName}
						placeholder="{datasource.type} connection 1"
					/>

					<label for="url" class="fieldset-label">URL</label>
					{#if datasource.type === 'SQLite'}
						<input
							type="file"
							class="file-input file-input-info w-full"
							onclick={handleSelectSQLiteFile}
						/>
					{:else}
						<input
							type="text"
							class="input w-full"
							bind:value={url}
							placeholder={getUrlConnectionPlaceholder(datasource.type)}
						/>
						{#if url !== ''}
							<p class="fieldset-label ml-2">E.g. {getUrlConnectionPlaceholder(datasource.type)}</p>
						{/if}
					{/if}

					<div class="modal-action">
						<button class="btn btn-success" onclick={() => handleTestConnection(datasource.type)}>
							Test Connection
						</button>
						<button class="btn btn-error" onclick={handleClose}>Cancel</button>
						<button class="btn btn-primary" onclick={() => handleConnect(datasource.type)}
							>Connect</button
						>
					</div>
				</fieldset>
			</form>
		</div>
	</div>
</div>
