<script lang="ts">
	import { getDatasources } from '$lib/tool/sqlite.tool';

	let allDatasource: any = [];

	async function getDatasourcesFrontend() {
		allDatasource = await getDatasources();
		console.log('allDatasource:', allDatasource);
	}

	getDatasourcesFrontend();
</script>

File Tree all. Datasource
{#if allDatasource.length > 0}
	<ul>
		{#each allDatasource as ds}
			<li>{JSON.stringify(ds)}</li>
		{/each}
	</ul>
{:else}
	<p>No datasources found.</p>
{/if}
