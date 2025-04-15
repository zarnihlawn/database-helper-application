import { browser } from '$app/environment';
import type { UserCookieInterface } from '$lib/model/interface/schema.interface';
import { userState } from '$lib/store/state/user.state.svelte';

const USER_STORAGE_KEY = 'data-user';

export const getUserFromLocalStorage = () => {
	if (browser) {
		const storedUser = localStorage.getItem(USER_STORAGE_KEY);
		if (storedUser) {
			userState.user = JSON.parse(storedUser);
			return JSON.parse(storedUser);
		}
	}
};

export const setUserToLocalStorage = (user: UserCookieInterface) => {
	if (browser) {
		localStorage.setItem(USER_STORAGE_KEY, JSON.stringify(user));
		userState.user = user;
	}
};

export const removeUserFromLocalStorage = () => {
	if (browser) {
		localStorage.removeItem(USER_STORAGE_KEY);
		userState.user = null;
	}
};
