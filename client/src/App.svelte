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
  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  :global(body) {
    font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
    background-color: #313338;
    color: #dbdee1;
    overflow: hidden;
  }

  .app {
    display: flex;
    height: 100vh;
    width: 100vw;
  }
</style>
