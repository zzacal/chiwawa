<script lang="ts">
  import Request from "../pages/library/Request.svelte";
  import type { Tab } from "./types";
  import Close from "~icons/eva/close-fill";
  import Save from "~icons/eva/save-outline";
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
<nav class="actions">
  <div class="menu">
    <button on:click={() => console.log("imma save now")}>
      <Save />
    </button>
  </div>
  <ul class="tabs">
    {#each tabs as tab }
      <li class="tab {tab === open ? "active": ""}">
        <a href="#{tab.label}" on:click={openTab(tab)}>
          {tab.label}
        </a>
        <button class="close" on:click={remove(tab.id)}>
          <Close />
        </button>
    </li>
    {/each}
  </ul>
</nav>
<div class="active">
  {#key open}
    {#if open?.type === "request"}
    <Request methods={open.methods} request={open.action}></Request>
    {/if}
  {/key}
</div>

<style lang="scss">
  nav {
    display: flex;
    align-items: center;

    margin-top: .25rem;
    border-bottom: 1px solid var(--background-emphasis);

    .menu {
      padding: 0 .5rem;
    }

    button {
      padding: unset;
      margin: unset;
      line-height: 1rem;
      border-radius: .3rem;

      &:focus, &:hover {
        background-color: var(--button-background);
        color: var(--button);
      }
    }
  }
  .tabs {
    display: flex;
    gap: .15rem;

    margin: unset;
    list-style: none;
    padding: unset;
    .tab {
      display: flex;
      gap: .5rem;

      padding: .25rem .3rem;
      border-top-right-radius: .5rem;
      border-top-left-radius: .5rem;
      border-right: 1px solid var(--background-preemph);
      
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
