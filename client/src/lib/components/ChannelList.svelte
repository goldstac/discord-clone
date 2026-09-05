<script>
  import {
    channels,
    currentChannel,
    selectChannel,
    createNewChannel,
    currentServerId,
    currentServer,
  } from "../stores/chatStore.js";
  import { user, doLogout } from "../stores/authStore.js";

  let showCreate = false;
  let newChannelName = "";
  let categories = [
    { name: "Text Channels", open: true, channels: [] }
  ];

  $: {
    categories[0].channels = $channels;
  }

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

  function toggleCategory(cat) {
    cat.open = !cat.open;
    categories = categories;
  }
</script>

<aside class="channel-list">
  {#if $currentServer}
    <header class="channel-header">
      <span class="server-name">{$currentServer.name}</span>
      <button class="dropdown-icon">
        <svg width="18" height="18" viewBox="0 0 24 24">
          <path fill="currentColor" d="M5.3 9.3a1 1 0 0 1 1.4 0l5.3 5.29 5.3-5.3a1 1 0 1 1 1.4 1.42l-6 6a1 1 0 0 1-1.4 0l-6-6a1 1 0 0 1 0-1.42z"/>
        </svg>
      </button>
    </header>

    <div class="channel-items">
      {#each categories as cat}
        <div class="category">
          <button class="category-header" on:click={() => toggleCategory(cat)}>
            <svg
              class="category-arrow"
              class:closed={!cat.open}
              width="12" height="12" viewBox="0 0 24 24"
            >
              <path fill="currentColor" d="M5.3 9.3a1 1 0 0 1 1.4 0l5.3 5.29 5.3-5.3a1 1 0 1 1 1.4 1.42l-6 6a1 1 0 0 1-1.4 0l-6-6a1 1 0 0 1 0-1.42z"/>
            </svg>
            <span>{cat.name}</span>
          </button>
          {#if cat.open}
            {#each cat.channels as channel}
              <button
                class="channel-item"
                class:active={$currentChannel?.id === channel.id}
                on:click={() => handleClick(channel)}
              >
                <svg class="channel-icon" width="20" height="20" viewBox="0 0 24 24">
                  <path fill="currentColor" d="M5.88657 21C5.57547 21 5.3399 20.7189 5.39427 20.4126L6.00001 17H2.59511C2.28449 17 2.04905 16.7198 2.10259 16.4138L2.27759 15.4138C2.31946 15.1746 2.52722 15 2.77011 15H6.35001L7.41001 9H4.00511C3.69449 9 3.45905 8.71977 3.51259 8.41381L3.68759 7.41381C3.72946 7.17456 3.93722 7 4.18011 7H7.76001L8.39677 3.41262C8.43914 3.17391 8.64664 3 8.88907 3H9.87344C10.1845 3 10.4201 3.28107 10.3657 3.58738L9.76001 7H15.76L16.3968 3.41262C16.4391 3.17391 16.6466 3 16.8891 3H17.8734C18.1845 3 18.4201 3.28107 18.3657 3.58738L17.76 7H21.1649C21.4755 7 21.711 7.28023 21.6574 7.58619L21.4824 8.58619C21.4406 8.82544 21.2328 9 20.9899 9H17.41L16.35 15H19.7549C20.0655 15 20.301 15.2802 20.2474 15.5862L20.0724 16.5862C20.0306 16.8254 19.8228 17 19.5799 17H16L15.3632 20.5874C15.3209 20.8261 15.1134 21 14.8709 21H13.8866C13.5755 21 13.3399 20.7189 13.3943 20.4126L14 17H8.00001L7.36325 20.5874C7.32088 20.8261 7.11337 21 6.87094 21H5.88657ZM9.41045 9L8.35045 15H14.3504L15.4104 9H9.41045Z"/>
                </svg>
                <span class="channel-name">{channel.name}</span>
              </button>
            {/each}
          {/if}
        </div>
      {/each}

      <button class="add-channel-btn" on:click={() => (showCreate = !showCreate)}>
        <svg width="18" height="18" viewBox="0 0 24 24">
          <path fill="currentColor" d="M20 11.1111H12.8889V4H11.1111V11.1111H4V12.8889H11.1111V20H12.8889V12.8889H20V11.1111Z"/>
        </svg>
      </button>

      {#if showCreate}
        <div class="create-form">
          <div class="create-header">Create Channel</div>
          <label class="field-label">Channel Name</label>
          <input
            type="text"
            placeholder="new-channel"
            bind:value={newChannelName}
            on:keydown={(e) => e.key === "Enter" && handleCreate()}
          />
          <div class="create-actions">
            <button class="cancel" on:click={() => (showCreate = false)}>Cancel</button>
            <button class="submit" on:click={handleCreate} disabled={!newChannelName.trim()}>Create Channel</button>
          </div>
        </div>
      {/if}
    </div>
  {:else}
    <div class="no-server">
      <span>Select a server</span>
    </div>
  {/if}

  {#if $user}
    <footer class="user-panel">
      <div class="user-info">
        <div class="avatar-wrapper">
          <div class="avatar">{$user.username?.charAt(0)?.toUpperCase() || "U"}</div>
          <div class="status-dot"></div>
        </div>
        <div class="user-details">
          <span class="username">{$user.username}</span>
          <span class="status">Online</span>
        </div>
      </div>
      <div class="user-controls">
        <button class="control-btn" title="Mute">
          <svg width="20" height="20" viewBox="0 0 24 24">
            <path fill="currentColor" d="M12 2C10.9 2 10 2.9 10 4V12C10 13.1 10.9 14 12 14C13.1 14 14 13.1 14 12V4C14 2.9 13.1 2 12 2Z"/>
            <path fill="currentColor" d="M17 12C17 14.76 14.76 17 12 17C9.24 17 7 14.76 7 12H5C5 15.53 7.61 18.43 11 18.93V22H13V18.93C16.39 18.43 19 15.53 19 12H17Z"/>
          </svg>
        </button>
        <button class="control-btn" title="Deafen">
          <svg width="20" height="20" viewBox="0 0 24 24">
            <path fill="currentColor" d="M12 2C6.48 2 2 6.48 2 12S6.48 22 12 22C17.52 22 22 17.52 22 12S17.52 2 12 2ZM12 20C7.59 20 4 16.41 4 12C4 7.59 7.59 4 12 4C16.41 4 20 7.59 20 12C20 16.41 16.41 20 12 20Z"/>
            <path fill="currentColor" d="M6 14C6 11.79 7.79 10 10 10H14C16.21 10 18 11.79 18 14V15H16V14C16 12.9 15.1 12 14 12H10C8.9 12 8 12.9 8 14V15H6V14Z"/>
          </svg>
        </button>
        <button class="control-btn" title="Settings" on:click={doLogout}>
          <svg width="20" height="20" viewBox="0 0 24 24">
            <path fill="currentColor" d="M19.14 12.94C19.18 12.64 19.2 12.33 19.2 12C19.2 11.68 19.18 11.36 19.13 11.06L21.16 9.48C21.34 9.34 21.39 9.07 21.28 8.87L19.36 5.55C19.24 5.33 18.99 5.26 18.77 5.33L16.38 6.29C15.88 5.91 15.35 5.59 14.76 5.35L14.4 2.81C14.36 2.57 14.16 2.4 13.92 2.4H10.08C9.84 2.4 9.65 2.57 9.61 2.81L9.25 5.35C8.66 5.59 8.12 5.92 7.63 6.29L5.24 5.33C5.02 5.25 4.77 5.33 4.65 5.55L2.74 8.87C2.62 9.08 2.66 9.34 2.86 9.48L4.89 11.06C4.84 11.36 4.8 11.69 4.8 12C4.8 12.31 4.82 12.64 4.87 12.94L2.84 14.52C2.66 14.66 2.61 14.93 2.72 15.13L4.64 18.45C4.76 18.67 5.01 18.74 5.23 18.67L7.62 17.71C8.12 18.09 8.65 18.41 9.24 18.65L9.6 21.19C9.65 21.43 9.84 21.6 10.08 21.6H13.92C14.16 21.6 14.36 21.43 14.39 21.19L14.75 18.65C15.34 18.41 15.88 18.09 16.37 17.71L18.76 18.67C18.98 18.75 19.23 18.67 19.35 18.45L21.27 15.13C21.39 14.91 21.34 14.66 21.15 14.52L19.14 12.94ZM12 15.6C10.02 15.6 8.4 13.98 8.4 12C8.4 10.02 10.02 8.4 12 8.4C13.98 8.4 15.6 10.02 15.6 12C15.6 13.98 13.98 15.6 12 15.6Z"/>
          </svg>
        </button>
      </div>
    </footer>
  {/if}
</aside>

<style>
  .channel-list {
    width: 240px;
    background-color: var(--bg-secondary);
    display: flex;
    flex-direction: column;
  }

  .channel-header {
    height: 48px;
    padding: 0 16px;
    border-bottom: 2px solid var(--bg-tertiary);
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
    box-shadow: 0 1px 0 rgba(4,4,5,0.2), 0 1.5px 0 rgba(6,6,7,0.05), 0 2px 0 rgba(4,4,5,0.05);
  }

  .channel-header:hover {
    background-color: var(--bg-modifier-hover);
  }

  .server-name {
    font-size: 16px;
    font-weight: 600;
    color: var(--header-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .dropdown-icon {
    color: var(--text-normal);
    background: none;
    border: none;
    cursor: pointer;
    display: flex;
    align-items: center;
  }

  .channel-items {
    flex: 1;
    overflow-y: auto;
    padding: 0 8px;
    padding-top: 16px;
  }

  .category {
    margin-bottom: 16px;
  }

  .category-header {
    display: flex;
    align-items: center;
    gap: 2px;
    padding: 0 0 4px 2px;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 12px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.02em;
    width: 100%;
    text-align: left;
  }

  .category-header:hover {
    color: var(--text-normal);
  }

  .category-arrow {
    transition: transform 0.2s ease;
  }

  .category-arrow.closed {
    transform: rotate(-90deg);
  }

  .channel-item {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 8px;
    border-radius: 4px;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    width: 100%;
    text-align: left;
    transition: all 0.1s ease;
    margin-bottom: 2px;
  }

  .channel-item:hover {
    background-color: var(--bg-modifier-hover);
    color: var(--text-normal);
  }

  .channel-item.active {
    background-color: var(--bg-modifier-active);
    color: var(--header-primary);
  }

  .channel-icon {
    flex-shrink: 0;
    opacity: 0.7;
  }

  .channel-item.active .channel-icon {
    opacity: 1;
  }

  .channel-name {
    font-size: 16px;
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .add-channel-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    border-radius: 50%;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    position: absolute;
    right: 12px;
    top: 56px;
  }

  .add-channel-btn:hover {
    color: var(--text-normal);
    background-color: var(--bg-modifier-hover);
  }

  .create-form {
    background: var(--bg-secondary);
    border-radius: 8px;
    padding: 16px;
    margin: 8px;
    box-shadow: 0 8px 16px rgba(0,0,0,0.24);
  }

  .create-header {
    color: var(--header-primary);
    font-size: 20px;
    font-weight: 700;
    margin-bottom: 12px;
  }

  .field-label {
    display: block;
    color: var(--header-secondary);
    font-size: 12px;
    font-weight: 700;
    text-transform: uppercase;
    margin-bottom: 8px;
  }

  .create-form input {
    width: 100%;
    padding: 10px;
    border-radius: 4px;
    border: none;
    background: var(--input-bg);
    color: var(--text-normal);
    font-size: 14px;
    outline: none;
    margin-bottom: 16px;
  }

  .create-form input:focus {
    box-shadow: 0 0 0 2px var(--brand);
  }

  .create-actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  .cancel {
    padding: 8px 16px;
    border-radius: 4px;
    border: none;
    background: transparent;
    color: var(--text-normal);
    cursor: pointer;
    font-size: 14px;
  }

  .cancel:hover {
    text-decoration: underline;
  }

  .submit {
    padding: 8px 16px;
    border-radius: 4px;
    border: none;
    background: var(--brand);
    color: white;
    cursor: pointer;
    font-size: 14px;
    font-weight: 500;
  }

  .submit:hover:not(:disabled) {
    background: var(--brand-hover);
  }

  .submit:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .no-server {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    font-size: 14px;
  }

  .user-panel {
    height: 52px;
    background-color: #232428;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 8px;
  }

  .user-info {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
    min-width: 0;
    padding: 4px;
    border-radius: 4px;
    cursor: pointer;
  }

  .user-info:hover {
    background-color: var(--bg-modifier-hover);
  }

  .avatar-wrapper {
    position: relative;
    flex-shrink: 0;
  }

  .avatar {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background-color: var(--brand);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 14px;
    color: white;
  }

  .status-dot {
    position: absolute;
    bottom: -2px;
    right: -2px;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background-color: var(--green);
    border: 3px solid #232428;
  }

  .user-details {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .username {
    font-size: 14px;
    font-weight: 600;
    color: var(--header-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .status {
    font-size: 12px;
    color: var(--text-muted);
  }

  .user-controls {
    display: flex;
    gap: 2px;
  }

  .control-btn {
    width: 32px;
    height: 32px;
    border-radius: 4px;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .control-btn:hover {
    background-color: var(--bg-modifier-hover);
    color: var(--text-normal);
  }
</style>
