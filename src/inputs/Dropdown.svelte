<script lang="ts">
  import type { DropdownOption, Style } from "./types";

  export let options: DropdownOption[];
  export let initial: string;
  export let onChange: (value: DropdownOption) => void;
  export let style: Style = undefined;

  let value = options[0]

  const handleSelect = (selected: DropdownOption) => () => {
    value = selected;
    onChange(selected);
    close();
  }

  let state = "";
  const close = () => state = "";
  const open = () => state = "open";
</script>

<div class={`dropdown ${style}`}>
  <div class="selected" on:click={open}>
    {value.display} <small>&#x25bc;</small>
  </div>
  <ul class="options" hidden={state === "" || null}>
    {#each options as option}
      <li><a href="#" on:click={handleSelect(option)}>{option.display}</a></li>
    {/each}
  </ul>
</div>

<style lang="scss">
  @import '../colors.scss';
  .dropdown {
    cursor: pointer;
    color: var(--text);
    display: inline-block;
    position: relative;

    &.fluid {
      width: 100%;
    }

    .selected {
      border-radius: 8px;
      border: 1px solid transparent;
      font-size: 1em;
      font-weight: 500;
      font-family: inherit;
      display: block;
      padding: .6rem 2rem .6rem 1.2rem;
      color: var(--input);
      background-color: var(--input-background);
      small {
        display: inline-block;
        position: absolute;
        right: .4rem;
      }
    }

    .options {
      list-style: none;
      position: absolute;
      background-color: var(--input-background);
      border-radius: 8px;
      border: 1px solid transparent;
      top: 0;
      margin: 0;
      padding: .6rem 1.2rem;
      text-align: left;
      li {
        margin: .25rem 0;
      }
      li:hover {
        color: var(--text-highlight);
      }
    }
  }
</style>