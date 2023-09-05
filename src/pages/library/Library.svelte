<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import type { Config } from "../../types";
  import type { Activity, LibraryNode } from "./types";
  import Request from "./Request.svelte";
  import Explorer from "./Explorer.svelte";
  let methods: string[];
  let library: LibraryNode;
  let selected: Activity;

  async function initialize() {
    const config: Config = await invoke("initialize");
    methods = config.methods;
  };

  async function getLibrary() {
    const defaultAct = {name: "default", method: "DELETE", url: "https://httpbin.org/get"};
    library = { name: "Library", activities: [defaultAct] };
  }

  async function select(activity: Activity) {
    selected = activity;
  }

  initialize();
  getLibrary();
</script>
<Explorer library={library} onSelect={select}/>
{#key selected}
  <Request methods={methods} request={selected}></Request>
{/key}