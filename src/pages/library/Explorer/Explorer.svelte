<script lang="ts">
  import type { Action, LibraryNode } from "../types";
  import Node from "./Node.svelte";
  import AddAction from "~icons/eva/plus-square-outline";
  import Save from "~icons/eva/save-outline";
  export let library: LibraryNode[];
  export let onSelect: (selected: Action) => void;
  export let onNew: (nodeId: string | null) => void;

  const onAdd = (nodeId: string | null = null) => () => onNew(nodeId);
</script>

<div class="explorer">
  <nav class="menu">
    <button on:click={()=>{}} >
      <Save />
    </button>
    <button on:click={onAdd()} >
      <AddAction />
    </button>
  </nav>
  <div class="nodes">
    {#each library ?? [] as lib }
      <Node library={lib} onSelect={onSelect}></Node>
    {/each}
  </div>
</div>

<style lang="scss">
  .nodes {
    margin: 1rem;
  }
  .menu {
    display: flex;
    justify-content: center;

    border-bottom: 1px solid #333;
    height: 2rem;
    margin-top: .25rem;

    button {
      padding: 0;
      line-height: 1rem;

      &:focus, &:hover {
        background-color: var(--button-background);
        color: var(--button);
      }
    }
  }
</style>
