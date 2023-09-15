<script lang="ts">
  import Request from "../pages/library/Request.svelte";
  import type { Tab } from "./types";

  export let tabs: Tab[] = [];
  export let onCloseTab: (id: string) => void;
  export let open: Tab | null;

  function openTab(tab: Tab) {
    return function handleSetActive() {
      open = tab;
    }
  }

  function remove(id: string) {
    return function() {
      if(open?.id === id)
      {
        open = null;
      }
      
      onCloseTab(id);
    }
  }
</script>
<ul class="tabs">
  {#each tabs as tab }
    <li class="tab {tab === open ? "active": ""}">
      <a href="#{tab.label}" on:click={openTab(tab)}>
        {tab.label}
      </a>
      <button on:click={remove(tab.id)}>X</button>
  </li>
  {/each}
</ul>

<div class="active">
  {#key open}
    {#if open?.type === "request"}
    <Request methods={open.methods} request={open.action}></Request>
    {/if}
  {/key}
</div>

<style lang="scss">
  .tabs {
    list-style: none;
    padding-left: unset;
    border-bottom: 1px solid var(--background-emphasis);
    .tab {
      display: inline-block;
      padding: .1rem .3rem;
      border-top-right-radius: .5rem;
      border-top-left-radius: .5rem;
      
      &:hover {
        background-color: var(--background-preemph);
      }

      &.active {
        background-color: var(--background-emphasis);
      }
      
      a {
        color: var(--text-highlight);
      }
    }
  }
</style>
