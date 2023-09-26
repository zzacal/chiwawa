<script lang="ts">
  import Materialistic from './layout/Materialistic.svelte';
  import type { ActionTab, Tab } from './layout/types';
  import Library from './pages/library/Library.svelte';
  import type { Action, LibraryNode } from './pages/library/types';
  import { getConfig, putAction } from './service/config-service';
  import type { Config } from './types';
  
  let config: Config;
  let tabs: Tab[] = [];
  let open: Tab;

  (async function init(){
    config = await getConfig();
    console.log(config.libraries);
  })();

  function openTab(action: Action) {
    let tab = tabs.find(t => t.id == action.id);
    if (!tab) {
      tab = {
        id: action.id,
        label: action.name,
        type: "request",
        methods: config.methods,
        action: action
      }
      tabs = [tab, ...tabs]
    }

    open = tab;
  }

  function closeTab(id: string) {
    tabs = tabs.filter(t => t.id != id);
  }

  async function addAction(nodeId: string | null = null) {
    // TODO: add a library
    // library = [...await putAction(nodeId)];
    // const newAction = library.find(() => true)?.actions?.find(() => true);
    // if(newAction) {
    //   openTab(newAction);
    // }
  }
</script>

{#if config && config.libraries}
  <Materialistic 
    library={config.libraries}
    tabs={tabs}
    open={open}
    onExplorerSelect={openTab}
    onCloseTab={closeTab}
    onNew={addAction}
    ></Materialistic>
{/if}
<style lang="scss">
</style>