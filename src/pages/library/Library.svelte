<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import type { Config } from "../../types";
  import type { Action, LibraryNode } from "./types";
  import Request from "./Request.svelte";
  import Explorer from "./Explorer/Explorer.svelte";

  export let config: Config;

  let methods: string[] = config.methods;
  let library: LibraryNode;
  let selected: Action;

  async function getLibrary() {
    const getAct = {name: "Some Get", method: "GET", url: "https://httpbin.org/get"};
    const postAct = {name: "Some Post", method: "POST", url: "https://httpbin.org/post"};
    const putAct = {name: "Some Put", method: "PUT", url: "https://httpbin.org/put"};
    const deleteAct = {name: "Some Delete", method: "DELETE", url: "https://httpbin.org/delete"};
    const defaultLib = { name: "Library", actions: [getAct, postAct, putAct, deleteAct] };

    library = {...defaultLib, children: [{...defaultLib, name: "Child"}, {...defaultLib, name: "Child2"}]}
  }

  async function select(activity: Action) {
    selected = activity;
  }

  getLibrary();
</script>
<div class="library">
  <div class="explorer">
    <Explorer library={library} onSelect={select}/>
  </div>
  <div class="actions">
    {#if methods}
      {#key selected}
      <Request methods={methods} request={selected}></Request>
      {/key}
    {/if}
  </div>
</div>

<style lang="scss">
  .library {
    display: flex;
  }
</style>
