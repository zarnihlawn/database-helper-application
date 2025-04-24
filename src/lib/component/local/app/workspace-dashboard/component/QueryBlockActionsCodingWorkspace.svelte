<script lang="ts">
	import { DeleteSvg } from '$lib/asset/image/svg/delete-svg';
	import { PlaySvg } from '$lib/asset/image/svg/play-svg';
	import { PlusSvg } from '$lib/asset/image/svg/plus-svg';
	import type { ContentTypeInterface } from '$lib/model/interface/schema.interface';
	import { invoke } from '@tauri-apps/api/core';
	import QueryBlockCodingWorkspace from './QueryBlockCodingWorkspace.svelte';
	import { selectedFileState } from '$lib/store/state/selectedFile.svelte';
	import type { QueryBlockInterface } from '$lib/model/interface/schema.interface';

	let { contentType } = $props<{ contentType: ContentTypeInterface[] }>();
	let targetFile = $derived(selectedFileState.selectedFile);

	let queryBlocks = $state<QueryBlockInterface[]>([]);

	let nextId = $state(1);

	$effect(() => {
		if (targetFile) {
			invoke<QueryBlockInterface[]>('get_query_blocks', {
				queryFileId: targetFile.id
			}).then((blocks) => {
				queryBlocks = blocks;
			});
		}
	});

	async function addNewBlock() {
		if (!targetFile?.id) return;

		await invoke('create_new_query_block', {
			queryFileId: targetFile.id,
			contentTypeId: 1,
			serialOrder: queryBlocks.length,
			queryContentBlock: ''
		});
		queryBlocks = [
			...queryBlocks,
			{
				id: nextId,
				query_file_id: targetFile.id,
				content_type_id: 1,
				serial_order: queryBlocks.length,
				query_content_block: ''
			}
		];
		nextId++;
	}

	function removeBlock(id: number) {
		queryBlocks = queryBlocks.filter((block) => block.id !== id);
	}

	function handleLanguageChange(event: Event, blockId: number) {
		const select = event.target as HTMLSelectElement;
		const selectedType = contentType.find(
			(type: ContentTypeInterface) => type.name === select.value
		);
		queryBlocks = queryBlocks.map((block) =>
			block.id === blockId
				? { ...block, content_type_id: selectedType?.id || 1 }
				: block
		);
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
					<button class="btn btn-success">
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
					class="select mx-2 w-full"
					onchange={(e) => handleLanguageChange(e, block.id)}
				>
					<option disabled selected>Pick a language ...</option>
					{#each contentType as type}
						<option value={type.name}>{type.name}</option>
					{/each}
				</select>
			</section>
		</section>
		<section class="px-2">
			<QueryBlockCodingWorkspace
				block={block}
				language={contentType
					.find(
						(type: ContentTypeInterface) => type.id === block.content_type_id
					)
					?.name.toLowerCase() || 'markdown'}
				theme="vs-dark"
			/>
		</section>
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
