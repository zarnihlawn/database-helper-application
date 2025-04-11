import { browser } from '$app/environment';
import { ThemeEnum } from '$lib/model/enum/theme.enum';
import { themeState } from '$lib/store/state/theme.state.svelte';

const THEME_STORAGE_KEY = 'data-theme';
const DEFAULT_THEME = ThemeEnum.LIGHT;

export const getThemeFromLocalStorage = () => {
	if (browser) {
		const storedTheme = localStorage.getItem(THEME_STORAGE_KEY);
		if (storedTheme && Object.values(ThemeEnum).includes(storedTheme as ThemeEnum)) {
			themeState.theme = storedTheme as ThemeEnum;
			document.documentElement.setAttribute(THEME_STORAGE_KEY, themeState.theme);
			return storedTheme as ThemeEnum;
		} else {
			themeState.theme = DEFAULT_THEME;
			document.documentElement.setAttribute(THEME_STORAGE_KEY, themeState.theme);
			return DEFAULT_THEME;
		}
	}
	return DEFAULT_THEME;
};

export const setThemeToLocalStorage = (theme: ThemeEnum) => {
	if (browser) {
		localStorage.setItem(THEME_STORAGE_KEY, theme);
		themeState.theme = theme;
		document.documentElement.setAttribute(THEME_STORAGE_KEY, themeState.theme);
	}
};

export const removeThemeFromLocalStorage = () => {
	if (browser) {
		localStorage.removeItem(THEME_STORAGE_KEY);
		themeState.theme = DEFAULT_THEME;
		document.documentElement.setAttribute(THEME_STORAGE_KEY, themeState.theme);
	}
};
