<script lang="ts">
	import { themeState } from '$lib/store/state/theme.state.svelte';
	import { setThemeToLocalStorage } from '$lib/store/safe/local-storage/theme.local-storage.svelte';
	import { ThemeEnum } from '$lib/model/enum/theme.enum'; // Import the ThemeEnum enum
	import { fontState } from '$lib/store/state/font.state.svelte';
	import {
		removeFontFromLocalStorage,
		setFontToLocalStorage
	} from '$lib/store/safe/local-storage/font.local-storage.svelte';
	import { FontEnum } from '$lib/model/enum/font.enum';

	let currentTheme = $derived(themeState.theme);
	let selectedFont = $derived(fontState.font);

	function handleThemeToggle() {
		const newTheme = currentTheme === ThemeEnum.ABYSS ? ThemeEnum.LIGHT : ThemeEnum.ABYSS;
		themeState.theme = newTheme;
		setThemeToLocalStorage(themeState.theme);
	}

	function handleFontChange(event: Event) {
		const select = event.currentTarget as HTMLSelectElement;
		fontState.font = select.value as FontEnum;
		setFontToLocalStorage(fontState.font);
	}

	function resetFont() {
		fontState.font = FontEnum.PROTO_NERD;
		setFontToLocalStorage(fontState.font);
	}
</script>

<main>
	<div class="overflow-x-auto p-[-64px]">
		<table class="table">
			<thead>
				<tr>
					<th></th>
					<th>Title</th>
					<th>Description</th>
					<th>Content</th>
				</tr>
			</thead>
			<tbody>
				<!-- Theme -->
				<tr>
					<td>1</td>
					<td>Theme</td>
					<td>Change the theme of the application.</td>
					<td>
						<label class="swap swap-rotate">
							<input
								type="checkbox"
								class="theme-controller invisible"
								checked={currentTheme === ThemeEnum.ABYSS}
								onchange={handleThemeToggle}
							/>

							<svg
								class="swap-off h-10 w-10 fill-current"
								xmlns="http://www.w3.org/2000/svg"
								viewBox="0 0 24 24"
							>
								<path
									d="M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z"
								/>
							</svg>

							<svg
								class="swap-on h-10 w-10 fill-current"
								xmlns="http://www.w3.org/2000/svg"
								viewBox="0 0 24 24"
							>
								<path
									d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z"
								/>
							</svg>
						</label>
					</td>
				</tr>

				<!-- Font -->
				<tr>
					<td>2</td>
					<td>Font Family</td>
					<td>Change the font of the application</td>
					<td class="flex items-center">
						<select class="select select-primary" value={selectedFont} onchange={handleFontChange}>
							<option value="ProtoNerd">0xProtoNerd</option>
							<option value="Inter">Inter</option>
							<option value="NotoSerif">Noto Serif</option>
						</select>
						<button class="btn ml-2" onclick={resetFont}>Reset</button>
					</td>
				</tr>
			</tbody>
		</table>
	</div>
</main>
