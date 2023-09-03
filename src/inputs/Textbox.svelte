<script lang="ts">
  import type { InputType, Style } from "./types";

  export let type: InputType;
  export let style: Style = undefined;
  export let value: string;

  // export let onAffirm: (value: string) => void;
</script>

{#if type == "text"}
  <input class={style} type="text" bind:value={value}/>
{:else if type == "number"}
  <input class={style} type="number" bind:value={value}/>
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
