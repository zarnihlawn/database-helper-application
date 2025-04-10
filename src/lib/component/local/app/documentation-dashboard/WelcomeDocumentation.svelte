<script lang="ts">
	import welcome from '$lib/asset/image/webp/welcome.webp';
	let username = $state('Zarni Hlawn');
</script>

<main
	class="flex flex-1 flex-col-reverse items-center justify-center gap-10 px-8 py-10 lg:flex-row"
>
	<img
		class="animate-fade-in-up w-32 drop-shadow-xl transition-transform duration-700 ease-in-out hover:scale-105 md:w-40 lg:w-[20rem] xl:w-[25rem]"
		src={welcome}
		alt="Welcome Graphic"
	/>
	<div class="animate-fade-in-up space-y-4 text-center lg:text-left">
		<h1 class="text-3xl font-bold md:text-5xl">
			Welcome Back, <span class="text-blue-600">{username}</span>!
		</h1>
		<p class="text-md md:text-lg">
			Ready to manage your data like a pro?<br />
			Let’s build something great with the
			<span class="font-semibold text-blue-500">Database Helper Application</span> — your all-in-one
			DBMS assistant.
		</p>
	</div>
</main>
