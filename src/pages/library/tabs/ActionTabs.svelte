<script lang="ts">
  import NameValCollection from "../NameValues/NameValCollection.svelte";
  import NameValDisplay from "../NameValues/NameValDisplay.svelte";
  import type { ActionTabContent } from "./tab-types";
  export let tabs: ActionTabContent[];
  let active: ActionTabContent = tabs[0];

  function activate(tab: ActionTabContent) {
    return function () {
      active = tab;
    };
  }
</script>

<ul class="tabs">
  {#each tabs as tab}
    <li class={`tab ${tab === active ? "active" : ""}`}>
      <a href="#{tab.label}" on:click={activate(tab)}>
        {tab.label}
      </a>
    </li>
  {/each}
</ul>

<div class="active">
  {#if active.type === "body"}
    <h3>{active.label}</h3>
    <textarea>{active.content}</textarea>
  {:else if active.type === "headers"}
    <h3>{active.label}</h3>
    <NameValCollection bind:value={active.content} />
  {:else if active.type === "params"}
    <h3>{active.label}</h3>
    {#if active.content.path}
      <h3>Query</h3>
      <NameValCollection bind:value={active.content.query} />
      {#if active.content.path}
        <h3>Path</h3>
        <NameValCollection bind:value={active.content.path} />
      {/if}
    {/if}
  {:else if active.type==="response-body"}
    <h3>{active.label}</h3>
    <textarea>{active.content}</textarea>
  {:else if active.type === "response-headers"}
    <h3>{active.label}</h3>
    <NameValDisplay value={active.content}></NameValDisplay>
  {/if}
</div>

<style lang="scss">
  .tabs {
    list-style: none;
    padding: 0 1rem;
    margin-bottom: 0;
    display: flex;
    gap: 0.3rem;
    border-bottom: 1px solid var(--background-emphasis);
  }
  .tab {
    padding: 0 1rem;


    &:hover {
        background-color: var(--background-preemph);
      }

      &.active {
        background-color: var(--background-emphasis);
        a {
          color: var(--tab-active);
        }
      }
  }
</style>
