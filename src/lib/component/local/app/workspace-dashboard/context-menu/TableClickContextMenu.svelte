<script lang="ts">
	// pos is cursor position when right click occur
	let pos = { x: 0, y: 0 };
	// menu is dimension (height and width) of context menu
	let menu: { h: number; w: number } = { h: 0, w: 0 };
	// browser/window dimension (height and width)
	let browser: { h: number; w: number } = { h: 0, w: 0 };
	// showMenu is state of context-menu visibility
	let showMenu = $state(false);
	// to display some text
	let content: HTMLDivElement | undefined;

	let menuElement: HTMLElement | null = null; // To hold a reference to the menu element

	function rightClickContextMenu(e: MouseEvent) {
		showMenu = true;
		browser = {
			w: window.innerWidth,
			h: window.innerHeight
		};
		pos = {
			x: e.clientX,
			y: e.clientY
		};

		// Get menu dimensions *after* it's potentially visible
		$effect(() => {
			if (showMenu && menuElement) {
				getContextMenuDimension(menuElement);
				// Adjust position *after* dimensions are known
				if (browser.h - pos.y < menu.h) pos.y = pos.y - menu.h;
				if (browser.w - pos.x < menu.w) pos.x = pos.x - menu.w;
			}
		});
	}

	function onPageClick(e: MouseEvent) {
		// To make context menu disappear when
		// mouse is clicked outside context menu
		showMenu = false;
	}
	function getContextMenuDimension(node: HTMLElement) {
		// This function will get context menu dimension
		// when navigation is shown => showMenu = true
		let height = node.offsetHeight;
		let width = node.offsetWidth;
		menu = {
			h: height,
			w: width
		};
	}

	function addItem() {
		if (content) {
			content.textContent = 'Add and item...';
		}
	}
	function print() {
		if (content) {
			content.textContent = 'Printed...';
		}
	}
	function zoom() {
		if (content) {
			content.textContent = 'Zooom...';
		}
	}
	function remove() {
		if (content) {
			content.textContent = 'Removed...';
		}
	}
	function setting() {
		if (content) {
			content.textContent = 'Settings...';
		}
	}
	let menuItems = [
		{
			name: 'addItem',
			onClick: addItem,
			displayText: 'Add Item',
			class: 'fa-solid fa-plus'
		},
		{
			name: 'emptyicons',
			onClick: addItem,
			displayText: 'Empty Icon',
			class: 'fa-solid fa-square'
		},
		{
			name: 'zoom',
			onClick: zoom,
			displayText: 'Zoom',
			class: 'fa-solid fa-magnifying-glass'
		},
		{
			name: 'printMenu',
			onClick: print,
			displayText: 'Print',
			class: 'fa-solid fa-print'
		},
		{
			name: 'hr'
		},
		{
			name: 'settings',
			onClick: setting,
			displayText: 'Settings',
			class: 'fa-solid fa-gear'
		},
		{
			name: 'hr'
		},
		{
			name: 'trash',
			onClick: remove,
			displayText: 'Trash',
			class: 'fa-solid fa-trash-can'
		}
	];
</script>

<svelte:head>
	<link
		rel="stylesheet"
		href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.1.1/css/all.min.css"
		integrity="sha512-KfkfwYDsLkIlwQp6LFnl8zNdLGxu9YAA1QvwINks4PhcElQSvqcyVLLD9aMhXd13uQjoXtEKNosOWaZqXgel0g=="
		crossorigin="anonymous"
		referrerpolicy="no-referrer"
	/>
</svelte:head>

<div class="content" bind:this={content}>Right click somewhere!</div>

{#if showMenu}
	<nav
		bind:this={menuElement}
		style="position: absolute; top:{pos.y}px; left:{pos.x}px"
	>
		<div class="navbar" id="navbar">
			<ul>
				{#each menuItems as item}
					{#if item.name == 'hr'}
						<hr />
					{:else}
						<li>
							<button onclick={item.onClick}
								><i class={item.class}></i>{item.displayText}</button
							>
						</li>
					{/if}
				{/each}
			</ul>
		</div>
	</nav>
{/if}

<svelte:window
	on:contextmenu|preventDefault={rightClickContextMenu}
	onclick={onPageClick}
/>

<style>
	* {
		padding: 0;
		margin: 0;
	}
	.navbar {
		display: inline-flex;
		border: 1px #999 solid;
		width: 170px;
		background-color: #fff;
		border-radius: 10px;
		overflow: hidden;
		flex-direction: column;
	}
	.navbar ul {
		margin: 6px;
	}
	ul li {
		display: block;
		list-style-type: none;
		width: 1fr;
	}
	ul li button {
		font-size: 1rem;
		color: #222;
		width: 100%;
		height: 30px;
		text-align: left;
		border: 0px;
		background-color: #fff;
	}
	ul li button:hover {
		color: #000;
		text-align: left;
		border-radius: 5px;
		background-color: #eee;
	}
	ul li button i {
		padding: 0px 15px 0px 10px;
	}
	ul li button i.fa-square {
		color: #fff;
	}
	ul li button:hover > i.fa-square {
		color: #eee;
	}
	ul li button:hover > i.warning {
		color: crimson;
	}
	:global(ul li button.info:hover) {
		color: navy;
	}
	hr {
		border: none;
		border-bottom: 1px solid #ccc;
		margin: 5px 0px;
	}
</style>
