<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import AddressBar from "./AddressBar.svelte";
  import type { Activity, Address } from "./types";
  export let request: Activity | undefined = undefined;
  export let methods: string[];

  let response: string = "";

  async function onAddressSubmit(address: Address) {
    // response = JSON.stringify(address);
    response = await invoke("send", {request: address})
  }

</script>

{#if methods}
  <AddressBar onSubmit={onAddressSubmit} methods={methods} method={request?.method} value={request?.url}></AddressBar>
{:else}
  Loading. . .
{/if}
<div>{response}</div>