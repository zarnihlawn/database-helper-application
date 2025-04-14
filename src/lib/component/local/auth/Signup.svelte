<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import type { UserInterface } from '$lib/model/interface/schema.interface';
	import { goToRoute } from '$lib/util/router.util';
	import { setUserToLocalStorage } from '$lib/store/safe/local-storage/user.local-storage.svelte';
	import { userState } from '$lib/store/state/user.state.svelte';
	import { onMount } from 'svelte';

	let name = $state('');
	let email = $state('');
	let password = $state('');
	let confirmPassword = $state('');

	let user = $derived(userState.user);

	onMount(() => {
		if (user) {
			goToRoute('/auth/profile');
		}
	});

	async function handleSubmit() {
		if (password !== confirmPassword) {
			return;
		} else {
			await invoke('signup_user', { name, email, password });
			const result: UserInterface = await invoke('get_user_by_email', { email, password });
			if (result) {
				const name = result.name;
				const email = result.email;
				setUserToLocalStorage({ name, email });
				goToRoute('/auth/profile');
			}
		}
	}
</script>

<main class="flex h-full w-full items-center justify-center">
	<form>
		<fieldset class="fieldset bg-base-200 border-base-300 rounded-box w-md border p-4">
			<legend class="fieldset-legend">
				<h2>
					<span class="text-xl font-bold">Sign Up</span>
				</h2></legend
			>

			<label for="name" class="fieldset-label">Name</label>
			<input type="text" name="name" class="input w-full" placeholder="Name" bind:value={name} />

			<label for="email" class="fieldset-label">Email</label>
			<input
				type="email"
				name="email"
				class="input w-full"
				placeholder="Email"
				bind:value={email}
			/>

			<label for="password" class="fieldset-label">Password</label>
			<div class="join">
				<input
					type="password"
					name="password"
					class="input join-item w-full"
					placeholder="Password"
					bind:value={password}
				/>
				<button class="btn join-item">Show</button>
			</div>

			<label for="confirmPassword" class="fieldset-label">Confirm Password</label>
			<input
				type="password"
				name="confirmPassword"
				class="input w-full"
				placeholder="Confirm Password"
				bind:value={confirmPassword}
			/>

			<button class="btn btn-primary mt-4 mb-2" onclick={handleSubmit}>Sign Up</button>
			<p>
				Already have an account? <a href="/auth/login" class="link text-primary">Login</a>
			</p>
			<p>
				Forgot password? <a href="/auth/reset-password" class="link text-primary">Reset</a>
			</p>
		</fieldset>
	</form>
</main>
