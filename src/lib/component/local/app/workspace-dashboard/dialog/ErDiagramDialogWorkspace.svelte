<script lang="ts">
	let { onClose } = $props<{
		onClose: () => void;
	}>();

	function handleClose() {
		onClose();
	}
</script>

<div class="modal modal-open">
	<div class="modal-box flex h-[50vh] w-11/12 max-w-5xl flex-col gap-3">
		<h2 class="text-2xl font-bold">ER Diagram File</h2>

		<div class="modal-action">
			<button class="btn btn-error" onclick={handleClose}>Cancel</button>
		</div>
	</div>
</div>
