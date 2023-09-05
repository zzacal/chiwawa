<script lang="ts">
  import { onAffirmativeKey } from "../utilities";
  import type { Style } from "./types";

  export let style: Style = undefined;
  export let onSubmit: () => void;
</script>

<button class={style} on:click={onSubmit} on:keyup={onAffirmativeKey(onSubmit)}><slot></slot></button>

<style lang="scss">
  button {
    color: var(--button);
    background-color: var(--button-background);
  }
  .fluid {
    width: 100%;
  }
</style>