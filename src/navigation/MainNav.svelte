<script lang="ts">
  import Book from "~icons/eva/book-open-fill";
  import Bell from "~icons/eva/bell-fill";
  import type { ComponentType } from "svelte";

  type Dish = {
    name: string,
    icon: ComponentType
  };
  let menu: Dish[] = [{
    name: "library",
    icon: Book
  }, {
    name: "notifications",
    icon: Bell
  }];

  let active = menu[0];

  const isActive = (name: string) => active.name === name;

  const onNavigate = (name: string) => () => 
    (active.name != name) && (active = menu.find(d => d.name === name) ?? active);
</script>
<ul class="expanded">
  {#key active}
    {#each menu as dish}
      <li class={ isActive(dish.name) ? "active" : ""}>
        <a href="#{dish.name}" on:click={onNavigate(dish.name)}><svelte:component this={dish.icon} /></a>
      </li>
    {/each}
  {/key}
</ul>

<style lang="scss">
  ul {
    list-style: none;
    padding-left: unset;
    
    a {
      display: inline-block;
      padding: 8px;
      color: var(--text-highlight);

      &:hover {
        filter: drop-shadow(0 0 .5em #bbb);
      }
    }

    li.active {
      border-left: 2px solid #bbb;
      background-color: #444;
      
      a {
        padding-left: 6px;
      }
      a:hover {
        filter: unset;
      }
    }
  }
</style>