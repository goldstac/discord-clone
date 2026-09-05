<script>
  import {
    channels,
    currentChannel,
    selectChannel,
    createNewChannel,
    currentServerId,
  } from "../stores/chatStore.js";
  import { user, doLogout } from "../stores/authStore.js";

  let showCreate = false;
  let newChannelName = "";

  async function handleCreate() {
    if (newChannelName.trim() && $currentServerId) {
      await createNewChannel($currentServerId, newChannelName.trim());
      newChannelName = "";
      showCreate = false;
    }
  }

  function handleClick(channel) {
    selectChannel(channel.id, channel.name);
  }

  function handleLogout() {
    doLogout();
  }
</script>

<aside class="channel-list">
  <header class="channel-header">
    <h2>Channels</h2>
    <button class="add-btn" on:click={() => (showCreate = !showCreate)}>
      +
    </button>
  </header>

  {#if showCreate}
    <div class="create-form">
      <input
        type="text"
        placeholder="Channel name"
        bind:value={newChannelName}
        on:keydown={(e) => e.key === "Enter" && handleCreate()}
      />
      <button on:click={handleCreate}>Create</button>
    </div>
  {/if}

  <div class="channel-items">
    {#each $channels as channel}
      <button
        class="channel-item"
        class:active={$currentChannel?.id === channel.id}
        on:click={() => handleClick(channel)}
      >
        <span class="channel-hash">#</span>
        <span class="channel-name">{channel.name}</span>
      </button>
    {:else}
      <p class="empty">No channels yet</p>
    {/each}
  </div>

  <footer class="user-panel">
    <div class="user-avatar">
      {$user?.username?.charAt(0)?.toUpperCase() || "U"}
    </div>
    <div class="user-info">
      <span class="username">{$user?.username || "User"}</span>
      <span class="status">Online</span>
    </div>
    <button class="logout-btn" on:click={handleLogout}>Logout</button>
  </footer>
</aside>

<style>
  .channel-list {
    width: 240px;
    background-color: #2b2d31;
    display: flex;
    flex-direction: column;
  }

  .channel-header {
    padding: 12px 16px;
    border-bottom: 1px solid #1e1f22;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .channel-header h2 {
    font-size: 16px;
    font-weight: 600;
    color: #f2f3f5;
  }

  .add-btn {
    background: none;
    border: none;
    color: #949ba4;
    font-size: 20px;
    cursor: pointer;
  }

  .add-btn:hover {
    color: #dbdee1;
  }

  .create-form {
    padding: 8px;
    display: flex;
    gap: 4px;
  }

  .create-form input {
    flex: 1;
    padding: 6px 8px;
    border-radius: 4px;
    border: none;
    background: #1e1f22;
    color: #dbdee1;
    font-size: 14px;
  }

  .create-form button {
    padding: 6px 12px;
    border-radius: 4px;
    border: none;
    background: #5865f2;
    color: white;
    cursor: pointer;
    font-size: 14px;
  }

  .channel-items {
    flex: 1;
    padding: 8px;
    overflow-y: auto;
  }

  .channel-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    border-radius: 4px;
    background: none;
    border: none;
    color: #949ba4;
    cursor: pointer;
    width: 100%;
    text-align: left;
    transition: all 0.1s ease;
  }

  .channel-item:hover {
    background-color: #35373c;
    color: #dbdee1;
  }

  .channel-item.active {
    background-color: #404249;
    color: #f2f3f5;
  }

  .channel-hash {
    font-size: 20px;
    font-weight: 500;
  }

  .channel-name {
    font-size: 16px;
  }

  .empty {
    color: #949ba4;
    font-size: 14px;
    padding: 8px 12px;
  }

  .user-panel {
    padding: 12px;
    background-color: #232428;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .user-avatar {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background-color: #5865f2;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 14px;
  }

  .user-info {
    display: flex;
    flex-direction: column;
    flex: 1;
  }

  .username {
    font-size: 14px;
    font-weight: 600;
    color: #f2f3f5;
  }

  .status {
    font-size: 12px;
    color: #23a55a;
  }

  .logout-btn {
    background: none;
    border: none;
    color: #949ba4;
    cursor: pointer;
    font-size: 12px;
  }

  .logout-btn:hover {
    color: #faa61a;
  }
</style>
