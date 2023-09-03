<script lang="ts">
  import Button from "../../inputs/Button.svelte";
import Dropdown from "../../inputs/Dropdown.svelte";
  import Textbox from "../../inputs/Textbox.svelte";
  import type { DropdownOption } from "../../inputs/types";
  import type { AddressEvent } from "./types";
  
  export let onSubmit: (event: AddressEvent) => void;
  
  let methods: DropdownOption[] = [
    {val:"GET", display: "GET"},
    {val:"POST", display: "POST"},
    {val:"PUT", display: "PUT"},
    {val:"DELETE", display: "DELETE"},
  ];
  let method = methods[0].val;
  let url: string;

  let onMethodChange = (name: DropdownOption): void => {method = name.val};

  let emitEvent = () => {
    onSubmit({method, url});
  }
</script>

<div class="address">
  <div class="method">
    <Dropdown style="fluid" options={methods} initial={method} onChange={onMethodChange}></Dropdown>
  </div>
  <div class="url">
    <Textbox type="text" style="fluid" bind:value={url} onAffirm={emitEvent}></Textbox>
  </div>
  <div class="send">
    <Button style="fluid" onSubmit={emitEvent}>Send</Button>
  </div>
</div>

<style lang="scss">
  .address {
    display: flex;
  }
  .method {
    display: inline-block;
    width: 7rem;
  }
  .url {
    display: inline-block;
    width: 20rem;
  }
</style>