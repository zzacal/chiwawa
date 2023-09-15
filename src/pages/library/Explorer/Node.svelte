<script lang="ts">
  import Folder from "~icons/eva/folder-outline";
  import { onAffirmativeKey } from "../../../utilities";
  import type { Action, LibraryNode } from "../types";
  export let library: LibraryNode;
  export let onSelect: (selected: Action) => void;
  const select = (selected: Action) => () => onSelect(selected);
</script>
{#if library}
  <div class="node">
    <Folder /> {library.name}

    <ul class="actions">
      {#each library.actions ?? [] as action }
        <li>
          <a tabindex={0} href={`#${action.id}`} on:click={select(action)} on:keyup={onAffirmativeKey(select(action))}>
            {action.name}
          </a>
        </li>
      {/each}
    </ul>
    
    <ul class="nodes">
      {#each library.children ?? [] as node}
        <svelte:self library={node} onSelect={onSelect} />
      {/each}
    </ul>
  </div>
{/if}

<style lang="scss">
  .actions, .nodes {
    list-style: none;
    margin: 0;
    padding-left: .5rem;
  }
</style>
