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
	let showPassword = $state(false);

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
			console.log("Result :", result)
			if (result) {
				const id: number = result.id;
				const name: string = result.name;
				const email: string = result.email;
				try {

					setUserToLocalStorage({ id, name, email });
				}
				catch( error ) {

				}
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
					type={showPassword ? "text" : "password"}
					name="password"
					class="input join-item w-full"
					placeholder="Password"
					bind:value={password}
				/>
				<button
					class="btn join-item"
					type="button"
					onclick={() => (showPassword = !showPassword)}
				>
					{#if showPassword}

					Hide{:else}
Show
					{/if}
				</button>
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
			
		</fieldset>
	</form>
</main>
