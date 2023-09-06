<script lang="ts">
  import Textbox from "../../../inputs/Textbox.svelte";
import type { EnabledKvp } from "../types";

  export let value: EnabledKvp<string, string>;
</script>

<input type="checkbox" bind:checked={value.isEnabled} />
<Textbox type="text" bind:value={value.key} onAffirm={()=>{}} ></Textbox>
<Textbox type="text" bind:value={value.value} onAffirm={()=>{}} ></Textbox>
