<script lang="ts">
  // scss
  import "@monax-owo/style/global.scss";
  import "@monax-owo/style/util.scss";
  // style
  import "$lib/style.scss";
  // rest css
  import "sanitize.css/assets.css";
  import "sanitize.css/sanitize.css";
  // fonts
  // Montserrat
  // Supports weights 100-900
  import "@fontsource-variable/montserrat";
  // Noto Sans JP
  // Supports weights 100-900
  import "@fontsource-variable/noto-sans-jp";
  // Fira Code
  // Supports weights 300-700
  import "@fontsource-variable/fira-code";
  import { state } from "$lib/stores/state";
  import { commands, events } from "$lib/generated/specta/bindings";
  import { unwrap } from "$lib/util/wrap";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  //
  onMount(async () => {
    const f = async () => {
      state.set(unwrap(await commands.getState()));
      console.log(JSON.stringify($state, null, 2));
    };
    f();
    await events.updateState(getCurrentWebviewWindow()).listen(() => f());
  });
</script>

<!-- <svelte:window on:contextmenu|preventDefault={() => console.log("contextmenu")} /> -->

<!-- TODO:ctrlとその他のlayoutを分ける -->
<div id="app">
  <slot></slot>
</div>

<style lang="scss">
  :root {
    font-size: 20px;
  }
  :global(body) {
    background-color: var(--bg);
    overflow: hidden;
    color: var(--text);
    user-select: none;
  }
  :global(::selection) {
    background-color: var(--highlight-bg);
    color: var(--bg);
  }
  :global(kbd, code) {
    font-family: "Fira Code Variable", monospace;
  }
  *::-webkit-scrollbar-track {
    border-radius: 1px;
    background-color: #e4e4e4;
  }
  #app {
    display: flex;
    justify-content: center;
    width: 100vw;
    height: 100vh;
    // pointer-events: none;
  }

  /* montserrat-latin-wght-normal */
  @font-face {
    font-style: normal;
    font-weight: 100 900;
    size-adjust: 110%;
    src: url(@fontsource-variable/montserrat/files/montserrat-latin-wght-normal.woff2)
      format("woff2-variations");
    font-family: "Montserrat Variable";
    font-display: swap;
    unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+0304,
      U+0308, U+0329, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF,
      U+FFFD;
  }

  /* fira-code-latin-wght-normal */
  @font-face {
    font-style: normal;
    font-weight: 300 700;
    size-adjust: 116%;
    src: url(@fontsource-variable/fira-code/files/fira-code-latin-wght-normal.woff2)
      format("woff2-variations");
    font-family: "Fira Code Variable";
    font-display: swap;
    unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+0304,
      U+0308, U+0329, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF,
      U+FFFD;
  }
</style>
