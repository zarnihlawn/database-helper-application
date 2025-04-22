<script lang="ts">
	import type { ContentTypeInterface } from '$lib/model/interface/schema.interface';
	import QueryBlockCodingWorkspace from './QueryBlockCodingWorkspace.svelte';

	let { contentType } = $props<{ contentType: ContentTypeInterface[] }>();

	let codingBlockCreated = $state(false);
</script>

<main class="bg-base-200 rounded-box my-3 flex justify-between p-3 shadow-sm">
	<section>
		{#if !codingBlockCreated}
			<div
				class="tooltip tooltip-right tooltip-primary"
				data-tip="create a new coding block"
			>
				<button
					class="btn btn-primary"
					onclick={() => (codingBlockCreated = true)}
				>
					New
				</button>
			</div>
		{:else}
			<div
				class="tooltip tooltip-right tooltip-success"
				data-tip="execute this query block"
			>
				<button class="btn btn-success"> Run </button>
			</div>
		{/if}
	</section>
	<section>
		<select class="select mx-2">
			<option disabled selected>Pick a language ...</option>
			{#each contentType as type}
				<option>{type.name}</option>
			{/each}
		</select>
	</section>

	<section>
		{#if codingBlockCreated}
			<div
				class="tooltip tooltip-left tooltip-warning"
				data-tip="edit this coding block"
			>
				<button
					class="btn btn-warning"
					onclick={() => (codingBlockCreated = false)}
				>
					Del
				</button>
			</div>
		{/if}
	</section>

	{#if codingBlockCreated}
		<section>
			<QueryBlockCodingWorkspace />
		</section>
	{/if}
</main>
