<script lang="ts">
  import type { ChiResponse } from "./types";

  export let value: ChiResponse;
</script>
<div>
  <div class="body">{value.body}</div>
  <div class="headers">
    <dl>
      {#each value.headers ?? [] as [name, val]}
      <dt>{name}</dt> <dd>{val}</dd>
      {/each}
    </dl>
  </div>
</div>