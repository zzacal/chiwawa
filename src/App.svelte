<script lang="ts">
  import MainNav from './navigation/MainNav.svelte';
  import Library from './pages/library/Library.svelte';
  import { getConfig } from './service/config-service';
  import type { Config } from './types';
  
  let config: Config;

  (async function init(){
    config = await getConfig();
  })();
</script>

<div class="navigable">
  <nav>
    <MainNav></MainNav>
  </nav>
  <main class="container">
    {#if config}
      <Library config={config}></Library>
    {/if}
  </main>
</div>

<style lang="scss">
  .navigable {
    display: flex;
    height: 100vh;
    
    nav {
      border-right: 1px solid #333;
    }

    main {
      flex-grow: 1;
    }
  }
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>