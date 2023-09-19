<script lang="ts">
  export let value: [string, string][];
</script>


<dl class="list">
  {#each value as [name, val]}
    <div>
      <dt>{name}:</dt> <dd>{val}</dd>
    </div>
  {/each}
</dl>

<style lang="scss">
  dt, dd {
    display: inline-block;
  }
  dt {
    color: var(--link);
  }
  dd {
    color: var(--text-highlight);
    margin-left: 1rem;
  }
</style>