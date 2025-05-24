<script lang="ts">
	import type { DatabaseConnectionInterface } from "$lib/model/interface/schema.interface";
	import { getIpV4Address } from "$lib/util/network.util";
	import { onMount } from "svelte";

  let {onClose, databaseConnection} = $props<{
    onClose: () => void;
    databaseConnection: DatabaseConnectionInterface;
  }>();

  let network = $state('');

  onMount(async() => {
    network = await getIpV4Address();
  })

  function handleClose() {
    onClose();
  }
</script>

<div class="modal modal-open ">
	<div class="modal-box">
		<h3 class="text-lg font-bold">Sharing Database Connection</h3>
		

    <p class="label-text mb-5">Make sure your collaborator is in the same network.</p>

    {#if databaseConnection.datasource_id === 2}
      SQLite connection can't be changed.
    {:else }

    <ol class="flex gap-5 px-5 flex-col list-decimal">
      <li>

        {databaseConnection.url}
      </li>
      
      {#if databaseConnection.url.includes('localhost') }
        <li>
          {databaseConnection.url.replace('localhost', network)}
        </li>
        {:else if databaseConnection.url.includes('127.0.0.1')}
        <li> </li>
      {:else}
<li>
        {databaseConnection.url}
      </li>
   {/if}
      </ol>

    {/if}
		
    <div class="modal-action">
      <button class="btn btn-primary" onclick={handleClose}>Cancel</button>
    </div>
    </div>
    </div>
    