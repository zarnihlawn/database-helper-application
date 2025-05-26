<script lang="ts">
	import type { UserInterface } from '$lib/model/interface/schema.interface';
	import { setUserToLocalStorage } from '$lib/store/safe/local-storage/user.local-storage.svelte';
	import { goToRoute } from '$lib/util/router.util';
	import { invoke } from '@tauri-apps/api/core';

	let email = $state('');
	let password = $state('');

	async function handleSubmit() {
		const result: UserInterface = await invoke('get_user_by_email', { email, password });
		if (result) {
			const id: number = result.id;
			const name: string = result.name;
			const email: string = result.email;
			try {

				setUserToLocalStorage({ id, name, email });
			}
			catch (error ){

			}
			goToRoute('/auth/profile');
		}
	}
</script>

<main class="flex h-full w-full items-center justify-center">
	<form>
		<fieldset class="fieldset bg-base-200 border-base-300 rounded-box w-md border p-4">
			<legend class="fieldset-legend">
				<h2>
					<span class="text-xl font-bold">Login</span>
				</h2>
			</legend>

			<label for="email" class="fieldset-label">Email</label>
			<input
				type="email"
				name="email"
				id="email"
				class="input w-full"
				placeholder="Email"
				bind:value={email}
			/>

			<label for="password" class="fieldset-label">Password</label>
			<input
				type="password"
				name="password"
				id="password"
				class="input w-full"
				placeholder="Password"
				bind:value={password}
			/>

			<button class="btn btn-primary mt-4 mb-2" onclick={handleSubmit}>Login</button>
			<p>
				Don't have an account? <a href="/auth/signup" class="link text-primary">Signup</a>
			</p>
			
		</fieldset>
	</form>
</main>
