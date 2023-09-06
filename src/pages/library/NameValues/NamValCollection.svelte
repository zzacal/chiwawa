<script lang="ts">
  import Button from "../../../inputs/Button.svelte";
import type { EnabledKvp } from "../types";
  import NameValuePair from "./NameValuePair.svelte";

  export let value: EnabledKvp<string, string>[];

  function tryAdd() {
    value= [...value, {
      isEnabled: true,
      key: "",
      value: ""
    }];
  }
  
</script>

{#each value ?? [] as kvp}
  <div>
    <NameValuePair bind:value={kvp}></NameValuePair>
  </div>
{/each}
<Button onSubmit={tryAdd}>Add</Button>