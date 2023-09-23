<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import AddressBar from "./AddressBar.svelte";
  import type { Action, ChiResponse, EnabledKvp } from "./types";
  import ActionTabs from "./tabs/ActionTabs.svelte";
  export let methods: string[];
  export let request: Action;

  console.log(request);
  let response: ChiResponse;

  async function onRequestBodyUpdate(body: string) {
    request = { ...request, body };
  }

  async function onHeaderUpdate(headers: EnabledKvp<string, string>[]) {
    request = { ...request, headers };
  }

  async function onParametersUpdate({
    query,
    path,
  }: {
    query: EnabledKvp<string, string>[];
    path: EnabledKvp<string, string>[];
  }) {
    request = { ...request, query, path };
  }

  async function onAddressSubmit() {
    // response = JSON.stringify({...request, headers, parameters});
    console.log(JSON.stringify(request));
    response = await invoke("send", { request });
  }
</script>

<div class="action">
  <div class="request">
    <div class="row">
      <AddressBar
        onSubmit={onAddressSubmit}
        {methods}
        method={request?.method}
        value={request?.url}
      />
    </div>
    <div class="details">
      <ActionTabs
        tabs={[
          {
            type: "headers",
            label: "Headers",
            content: request.headers,
            isEditable: true,
            onUpdate: onHeaderUpdate,
          },
          {
            type: "body",
            label: "Body",
            content: request.body,
            isEditable: true,
            onUpdate: onRequestBodyUpdate,
          },
          {
            type: "params",
            label: "Parameters",
            content: {
              path: request.path,
              query: request.query,
            },
            isEditable: true,
            onUpdate: onParametersUpdate,
          },
        ]}
      />
    </div>
  </div>
  <div class="response">
    <h2>Response</h2>
    {#key response}
      {#if response}
        <div class="details">
          <ActionTabs
            tabs={[
              {
                type: "response-body",
                label: "Body",
                content: response.body,
                isEditable: false,
              },
              {
                type: "response-headers",
                label: "Headers",
                content: response.headers,
                isEditable: true,
              },
            ]}
          />
        </div>
      {/if}
    {/key}
  </div>
</div>

<style lang="scss">
  .request, .response {
    margin: 0.5rem;

    display: grid;
    grid-template-rows: 50px 100%;
  }
  .details {
    display: grid;
    grid-template-rows: 40px;
  }
  .response {    
    h2 {
      line-height: 47px;
      margin: unset;
    }
  }
  .action {
    @media screen and (min-width: 1400px) {
      display: grid;
      justify-content: stretch;
      align-content: stretch;
      grid-template-columns: 50% 50%;
    }
  }
</style>
