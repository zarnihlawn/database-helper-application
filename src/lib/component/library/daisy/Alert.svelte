<script lang="ts">
	import { errorSvg } from '$lib/asset/image/svg/error-svg';
	import { successSvg } from '$lib/asset/image/svg/success-svg';
	import { fade, fly } from 'svelte/transition';

	export let show = false;
	export let type: 'success' | 'error' = 'success';
	export let message = '';
</script>

{#if show}
	<div
		class="fixed bottom-20 left-1/2 z-50 -translate-x-1/2"
		in:fly={{ y: 100, duration: 100 }}
		out:fade={{ duration: 100 }}
	>
		<div class="alert alert-{type}">
			{@html type === 'success' ? successSvg : errorSvg}
			<span>{message}</span>
		</div>
	</div>
{/if}
