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
  let hoveredServer = null;

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
  <div class="sidebar-inner">
    <div class="top-section">
      <div class="home-wrapper">
        <div class="pill" class:active={$currentServerId === null}></div>
        <button
          class="home-icon"
          class:active={$currentServerId === null}
          on:click={() => currentServerId.set(null)}
          title="Direct Messages"
        >
          <svg width="28" height="20" viewBox="0 0 28 20">
            <path fill="currentColor" d="M23.0212 1.67671C21.3107 0.879656 19.5079 0.318797 17.6584 0C17.4062 0.461742 17.1749 0.934541 16.9708 1.4184C15.003 1.12145 12.9974 1.12145 11.0292 1.4184C10.8251 0.934541 10.5765 0.461742 10.3415 0C8.49099 0.318797 6.68722 0.879656 4.97673 1.67671C0.715903 7.74793 -0.437921 13.6549 0.138602 19.4768C2.10107 20.9312 4.33262 21.9839 6.69837 22.5784C7.22304 21.8618 7.69563 21.0988 8.11036 20.2994C7.33883 20.0101 6.59279 19.6562 5.88006 19.2426C6.07154 19.1043 6.25824 18.9615 6.43896 18.8144C11.7309 21.2937 17.4127 21.2937 22.6503 18.8144C22.8348 18.9642 23.0212 19.1069 23.21 19.2426C22.4945 19.659 21.7457 20.0101 20.9742 20.2994C21.3916 21.0988 21.8642 21.8618 22.3889 22.5784C24.7573 21.9839 26.9889 20.9312 28.9513 19.4768C29.6238 12.6536 28.0477 6.80412 23.0212 1.67671ZM9.68041 15.9376C8.33755 15.9376 7.23607 14.7094 7.23607 13.2123C7.23607 11.7152 8.31355 10.487 9.68041 10.487C11.0445 10.487 12.146 11.7152 12.1248 13.2123C12.1035 14.7094 11.0418 15.9376 9.68041 15.9376ZM18.3161 15.9376C16.9732 15.9376 15.8718 14.7094 15.8718 13.2123C15.8718 11.7152 16.9502 10.487 18.3161 10.487C19.6803 10.487 20.7818 11.7152 20.7606 13.2123C20.7393 14.7094 19.6803 15.9376 18.3161 15.9376Z"/>
          </svg>
        </button>
      </div>

      <div class="separator"></div>

      {#each $servers as server}
        <div
          class="server-wrapper"
          on:mouseenter={() => (hoveredServer = server.id)}
          on:mouseleave={() => (hoveredServer = null)}
        >
          <div
            class="pill"
            class:active={$currentServerId === server.id}
            class:hovered={hoveredServer === server.id && $currentServerId !== server.id}
          ></div>
          <button
            class="server-icon"
            class:active={$currentServerId === server.id}
            on:click={() => handleClick(server)}
            title={server.name}
          >
            {server.name.charAt(0).toUpperCase()}
          </button>
          {#if hoveredServer === server.id}
            <div class="tooltip">{server.name}</div>
          {/if}
        </div>
      {/each}

      <div class="server-wrapper">
        <div class="pill" class:hovered={false}></div>
        <button
          class="server-icon add"
          class:open={showCreate}
          on:click={() => (showCreate = !showCreate)}
          title="Add a Server"
        >
          <svg width="24" height="24" viewBox="0 0 24 24">
            <path fill="currentColor" d="M20 11.1111H12.8889V4H11.1111V11.1111H4V12.8889H11.1111V20H12.8889V12.8889H20V11.1111Z"/>
          </svg>
        </button>
      </div>

      {#if showCreate}
        <div class="create-form">
          <div class="create-header">Create a Server</div>
          <input
            type="text"
            placeholder="Server name"
            bind:value={newServerName}
            on:keydown={(e) => e.key === "Enter" && handleCreate()}
          />
          <div class="create-actions">
            <button class="cancel" on:click={() => (showCreate = false)}>Cancel</button>
            <button class="submit" on:click={handleCreate} disabled={!newServerName.trim()}>Create</button>
          </div>
        </div>
      {/if}
    </div>

    <div class="bottom-section">
      <div class="user-wrapper">
        <button class="user-icon" title={$user?.username || "User"}>
          {$user?.username?.charAt(0)?.toUpperCase() || "U"}
        </button>
      </div>
    </div>
  </div>
</aside>

<style>
  .server-sidebar {
    width: 72px;
    background-color: var(--bg-tertiary);
    display: flex;
    flex-direction: column;
    user-select: none;
  }

  .sidebar-inner {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow-y: auto;
    overflow-x: hidden;
    scrollbar-width: none;
  }

  .sidebar-inner::-webkit-scrollbar {
    display: none;
  }

  .top-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-top: 12px;
    gap: 8px;
    flex: 1;
  }

  .bottom-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-bottom: 12px;
    gap: 8px;
  }

  .home-wrapper, .server-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 72px;
    height: 48px;
  }

  .pill {
    position: absolute;
    left: 0;
    width: 4px;
    background-color: var(--header-primary);
    border-radius: 0 4px 4px 0;
    transition: height 0.2s ease, opacity 0.2s ease;
    height: 0;
    opacity: 0;
  }

  .pill.active {
    height: 40px;
    opacity: 1;
  }

  .pill.hovered {
    height: 20px;
    opacity: 1;
  }

  .home-icon {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    background-color: var(--bg-primary);
    color: var(--green);
    border: none;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
  }

  .home-icon:hover, .home-icon.active {
    border-radius: 16px;
    background-color: var(--brand);
    color: white;
  }

  .separator {
    width: 32px;
    height: 2px;
    background-color: var(--bg-modifier-active);
    border-radius: 1px;
    margin: 0 0 4px 0;
  }

  .server-icon {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    background-color: var(--bg-primary);
    color: var(--text-normal);
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
    background-color: var(--brand);
    color: white;
  }

  .server-icon.active {
    border-radius: 16px;
    background-color: var(--brand);
    color: white;
  }

  .server-icon.add {
    background-color: var(--bg-primary);
    color: var(--green);
  }

  .server-icon.add:hover {
    background-color: var(--green);
    color: white;
    border-radius: 16px;
  }

  .server-icon.add.open {
    background-color: var(--green);
    color: white;
    border-radius: 16px;
  }

  .tooltip {
    position: absolute;
    left: 76px;
    background-color: var(--bg-tertiary);
    color: var(--header-primary);
    padding: 8px 12px;
    border-radius: 6px;
    font-size: 14px;
    font-weight: 600;
    white-space: nowrap;
    z-index: 100;
    box-shadow: 0 8px 16px rgba(0,0,0,0.24);
    pointer-events: none;
  }

  .tooltip::before {
    content: "";
    position: absolute;
    left: -4px;
    top: 50%;
    transform: translateY(-50%) rotate(45deg);
    width: 8px;
    height: 8px;
    background-color: var(--bg-tertiary);
  }

  .create-form {
    position: absolute;
    left: 76px;
    top: 0;
    background: var(--bg-secondary);
    border-radius: 8px;
    padding: 16px;
    width: 280px;
    z-index: 100;
    box-shadow: 0 8px 16px rgba(0,0,0,0.24);
  }

  .create-header {
    color: var(--header-primary);
    font-size: 20px;
    font-weight: 700;
    margin-bottom: 12px;
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

  .user-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 72px;
    height: 48px;
  }

  .user-icon {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    background-color: var(--brand);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 16px;
    color: white;
    border: none;
    cursor: pointer;
  }

  .user-icon:hover {
    opacity: 0.8;
  }
</style>
