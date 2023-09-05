<script lang="ts">
  import { onEnterKey } from "../utilities";
  import type { InputType, Style } from "./types";

  export let type: InputType;
  export let style: Style = undefined;
  export let value: string;
  export let onAffirm: () => void;

  // export let onAffirm: (value: string) => void;
</script>

{#if type == "text"}
  <input class={style} type="text" bind:value={value} on:keyup={onEnterKey(onAffirm)}/>
{:else if type == "number"}
  <input class={style} type="number" bind:value={value} on:keyup={onEnterKey(onAffirm)}/>
{/if}
<style lang="scss">
  input {
    color: var(--input);
    background-color: var(--input-background);
  }
  .fluid {
    width: 100%;
    box-sizing: border-box;
  }
</style>
