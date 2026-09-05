<script>
  import { onMount } from "svelte";
  import {
    isAuthenticated,
    user,
    loadUser,
    doLogout,
  } from "./lib/stores/authStore.js";
  import {
    loadServers,
    addWsMessage,
  } from "./lib/stores/chatStore.js";
  import { connectWebSocket } from "./lib/api.js";

  import Login from "./lib/components/Login.svelte";
  import Register from "./lib/components/Register.svelte";
  import ServerSidebar from "./lib/components/ServerSidebar.svelte";
  import ChannelList from "./lib/components/ChannelList.svelte";
  import ChatArea from "./lib/components/ChatArea.svelte";

  let authView = "login";
  let ws = null;

  onMount(async () => {
    await loadUser();
    if ($isAuthenticated) {
      await loadServers();
      startWebSocket();
    }
  });

  async function handleAuthSuccess() {
    await loadServers();
    startWebSocket();
  }

  function startWebSocket() {
    if (ws) ws.close();
    ws = connectWebSocket((msg) => {
      if (msg.type === "connected") return;
      addWsMessage(msg);
    });
  }

  function handleLogout() {
    if (ws) ws.close();
    doLogout();
  }
</script>

<svelte:head>
  <title>Discord Clone</title>
</svelte:head>

{#if !$isAuthenticated}
  {#if authView === "login"}
    <Login
      on:success={handleAuthSuccess}
      on:switchToRegister={() => (authView = "register")}
    />
  {:else}
    <Register
      on:success={handleAuthSuccess}
      on:switchToLogin={() => (authView = "login")}
    />
  {/if}
{:else}
  <main class="app">
    <ServerSidebar />
    <ChannelList on:logout={handleLogout} />
    <ChatArea />
  </main>
{/if}

<style>
  :global(:root) {
    --bg-primary: #313338;
    --bg-secondary: #2b2d31;
    --bg-tertiary: #1e1f22;
    --bg-modifier-hover: #35373c;
    --bg-modifier-active: #404249;
    --bg-accent: #4e505899;
    --brand: #5865f2;
    --brand-hover: #4752c4;
    --green: #23a55a;
    --red: #da373c;
    --yellow: #f0b232;
    --text-normal: #dbdee1;
    --text-muted: #949ba4;
    --text-link: #00a8fc;
    --header-primary: #f2f3f5;
    --header-secondary: #b5bac1;
    --input-bg: #1e1f22;
    --scrollbar-thin-thumb: #1a1b1e;
    --scrollbar-thin-track: #2b2d31;
    --scrollbar-auto-thumb: #1a1b1e;
    --scrollbar-auto-track: #2b2d31;
    --font-primary: "gg sans", "Noto Sans", "Helvetica Neue", Helvetica, Arial, sans-serif;
    --font-display: "gg sans", "Noto Sans", "Helvetica Neue", Helvetica, Arial, sans-serif;
  }

  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  :global(body) {
    font-family: var(--font-primary);
    background-color: var(--bg-primary);
    color: var(--text-normal);
    overflow: hidden;
    font-size: 16px;
    line-height: 1.375;
    -webkit-font-smoothing: antialiased;
  }

  :global(::-webkit-scrollbar) {
    width: 8px;
    height: 8px;
  }

  :global(::-webkit-scrollbar-track) {
    background: var(--scrollbar-thin-track);
    border-radius: 4px;
  }

  :global(::-webkit-scrollbar-thumb) {
    background: var(--scrollbar-thin-thumb);
    border-radius: 4px;
  }

  :global(::-webkit-scrollbar-thumb:hover) {
    background: #232428;
  }

  :global(::-webkit-scrollbar-corner) {
    background: transparent;
  }

  :global(::selection) {
    background-color: var(--brand);
    color: white;
  }

  .app {
    display: flex;
    height: 100vh;
    width: 100vw;
  }
</style>
