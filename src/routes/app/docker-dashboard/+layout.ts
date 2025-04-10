import { RouteDockerDashboard } from "$lib/model/data/route.data";
import type { Load } from "@sveltejs/kit";

export const load: Load = async () => {
  const routeDockerDashboard = RouteDockerDashboard;

  return { routeDockerDashboard };
};