<script lang="ts">
	import { goToRoute } from "$lib/util/router.util";
	import { onMount } from "svelte";

  onMount(() => {
    goToRoute('/app/documentation-dashboard/')
  })
</script>
