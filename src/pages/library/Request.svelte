<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import AddressBar from "./AddressBar.svelte";
  import type { Action, Address } from "./types";
  import NamValCollection from "./NameValues/NamValCollection.svelte";
  export let methods: string[];
  export let request: Action | undefined = undefined;
  
  request = request ?? {
    name: "Get",
    method: methods[0],
    url: "",
  };

  let headers = request.headers ?? [];

  let response: string = "";

  async function onAddressSubmit(address: Address) {
    // response = JSON.stringify(address);
    response = await invoke("send", {request: address})
  }

</script>

<div class="row">
    <AddressBar onSubmit={onAddressSubmit} methods={methods} method={request?.method} value={request?.url}></AddressBar>
</div>
<div>
  <NamValCollection bind:value={headers}></NamValCollection>
</div>

<div class="response">{response}</div>