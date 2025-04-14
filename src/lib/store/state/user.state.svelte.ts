import type { UserCookieInterface } from '$lib/model/interface/schema.interface';

export const userState = $state({
	user: null as UserCookieInterface | null,
});
