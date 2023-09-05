<script lang="ts">
  import type { Activity, LibraryNode } from "./types";
  export let library: LibraryNode;
  export let onSelect: (selected: Activity) => void;
  const select = (selected: Activity) => () => onSelect(selected);
</script>

{#if library}
  {#if library.activities}
    {#each library.activities as activity}
      <a on:click={select(activity)} href="#">
        {activity.name}
      </a>
    {/each}
  {/if}
{/if}