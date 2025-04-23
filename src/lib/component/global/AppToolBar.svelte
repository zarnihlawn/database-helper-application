<script lang="ts">
	import { exitApplication, minimizeApplication, maximizeApplication } from '$lib/util/window.util';

	import { goToRoute } from '$lib/util/router.util';
	import { checkForAppUpdates } from '$lib/util/update.util';
</script>

<div class="navbar bg-base-100 shadow-sm" data-tauri-drag-region>
	<div class="navbar-start">
		<div class="dropdown">
			<button aria-label="Menu" class="btn btn-ghost">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-5 w-5"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M4 6h16M4 12h8m-8 6h16"
					/>
				</svg>
			</button>
			<ul class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1 w-52 shadow">
				<li>
					<button class="btn btn-primary" onclick={() => goToRoute('/auth/profile')}>Account</button
					>
				</li>
				<li>
					<button class="btn btn-primary" onclick={() => checkForAppUpdates(true)}>
						Check for Updates
					</button>
				</li>
			</ul>
		</div>
		<h1 class="btn btn-ghost text-xl">Database Helper Application</h1>
	</div>

	<div class="navbar-end gap-1">
		<div class="tooltip tooltip-left" data-tip="Minimize Application">
			<button
				aria-label="Minimize Application"
				class="btn btn-circle"
				onclick={minimizeApplication}
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					class="h-6 w-6"
				>
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 12h16" />
				</svg>
			</button>
		</div>
		<div class="tooltip tooltip-left" data-tip="Maximize Application">
			<button
				aria-label="Maximize Application"
				class="btn btn-circle"
				onclick={maximizeApplication}
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					class="h-6 w-6"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 4v16m8-8H4"
					/>
				</svg>
			</button>
		</div>
		<div class="tooltip tooltip-left tooltip-error" data-tip="Close Application">
			<button
				aria-label="Close Application"
				class="btn btn-error btn-circle"
				onclick={exitApplication}
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					class="h-6 w-6"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M6 18L18 6M6 6l12 12"
					/>
				</svg>
			</button>
		</div>
	</div>
</div>
