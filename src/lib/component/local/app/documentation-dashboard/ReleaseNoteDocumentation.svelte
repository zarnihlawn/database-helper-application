<script lang="ts">
	import { ReleaseNoteData } from '$lib/model/data/release-note.data';

	let releaseNotesData = ReleaseNoteData;
</script>

<main class="animate-fade-in-up min-h-screentext-sm sm:text-base">
	<h1 class="mb-8 text-4xl font-bold text-blue-600">Release Notes</h1>

	<section class="grid gap-6">
		{#each releaseNotesData as releaseNote}
			<div class="bg-info space-y-2 rounded-xl border border-gray-800 p-6 shadow-md">
				<div class="flex items-center justify-between">
					<h2 class="text-xl font-semibold">{releaseNote.title}</h2>
					<span class="text-xs">{releaseNote.date}</span>
				</div>
				<ul class="list-inside list-disc space-y-1">
					{#each releaseNote.content as feature}
						<li>{feature}</li>
					{/each}
				</ul>
			</div>
		{/each}
	</section>
</main>
