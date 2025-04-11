import type { Load } from '@sveltejs/kit';
import { RouteSettingsDashboard } from '$lib/model/data/route.data';

export const load: Load = async () => {
	const routeSettingsDashboard = RouteSettingsDashboard;

	return { routeSettingsDashboard };
};
