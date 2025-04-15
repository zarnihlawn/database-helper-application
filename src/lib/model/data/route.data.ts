// Imports
//#region 
import { AppearanceSvg } from '$lib/asset/image/svg/appearance-svg';
import { ContainerSvg } from '$lib/asset/image/svg/container-svg';
import { GeneralSettingsSvg } from '$lib/asset/image/svg/general-settings-svg';
import { ImageSvg } from '$lib/asset/image/svg/image-svg';
import { MaintenanceSvg } from '$lib/asset/image/svg/maintenance-svg';
import { PullSvg } from '$lib/asset/image/svg/pull-svg';
import { ReleaseNotes } from '$lib/asset/image/svg/release-notes-svg';
import { ShortcutKeySvg } from '$lib/asset/image/svg/shortcut-key-svg';
import { TerminalSvg } from '$lib/asset/image/svg/terminal-svg';
import { UserManualSvg } from '$lib/asset/image/svg/user-manual-svg';
import { WelcomeSvg } from '$lib/asset/image/svg/welcome-svg';
import type { RouteInterface } from '../interface/route.interface';
// #endregion

// Dashboard Route data use as main routes in DOCK NAVIGATION
//#region
export const RouteDocumentationDashboard: RouteInterface[] = [
	{
		name: 'Welcome',
		icon: WelcomeSvg,
		url: '/app/documentation-dashboard/welcome-documentation'
	},
	{
		name: 'User Manual',
		icon: UserManualSvg,
		url: '/app/documentation-dashboard/user-manual-documentation'
	},
	{
		name: 'Maintenance',
		icon: MaintenanceSvg,
		url: '/app/documentation-dashboard/maintenance-documentation'
	},
	{
		name: 'Release Notes',
		icon: ReleaseNotes,
		url: '/app/documentation-dashboard/release-notes-documentation'
	}
];
//#endregion

// Settings dashboard data use as child routes with TABS NAVIGATION
//#region
export const RouteSettingsDashboard: RouteInterface[] = [
	{
		name: 'General',
		icon: GeneralSettingsSvg,
		url: '/app/settings-dashboard/general-settings'
	},
	{
		name: 'Appearance',
		icon: AppearanceSvg,
		url: '/app/settings-dashboard/appearance-settings'
	},
	{
		name: 'Shortcut',
		icon: ShortcutKeySvg,
		url: '/app/settings-dashboard/shortcut-settings'
	}
];
//#endregion

// Docker dashboard data use as child routes with TABS NAVIGATION
//#region
export const RouteDockerDashboard: RouteInterface[] = [
	{
		name: 'Containers',
		icon: ContainerSvg,
		url: '/app/docker-dashboard/docker-containers'
	},
	{
		name: 'Images',
		icon: ImageSvg,
		url: '/app/docker-dashboard/docker-images'
	},
	{
		name: 'Pulling Images',
		icon: PullSvg(),
		url: '/app/docker-dashboard/docker-pulling-images'
	},
	{
		name: 'System Terminal',
		icon: TerminalSvg,
		url: '/app/docker-dashboard/docker-system-terminal'
	}
];
// #endregion
