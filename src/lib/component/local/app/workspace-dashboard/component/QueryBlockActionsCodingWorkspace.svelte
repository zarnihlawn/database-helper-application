<script lang="ts">
	import type { ContentTypeInterface } from '$lib/model/interface/schema.interface';

	let { contentType } = $props<{ contentType: ContentTypeInterface[] }>();
</script>

<main class="bg-base-200 rounded-box my-3 flex justify-between p-3 shadow-sm">
	<section>
		<div class="tooltip tooltip-right tooltip-primary" data-tip="create a new coding block">
			<button class="btn btn-primary"> New </button>
		</div>
		<div class="tooltip tooltip-right tooltip-success" data-tip="execute this query block">
			<button class="btn btn-success"> Run </button>
		</div>
	</section>
	<section>
		<select class="select select-secondary mx-2">
			<option disabled selected>Pick a language ...</option>
			{#each contentType as type}
				<option>{type.name}</option>
			{/each}
		</select>
	</section>

	<section>
		<div class="tooltip tooltip-left tooltip-error" data-tip="delete this coding block">
			<button class="btn btn-error"> Del </button>
		</div>
	</section>
</main>
