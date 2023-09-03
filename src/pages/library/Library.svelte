<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import Address from "./Address.svelte";
  import type { AddressEvent } from "./types";
  let response = "";

  async function onAddressSubmit(address: AddressEvent) {
    // response = JSON.stringify(address);
    response = await invoke("send", {request: address.url})
  }

</script>
<Address onSubmit={onAddressSubmit}></Address>
<textarea>{response}</textarea>