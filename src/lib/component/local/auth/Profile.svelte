<script lang="ts">
	import { removeUserFromLocalStorage } from '$lib/store/safe/local-storage/user.local-storage.svelte';
	import { userState } from '$lib/store/state/user.state.svelte';
	import { goToRoute } from '$lib/util/router.util';
	import { onMount } from 'svelte';

	let user = $derived(userState.user);

	onMount(() => {
		if (!user) {
			goToRoute('/auth/login');
		}
	});

	function handleLogout() {
		removeUserFromLocalStorage();
		goToRoute('/auth/login');
	}
</script>

<main class="flex h-full w-full flex-col items-center justify-center gap-5">
	<div class="avatar">
		<div class="w-24 rounded-xl">
			<img src="/favicon.svg" alt="logo" />
		</div>
	</div>
	<h1>Your profile</h1>
	<p class="text-md opacity-80">{user?.name}</p>
	<p class="text-md opacity-80">{user?.email}</p>
	<form>
		<button class="btn btn-error" onclick={handleLogout}>Logout</button>
	</form>
</main>
