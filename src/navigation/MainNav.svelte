<script lang="ts">
  type Dish = {
    name: string,
    icon: string
  };
  let menu: Dish[] = [{
    name: "library",
    icon: "📚"
  }, {
    name: "notifications",
    icon: "🔔"
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
        <a href="#{dish.name}" on:click={onNavigate(dish.name)}>{dish.icon}</a>
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