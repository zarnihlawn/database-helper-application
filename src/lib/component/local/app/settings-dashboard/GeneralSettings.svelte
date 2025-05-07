<script lang="ts">
	import type {
		MemoryInformation,
		OsInformation
	} from '$lib/model/interface/os-information.interface';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';

	let osInformation: OsInformation = $state({});
	let memoryInformation: MemoryInformation = $state({});

	onMount(() => {
		getOsInformation();
		setInterval(() => {
			getMemoryInformation();
		}, 10000);
	});

	async function getOsInformation() {
		osInformation = await invoke<OsInformation>('get_os_information');
	}

	async function getMemoryInformation() {
		memoryInformation = await invoke<MemoryInformation>(
			'get_memory_information'
		);
	}

	function convertBytesToMB(byteValue: number | undefined): string {
		if (byteValue === undefined || byteValue === null || isNaN(byteValue)) {
			// Show loading or placeholder if value is not a valid number yet
			return ''; // Or you could return 'Loading...' or specific placeholder
		}
		// Convert Bytes to MB: Bytes / (1024 KB/Byte * 1024 MB/KB)
		return (byteValue / (1024 * 1024)).toFixed(2) + ' MB';
	}
</script>

<main>
	<div class="overflow-x-auto p-[-64px]">
		<table class="table">
			<thead>
				<tr>
					<th></th>
					<th>Title</th>
					<th>Content</th>
				</tr>
			</thead>
			<tbody>
				<tr>
					<td>1</td>
					<td>Arch</td>
					<td>{osInformation.arch}</td>
				</tr>
				<tr>
					<td>2</td>
					<td>EXE</td>
					<td>{osInformation.exe_extension}</td>
				</tr>

				<tr>
					<td>3</td>
					<td>Hostname</td>
					<td>{osInformation.hostname}</td>
				</tr>
				<tr>
					<td>4</td>
					<td>Locale</td>
					<td>{osInformation.locale}</td>
				</tr>
				<tr>
					<td>5</td>
					<td>Platform</td>
					<td>{osInformation.platform}</td>
				</tr>
				<tr>
					<td>6</td>
					<td>Version</td>
					<td>{osInformation.version}</td>
				</tr>
				<tr>
					<td>7</td>
					<td>Total Memory</td>
					<td>
						{#if memoryInformation.total_memory !== undefined}
							{convertBytesToMB(memoryInformation.total_memory)}
						{:else}
							<span class="loading loading-spinner text-primary"></span>
						{/if}
					</td>
				</tr>
				<tr>
					<td>8</td>
					<td>Free Memory</td>
					<td>
						{#if memoryInformation.available_memory !== undefined}
							{convertBytesToMB(memoryInformation.available_memory)}
						{:else}
							<span class="loading loading-spinner text-primary"></span>
						{/if}
					</td>
				</tr>
			</tbody>
		</table>
	</div>
</main>
