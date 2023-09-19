<script lang="ts">
  import Button from "../../../inputs/Button.svelte";
import type { EnabledKvp } from "../types";
  import NameValuePair from "./NameValuePair.svelte";

  export let value: EnabledKvp<string, string>[];
  let local: EnabledKvp<string, string>[];

  function tryAdd() {
    // This magically does not add new items if the last item is empty.
    // Unsettling, but also kinda cool
    local = [...value, {
      isEnabled: true,
      key: "",
      value: ""
    }];
  }
  $: {
    value = local?.filter(kvp => kvp.key?.length > 0);
  }

  tryAdd();
</script>

{#each local ?? [] as kvp}
  <div>
    <NameValuePair bind:value={kvp}></NameValuePair>
  </div>
{/each}
<Button onSubmit={tryAdd}>Add</Button>