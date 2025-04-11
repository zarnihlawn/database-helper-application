import { browser } from '$app/environment';
import { FontEnum } from '$lib/model/enum/font.enum';
import { fontState } from '$lib/store/state/font.state.svelte';

const FONT_STORAGE_KEY = 'data-font';
const DEFAULT_FONT = FontEnum.PROTO_NERD;

export const getFontFromLocalStorage = () => {
	if (browser) {
		const storedFont = localStorage.getItem(FONT_STORAGE_KEY);
		if (storedFont && Object.values(FontEnum).includes(storedFont as FontEnum)) {
			fontState.font = storedFont as FontEnum;
			document.documentElement.setAttribute(FONT_STORAGE_KEY, fontState.font);

			return storedFont as FontEnum;
		} else {
			fontState.font = DEFAULT_FONT;
			document.documentElement.setAttribute(FONT_STORAGE_KEY, fontState.font);
			return DEFAULT_FONT;
		}
	}
	return DEFAULT_FONT;
};

export const setFontToLocalStorage = (font: FontEnum) => {
	if (browser) {
		localStorage.setItem(FONT_STORAGE_KEY, font);
		fontState.font = font;
		document.documentElement.setAttribute(FONT_STORAGE_KEY, fontState.font);
	}
};

export const removeFontFromLocalStorage = () => {
	if (browser) {
		localStorage.removeItem(FONT_STORAGE_KEY);
		fontState.font = DEFAULT_FONT;
		document.documentElement.setAttribute(FONT_STORAGE_KEY, fontState.font);
	}
};
