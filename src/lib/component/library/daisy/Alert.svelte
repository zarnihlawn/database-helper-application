<script lang="ts">
	import { errorSvg } from '$lib/asset/image/svg/error-svg';
	import { successSvg } from '$lib/asset/image/svg/success-svg';
	import { onMount } from 'svelte';
	import { fade, fly } from 'svelte/transition';

	let {show, type, message} = $props<{
		show: boolean;
		type: 'success' | 'error';
		message: string;
	}>();


</script>

{#if show}
	<div
		class="alert"
		class:alert-success={type === 'success'}
		class:alert-error={type === 'error'}
	>
		{@html type === 'success' ? successSvg : errorSvg}
		<span>{message}</span>
	</div>
{/if}
