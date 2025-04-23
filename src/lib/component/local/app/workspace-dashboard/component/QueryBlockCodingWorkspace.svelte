<script lang="ts">
	import Monaco from '$lib/component/library/monaco/Monaco.svelte';

	let { language = 'markdown', theme = 'vs-dark' } = $props<{
		language?: 'markdown' | 'sql' | 'json';
		theme?: 'vs' | 'vs-dark' | 'hc-black';
	}>();
</script>

<main class="my-2">
	<Monaco {language} {theme} />
</main>
