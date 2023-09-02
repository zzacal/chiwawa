<script lang="ts">
  import Dropdown from "../../inputs/Dropdown.svelte";
  import type { DropdownOption } from "../../inputs/types";

  let methods: DropdownOption[] = [
    {val:"GET", display: "GET"},
    {val:"POST", display: "POST"},
    {val:"PUT", display: "PUT"},
    {val:"DELETE", display: "DELETE"},
  ];
  let method = methods[0].val;

  let onMethodChange = (name: DropdownOption): void => {method = name.val};
</script>

<div class="address">
  <Dropdown options={methods} initial={method} onChange={onMethodChange} ></Dropdown>
  <input type="text" value="" />
</div>

<style lang="scss">
</style>