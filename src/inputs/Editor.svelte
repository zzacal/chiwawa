<script lang="ts">
  export let value: string;
    import CodeMirror from "svelte-codemirror-editor";
    import { javascript } from "@codemirror/lang-javascript";
    import { oneDark } from "@codemirror/theme-one-dark";
</script>

<div class="editor">
  <CodeMirror bind:value lang={javascript()} theme={oneDark} />
</div>
