<script lang="ts">
	import Monaco from '$lib/component/library/monaco/Monaco.svelte';
	import type { QueryBlockInterface } from '$lib/model/interface/schema.interface';
	import type { ContentTypeInterface } from '$lib/model/interface/schema.interface';
	let {
		block,
		language = 'markdown',
		theme = 'vs-dark'
	} = $props<{
		block: QueryBlockInterface;
		language?: ContentTypeInterface;
		theme?: 'vs' | 'vs-dark' | 'hc-black';
	}>();
</script>

<main class="my-2">
	<Monaco {block} {language} {theme} />
</main>
