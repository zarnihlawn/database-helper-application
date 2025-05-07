<script lang="ts">
	let { action, keyword, optionalReminder, onClose, doAction } = $props<{
		action: string;
		keyword: string;
		optionalReminder?: string;
		onClose: () => void;
		doAction: () => void;
	}>();

	let typeInKeyword = $state('');
	let isKeywordCorrect = $state(true);

	$effect(() => {
		if (typeInKeyword !== keyword) {
			isKeywordCorrect = false;
		} else {
			isKeywordCorrect = true;
		}
	});

	function handleClose() {
		onClose();
	}
	function handleConfirm() {
		if (typeInKeyword === keyword) {
			doAction();
		}
	}
</script>

<div class="modal modal-open">
	<div
		class="modal-box flex h-[35vh] w-11/12 max-w-2xl flex-col justify-between gap-3"
	>
		<h2 class="text-2xl font-bold">Confirm {action} Action</h2>

		<fieldset class="fieldset">
			<legend class="fieldset-legend"
				>Type in "{keyword}" to confirm this action</legend
			>
			<p class="label opacity-60">{optionalReminder}</p>
			<input
				type="text"
				class="input w-full"
				placeholder="Type here"
				bind:value={typeInKeyword}
			/>
			{#if typeInKeyword !== keyword}
				<p class="label text-error">Wrong keyword</p>
			{:else}
				<p class="label text-success">Correct keyword</p>
			{/if}
		</fieldset>

		<div class="modal-action justify-end">
			<button
				class="btn btn-success"
				onclick={handleConfirm}
				disabled={!isKeywordCorrect}>{action}</button
			>
			<button class="btn btn-error" onclick={handleClose}>Cancel</button>
		</div>
	</div>
</div>
