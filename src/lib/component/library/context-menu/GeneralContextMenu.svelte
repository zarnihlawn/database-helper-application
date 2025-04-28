<script lang="ts">
	import { onMount } from 'svelte';

	export let x = 0;
	export let y = 0;
	export let show = false;

	let menuElement: HTMLDivElement;

	function handleClickOutside(event: MouseEvent) {
		if (menuElement && !menuElement.contains(event.target as Node)) {
			show = false;
		}
	}

	onMount(() => {
		document.addEventListener('click', handleClickOutside);
		return () => {
			document.removeEventListener('click', handleClickOutside);
		};
	});

	
</script>

{#if show}
	<div
		bind:this={menuElement}
		class="context-menu"
		style="left: {x}px; top: {y}px;"
	>
		<div class="menu-item">Option 1</div>
		<div class="menu-item">Option 2</div>
		<div class="menu-item">Option 3</div>
	</div>
{/if}

<style>
	.context-menu {
		position: fixed;
		background: white;
		border: 1px solid #ccc;
		border-radius: 4px;
		padding: 8px 0;
		min-width: 150px;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
		z-index: 1000;
	}

	.menu-item {
		padding: 8px 16px;
		cursor: pointer;
	}

	.menu-item:hover {
		background-color: #f0f0f0;
	}
</style>
