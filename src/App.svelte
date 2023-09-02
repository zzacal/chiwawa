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
    <h1>Welcome to Tauri!</h1>
    <div class="row">
      <a href="https://vitejs.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
      </a>
      <a href="https://svelte.dev" target="_blank">
        <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
      </a>
    </div>
    <p>
      Click on the Tauri, Vite, and Svelte logos to learn more.
    </p>
  
    <div class="row">
      <Greet />
    </div>
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