<script lang="ts">
  import type { Action, LibraryNode } from "../types";
  import Node from "./Node.svelte";
  export let library: LibraryNode;
  export let onSelect: (selected: Action) => void;
</script>

<div class="explorer">
  <Node library={library} onSelect={onSelect}></Node>
</div>

<style lang="scss">
  .explorer {
    margin: 1rem;
  }
</style>
