<script lang="ts">
	import {
		catchAllDockerContainers,
		catchAllDockerImages,
		checkDockerStatus
	} from '$lib/tool/docker.tool';
	import { onDestroy, onMount } from 'svelte';

	import {
		dockerContainersState,
		dockerImagesState,
		dockerStatusState
	} from '$lib/store/state/docker.state.svelte';
	
	let dockerStalkTimeout: NodeJS.Timeout;

	async function fetchDockerStatus() {
		try {
			const result: any = await checkDockerStatus();
			dockerStatusState.status = result[0];
			if (dockerStatusState.status) {
				dockerContainersState.containers = await catchAllDockerContainers();
				dockerImagesState.images = await catchAllDockerImages();
			} else {
				dockerContainersState.containers = [];
				dockerImagesState.images = [];
			}
		} catch {
			dockerStatusState.status = false;
		}
	}

	onMount(() => {
		fetchDockerStatus();
		dockerStalkTimeout = setInterval(fetchDockerStatus, 1000);
	});

	onDestroy(() => {
		clearInterval(dockerStalkTimeout);
	});
</script>
