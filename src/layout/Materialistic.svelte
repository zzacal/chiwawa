<script lang="ts">
  import type { Tab } from "./types";
  import Tabbable from "./Tabbable.svelte";
  import MainNav from "../navigation/MainNav.svelte";
  import Explorer from "../pages/library/Explorer/Explorer.svelte";
  import type { Action, LibraryNode } from "../pages/library/types";

  export let library: LibraryNode;
  export let onExplorerSelect: (selected: Action) => void;
  export let onCloseTab: (id: string) => void
  export let tabs: Tab[];
  export let open: Tab | null = null;
</script>

<div class="layout materialistic">
  <nav class="nav">
    <MainNav></MainNav>
  </nav>
  
  <div class="side">
    <Explorer library={library} onSelect={onExplorerSelect}></Explorer>
  </div>
  
  <main class="main">
    <Tabbable tabs={tabs} onCloseTab={onCloseTab} open={open}></Tabbable>
  </main>
</div>
 
<style lang="scss">
  .materialistic {
    display: flex;
  }
  div.side {
    flex-shrink: 0;
  }
  nav, div.side, main {
    border: 1px solid #333;
  }
</style>
