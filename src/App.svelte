<script lang="ts">
  import Greet from './lib/Greet.svelte'
  import MainNav from './navigation/MainNav.svelte';
  import Library from './pages/library/Library.svelte';
</script>

<div class="navigable">
  <nav>
    <MainNav></MainNav>
  </nav>
  <main class="container">
    <Library></Library>
    
    <!-- <div class="row">
      <Greet />
    </div> -->
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