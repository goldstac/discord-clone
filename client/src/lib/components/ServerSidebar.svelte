<script>
  import {
    servers,
    currentServerId,
    selectServer,
    createNewServer,
  } from "../stores/chatStore.js";
  import { user } from "../stores/authStore.js";

  let showCreate = false;
  let newServerName = "";

  async function handleCreate() {
    if (newServerName.trim()) {
      await createNewServer(newServerName.trim());
      newServerName = "";
      showCreate = false;
    }
  }

  function handleClick(server) {
    selectServer(server.id);
  }
</script>

<aside class="server-sidebar">
  {#each $servers as server}
    <button
      class="server-icon"
      class:active={$currentServerId === server.id}
      on:click={() => handleClick(server)}
      title={server.name}
    >
      {server.name.charAt(0).toUpperCase()}
    </button>
  {/each}

  <button class="server-icon add" on:click={() => (showCreate = !showCreate)}>
    +
  </button>

  {#if showCreate}
    <div class="create-form">
      <input
        type="text"
        placeholder="Server name"
        bind:value={newServerName}
        on:keydown={(e) => e.key === "Enter" && handleCreate()}
      />
      <button on:click={handleCreate}>Create</button>
    </div>
  {/if}

  <div class="spacer"></div>

  {#if $user}
    <div class="user-badge" title={$user.username}>
      {$user.username.charAt(0).toUpperCase()}
    </div>
  {/if}
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
    overflow-y: auto;
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

  .server-icon.add {
    background-color: #313338;
    color: #23a55a;
    font-size: 24px;
  }

  .server-icon.add:hover {
    background-color: #23a55a;
    color: white;
  }

  .create-form {
    position: absolute;
    left: 72px;
    background: #2b2d31;
    padding: 12px;
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    z-index: 10;
  }

  .create-form input {
    padding: 8px;
    border-radius: 4px;
    border: none;
    background: #1e1f22;
    color: #dbdee1;
    font-size: 14px;
  }

  .create-form button {
    padding: 8px;
    border-radius: 4px;
    border: none;
    background: #5865f2;
    color: white;
    cursor: pointer;
    font-weight: 600;
  }

  .spacer {
    flex: 1;
  }

  .user-badge {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    background-color: #5865f2;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 16px;
    margin-bottom: 12px;
  }
</style>
