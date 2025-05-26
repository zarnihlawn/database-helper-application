<script lang="ts">
	import { DeleteSvg } from '$lib/asset/image/svg/delete-svg';
	import { PlaySvg } from '$lib/asset/image/svg/play-svg';
	import { PlusSvg } from '$lib/asset/image/svg/plus-svg';
	import type {
		ContentTypeInterface,
		DatabaseConnectionInterface
	} from '$lib/model/interface/schema.interface';
	import { invoke } from '@tauri-apps/api/core';
	import QueryBlockCodingWorkspace from './QueryBlockCodingWorkspace.svelte';
	import { selectedFileState } from '$lib/store/state/selectedFile.svelte';
	import type { QueryBlockInterface } from '$lib/model/interface/schema.interface';
	import { selectedDatabaseState } from '$lib/store/state/selectedDatabase.svelte';

	let { contentType } = $props<{ contentType: ContentTypeInterface[] }>();
	let targetFile = $derived(selectedFileState.selectedFile);

	let queryBlocks = $state<QueryBlockInterface[]>([]);
	let queryBlocksRunResult = $state<Record<number, any>>({});
	let nextId = $state(1);

	$effect(() => {
		if (targetFile) {
			invoke<QueryBlockInterface[]>('get_query_blocks', {
				queryFileId: targetFile.id
			}).then((blocks) => {
				queryBlocks = blocks;
				nextId = Math.max(...blocks.map((b) => b.id), 0) + 1;
			});
		}
	});

	async function addNewBlock() {
		if (!targetFile?.id) return;

		const newId = nextId++;
		await invoke('create_new_query_block', {
			queryFileId: targetFile.id,
			contentTypeId: 1,
			serialOrder: queryBlocks.length,
			queryContentBlock: ''
		});
		queryBlocks = [
			...queryBlocks,
			{
				id: newId,
				query_file_id: targetFile.id,
				content_type_id: 1,
				serial_order: queryBlocks.length,
				query_content_block: ''
			}
		];
	}

	async function removeBlock(id: number) {
		queryBlocks = queryBlocks.filter((block) => block.id !== id);
		await invoke('delete_query_block', {
			queryBlockId: id
		});
	}

	async function runBlock(id: number) {
		let content = await invoke('get_content_from_query_block', {
			queryBlockId: id
		});
		let database = selectedDatabaseState.selectedDatabase;
		try {
			let result = await invoke('run_query_block', {
				queryBlockId: id,
				databaseSource: database?.datasource_id,
				databaseConnection: database?.url,
				content: content
			});
			console.log('Query result:', result);
			queryBlocksRunResult[id] = result;
		} catch (error) {
			console.error('Query error:', error);
			queryBlocksRunResult[id] = { error: (error as Error).toString() };
		}
	}

	async function handleLanguageChange(event: Event, blockId: number) {
		const select = event.target as HTMLSelectElement;
		const selectedType = contentType.find(
			(type: ContentTypeInterface) => type.name === select.value
		);
		queryBlocks = queryBlocks.map((block) =>
			block.id === blockId
				? { ...block, content_type_id: selectedType?.id || 1 }
				: block
		);

		await invoke('update_query_block_content_type_id', {
			queryBlockId: blockId,
			contentTypeId: selectedType?.id || 1
		});
	}
</script>

{#if queryBlocks.length === 0}
	<main
		class="bg-base-200 rounded-box my-3 flex flex-col justify-between gap-3 px-5 py-3 shadow-sm"
	>
		<section class="flex">
			<section class="w-full">
				<div
					class="tooltip tooltip-right tooltip-primary"
					data-tip="create a new coding block"
				>
					<button class="btn btn-primary" onclick={addNewBlock}>
						{@html PlusSvg('size-4')}
						New Query Block
					</button>
				</div>
			</section>
		</section>
	</main>
{/if}

{#each queryBlocks as block (block.id)}
	<main
		class="bg-base-200 rounded-box my-3 flex flex-col justify-between gap-3 p-3 shadow-sm"
	>
		<section class="mx-2 flex">
			<section class="w-full">
				<div
					class="tooltip tooltip-right tooltip-success"
					data-tip="execute this query block"
				>
					<button class="btn btn-success" onclick={() => runBlock(block.id)}>
						{@html PlaySvg('size-4')} Run Block
					</button>
				</div>
				<div
					class="tooltip tooltip-right tooltip-error"
					data-tip="delete this coding block"
				>
					<button class="btn btn-error" onclick={() => removeBlock(block.id)}>
						{@html DeleteSvg('size-4')}
						Delete Block
					</button>
				</div>
			</section>
			<section class="w-full">
				<select
					class="select mr-4 w-full"
					onchange={(e) => handleLanguageChange(e, block.id)}
				>
					<option disabled selected>Pick a language ...</option>
					{#each contentType as type}
						{#if type.id === block.content_type_id}
							<option value={type.name} selected
								>{type.name.toUpperCase()}</option
							>
						{:else}
							<option value={type.name}>{type.name.toUpperCase()}</option>
						{/if}
					{/each}
				</select>
			</section>
		</section>
		<section class="px-2">
			<QueryBlockCodingWorkspace
				{block}
				language={contentType
					.find(
						(type: ContentTypeInterface) => type.id === block.content_type_id
					)
					?.name.toLowerCase() || 'markdown'}
				theme="vs-dark"
			/>
		</section>
		{#if queryBlocksRunResult[block.id]}
			<section class="mt-2 max-h-36 overflow-auto px-2">
				<div class="bg-base-300 rounded-box p-2">
					{#if queryBlocksRunResult[block.id].error}
						<h3 class="mb-2 text-error">Query Error:</h3>
						<pre class="whitespace-pre-wrap text-error">{queryBlocksRunResult[block.id].error}</pre>
					{:else}
						<h3 class="mb-2">Query Results:</h3>
						<pre class="whitespace-pre-wrap">{JSON.stringify(
								queryBlocksRunResult[block.id],
								null,
								2
							)}</pre>
					{/if}
				</div>
			</section>
		{/if}
	</main>
{/each}

{#if queryBlocks.length > 0}
	<main
		class="bg-base-200 rounded-box my-3 flex flex-col justify-between gap-3 px-5 py-3 shadow-sm"
	>
		<section class="flex">
			<section class="w-full">
				<div
					class="tooltip tooltip-right tooltip-primary"
					data-tip="create a new coding block"
				>
					<button class="btn btn-primary" onclick={addNewBlock}>
						{@html PlusSvg('size-4')}
						New Query Block
					</button>
				</div>
			</section>
		</section>
	</main>
{/if}
