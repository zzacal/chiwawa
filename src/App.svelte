<script lang="ts">
  import Materialistic from './layout/Materialistic.svelte';
  import type { ActionTab, Tab } from './layout/types';
  import type { Action, LibraryNode } from './pages/library/types';
  import { getConfig, getLibrary, putAction } from './service/config-service';
  import type { Config } from './types';
  
  let config: Config;
  let library: LibraryNode[];
  let tabs: Tab[] = [];
  let open: Tab;

  (async function init(){
    config = await getConfig();
    library = [...await getLibrary()];
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
    library = [...await putAction(nodeId)];
    const newAction = library.find(() => true)?.actions?.find(() => true);
    if(newAction) {
      openTab(newAction);
    }
  }
</script>

<Materialistic 
  library={library}
  tabs={tabs}
  open={open}
  onExplorerSelect={openTab}
  onCloseTab={closeTab}
  onNew={addAction}
  ></Materialistic>

<style lang="scss">
</style>