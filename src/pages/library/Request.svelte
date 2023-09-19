<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import AddressBar from "./AddressBar.svelte";
  import type { Action, ChiResponse, EnabledKvp } from "./types";
  import NamValCollection from "./NameValues/NamValCollection.svelte";
  import Response from "./Response.svelte";
  import ActionTabs from "./tabs/ActionTabs.svelte";
  import { path } from "@tauri-apps/api";
  export let methods: string[];
  export let request: Action;
  
  // request = request ?? {
  //   id: "",
  //   name: "Get",
  //   method: methods[0],
  //   url: "",
  // };

  // let headers = request.headers ?? [];
  // let query = request.query ?? [];
  // let path = request.path ?? [];
  // let body = request.body;

  console.log(request)
  let response: ChiResponse;
  
  async function onRequestBodyUpdate(body: string) {
    request = {...request, body};
  }

  async function onHeaderUpdate(headers: EnabledKvp<string, string>[]) {
    request = {...request, headers};
  }

  async function onParametersUpdate({query, path}: {
    query: EnabledKvp<string, string>[];
    path: EnabledKvp<string, string>[];
  }) {
    request = {...request, query, path};
  }

  async function onAddressSubmit() {
    // response = JSON.stringify({...request, headers, parameters});
    console.log(JSON.stringify(request));
    response = await invoke("send", request);
  }
</script>

<div class="request">
  <div class="row">
      <AddressBar onSubmit={onAddressSubmit} methods={methods} method={request?.method} value={request?.url}></AddressBar>
  </div>

  <div class="details" >
    <ActionTabs tabs={[{
      type: "headers",
      label: "Headers",
      content: request.headers,
      isEditable: true,
      onUpdate: onHeaderUpdate,
    }, {
      type: "body",
      label: "Body",
      content: request.body,
      isEditable: true,
      onUpdate: onRequestBodyUpdate,
    }, {
      type: "params",
      label: "Parameters",
      content: {
        path: request.path,
        query: request.query
      },
      isEditable: true,
      onUpdate: onParametersUpdate,
    }]}></ActionTabs>
  </div>
</div>

<style lang="scss">
  .request {
    margin-top: .5rem;
  }
</style>
