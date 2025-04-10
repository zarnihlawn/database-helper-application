import { RouteDocumentationDashboard } from '$lib/model/data/route.data';
import type { Load } from '@sveltejs/kit';

export const load: Load = async () => {
	const routeDocumentationDashboard = RouteDocumentationDashboard;

	return { routeDocumentationDashboard };
};
