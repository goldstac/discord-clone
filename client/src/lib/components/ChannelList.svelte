<script>
  import { channels, currentChannel, selectChannel } from "../stores/chatStore.js";
</script>

<aside class="channel-list">
  <header class="channel-header">
    <h2>Channels</h2>
  </header>

  <div class="channel-items">
    {#each $channels as channel}
      <button
        class="channel-item"
        class:active={$currentChannel === channel}
        on:click={() => selectChannel(channel)}
      >
        <span class="channel-hash">#</span>
        <span class="channel-name">{channel}</span>
      </button>
    {/each}
  </div>

  <footer class="user-panel">
    <div class="user-avatar">U</div>
    <div class="user-info">
      <span class="username">User</span>
      <span class="status">Online</span>
    </div>
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
  }

  .channel-header h2 {
    font-size: 16px;
    font-weight: 600;
    color: #f2f3f5;
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
</style>
