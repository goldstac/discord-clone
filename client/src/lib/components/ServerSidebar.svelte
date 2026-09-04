<script>
  import { servers, currentServerId, selectServer } from "../stores/chatStore.js";
</script>

<aside class="server-sidebar">
  {#each $servers as server}
    <button
      class="server-icon"
      class:active={$currentServerId === server.id}
      on:click={() => selectServer(server.id)}
      title={server.name}
    >
      {server.icon}
    </button>
  {/each}
</aside>

<style>
  .server-sidebar {
    width: 72px;
    background-color: #1e1f22;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-top: 12px;
    gap: 8px;
  }

  .server-icon {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    background-color: #313338;
    color: #dbdee1;
    border: none;
    cursor: pointer;
    font-size: 16px;
    font-weight: 600;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .server-icon:hover {
    border-radius: 16px;
    background-color: #5865f2;
  }

  .server-icon.active {
    border-radius: 16px;
    background-color: #5865f2;
  }
</style>
