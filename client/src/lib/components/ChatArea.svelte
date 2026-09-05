<script>
  import { onMount, afterUpdate, tick } from "svelte";
  import { currentChannel, messages, sendNewMessage } from "../stores/chatStore.js";
  import { user } from "../stores/authStore.js";

  let input = "";
  let chatArea;
  let showScrollBtn = false;

  $: if ($messages) {
    scrollToBottom();
  }

  function scrollToBottom() {
    tick().then(() => {
      if (chatArea) {
        chatArea.scrollTop = chatArea.scrollHeight;
      }
    });
  }

  function handleScroll() {
    if (!chatArea) return;
    const { scrollTop, scrollHeight, clientHeight } = chatArea;
    showScrollBtn = scrollHeight - scrollTop - clientHeight > 100;
  }

  function jumpToBottom() {
    scrollToBottom();
  }

  async function handleSubmit() {
    if (input.trim() && $currentChannel) {
      await sendNewMessage(input);
      input = "";
    }
  }

  function shouldShowHeader(msg, index) {
    if (index === 0) return true;
    const prev = $messages[index - 1];
    if (prev.user_id !== msg.user_id) return true;
    const prevTime = new Date(prev.created_at).getTime();
    const msgTime = new Date(msg.created_at).getTime();
    return msgTime - prevTime > 7 * 60 * 1000;
  }

  function formatTimestamp(ts) {
    const date = new Date(ts);
    const now = new Date();
    const isToday = date.toDateString() === now.toDateString();
    const hours = date.getHours();
    const minutes = date.getMinutes().toString().padStart(2, "0");
    const ampm = hours >= 12 ? "PM" : "AM";
    const h12 = hours % 12 || 12;
    const time = `${h12}:${minutes} ${ampm}`;
    if (isToday) return `Today at ${time}`;
    return `${date.toLocaleDateString()} ${time}`;
  }

  function getInitial(name) {
    return name?.charAt(0)?.toUpperCase() || "U";
  }
</script>

<div class="chat-area">
  {#if $currentChannel}
    <header class="chat-header">
      <div class="header-left">
        <svg class="hash-icon" width="24" height="24" viewBox="0 0 24 24">
          <path fill="currentColor" d="M5.88657 21C5.57547 21 5.3399 20.7189 5.39427 20.4126L6.00001 17H2.59511C2.28449 17 2.04905 16.7198 2.10259 16.4138L2.27759 15.4138C2.31946 15.1746 2.52722 15 2.77011 15H6.35001L7.41001 9H4.00511C3.69449 9 3.45905 8.71977 3.51259 8.41381L3.68759 7.41381C3.72946 7.17456 3.93722 7 4.18011 7H7.76001L8.39677 3.41262C8.43914 3.17391 8.64664 3 8.88907 3H9.87344C10.1845 3 10.4201 3.28107 10.3657 3.58738L9.76001 7H15.76L16.3968 3.41262C16.4391 3.17391 16.6466 3 16.8891 3H17.8734C18.1845 3 18.4201 3.28107 18.3657 3.58738L17.76 7H21.1649C21.4755 7 21.711 7.28023 21.6574 7.58619L21.4824 8.58619C21.4406 8.82544 21.2328 9 20.9899 9H17.41L16.35 15H19.7549C20.0655 15 20.301 15.2802 20.2474 15.5862L20.0724 16.5862C20.0306 16.8254 19.8228 17 19.5799 17H16L15.3632 20.5874C15.3209 20.8261 15.1134 21 14.8709 21H13.8866C13.5755 21 13.3399 20.7189 13.3943 20.4126L14 17H8.00001L7.36325 20.5874C7.32088 20.8261 7.11337 21 6.87094 21H5.88657ZM9.41045 9L8.35045 15H14.3504L15.4104 9H9.41045Z"/>
        </svg>
        <span class="channel-name">{$currentChannel.name}</span>
      </div>
      <div class="header-right">
        <button class="header-btn" title="Pinned Messages">
          <svg width="24" height="24" viewBox="0 0 24 24">
            <path fill="currentColor" d="M22 12L12.101 2.10101L10.686 3.51401L12.101 4.92901L7.15096 9.87801V9.87901L5.73596 8.46401L4.32196 9.87801L8.56496 14.121L2.90796 19.778L4.32196 21.192L9.97896 15.536L14.222 19.778L15.636 18.364L14.222 16.95L19.171 12H19.172L20.586 13.414L22 12Z"/>
          </svg>
        </button>
        <button class="header-btn" title="Member List">
          <svg width="24" height="24" viewBox="0 0 24 24">
            <path fill="currentColor" d="M14 8.00598C14 10.211 12.206 12.006 10 12.006C7.795 12.006 6 10.211 6 8.00598C6 5.80098 7.794 4.00598 10 4.00598C12.206 4.00598 14 5.80098 14 8.00598ZM2 19.006C2 15.473 5.29 13.006 10 13.006C14.711 13.006 18 15.473 18 19.006V20.006H2V19.006ZM20 20.006H22V19.006C22 16.4469 20.2663 14.4633 17.5 13.453C19.0113 14.8753 20 16.8753 20 19.006V20.006ZM14 8.00598C14 9.44098 13.4723 10.7463 12.6143 11.7623C14.3983 11.2963 15.7363 9.68498 15.8663 7.74598C15.9263 6.82698 15.6433 5.94598 15.1043 5.23398C14.6153 4.58498 13.9433 4.08998 13.1733 3.81098C13.4373 4.71898 13.6083 5.66098 13.6763 6.62198L13.6903 6.80998C13.7283 7.33198 13.7483 7.85898 13.7483 8.39098V8.00598H14Z"/>
          </svg>
        </button>
        <div class="search-bar">
          <input type="text" placeholder="Search" />
        </div>
      </div>
    </header>

    <div class="messages" bind:this={chatArea} on:scroll={handleScroll}>
      <div class="welcome">
        <div class="welcome-icon">
          <svg width="68" height="68" viewBox="0 0 24 24">
            <path fill="currentColor" d="M5.88657 21C5.57547 21 5.3399 20.7189 5.39427 20.4126L6.00001 17H2.59511C2.28449 17 2.04905 16.7198 2.10259 16.4138L2.27759 15.4138C2.31946 15.1746 2.52722 15 2.77011 15H6.35001L7.41001 9H4.00511C3.69449 9 3.45905 8.71977 3.51259 8.41381L3.68759 7.41381C3.72946 7.17456 3.93722 7 4.18011 7H7.76001L8.39677 3.41262C8.43914 3.17391 8.64664 3 8.88907 3H9.87344C10.1845 3 10.4201 3.28107 10.3657 3.58738L9.76001 7H15.76L16.3968 3.41262C16.4391 3.17391 16.6466 3 16.8891 3H17.8734C18.1845 3 18.4201 3.28107 18.3657 3.58738L17.76 7H21.1649C21.4755 7 21.711 7.28023 21.6574 7.58619L21.4824 8.58619C21.4406 8.82544 21.2328 9 20.9899 9H17.41L16.35 15H19.7549C20.0655 15 20.301 15.2802 20.2474 15.5862L20.0724 16.5862C20.0306 16.8254 19.8228 17 19.5799 17H16L15.3632 20.5874C15.3209 20.8261 15.1134 21 14.8709 21H13.8866C13.5755 21 13.3399 20.7189 13.3943 20.4126L14 17H8.00001L7.36325 20.5874C7.32088 20.8261 7.11337 21 6.87094 21H5.88657ZM9.41045 9L8.35045 15H14.3504L15.4104 9H9.41045Z"/>
          </svg>
        </div>
        <h1 class="welcome-title">Welcome to #{$currentChannel.name}!</h1>
        <p class="welcome-desc">This is the start of the #{$currentChannel.name} channel.</p>
      </div>

      {#each $messages as message, i (message.id)}
        {#if shouldShowHeader(message, i)}
          <div class="message-group">
            <div class="message-avatar">
              {getInitial(message.username)}
            </div>
            <div class="message-content">
              <div class="message-header">
                <span class="username">{message.username || "Unknown"}</span>
                <span class="timestamp" title={new Date(message.created_at).toLocaleString()}>
                  {formatTimestamp(message.created_at)}
                </span>
              </div>
              <div class="message-text">{message.content}</div>
            </div>
          </div>
        {:else}
          <div class="message-continued">
            <span class="hover-timestamp">{new Date(message.created_at).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}</span>
            <div class="message-text">{message.content}</div>
          </div>
        {/if}
      {/each}
    </div>

    {#if showScrollBtn}
      <button class="scroll-btn" on:click={jumpToBottom}>
        <svg width="24" height="24" viewBox="0 0 24 24">
          <path fill="currentColor" d="M16.59 8.59L12 13.17 7.41 8.59 6 10l6 6 6-6z"/>
        </svg>
      </button>
    {/if}

    <div class="input-area">
      <form on:submit|preventDefault={handleSubmit}>
        <button type="button" class="attach-btn" title="Attach file">
          <svg width="24" height="24" viewBox="0 0 24 24">
            <path fill="currentColor" d="M12 2C6.48 2 2 6.48 2 12C2 17.52 6.48 22 12 22C17.52 22 22 17.52 22 12C22 6.48 17.52 2 12 2ZM17 13H13V17H11V13H7V11H11V7H13V11H17V13Z"/>
          </svg>
        </button>
        <input
          type="text"
          placeholder="Message #{$currentChannel.name}"
          bind:value={input}
        />
        <button type="submit" class="send-btn" disabled={!input.trim()} title="Send">
          <svg width="24" height="24" viewBox="0 0 24 24">
            <path fill="currentColor" d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
          </svg>
        </button>
      </form>
    </div>
  {:else}
    <div class="no-channel">
      <div class="no-channel-inner">
        <svg width="120" height="120" viewBox="0 0 24 24" class="no-channel-icon">
          <path fill="currentColor" d="M5.88657 21C5.57547 21 5.3399 20.7189 5.39427 20.4126L6.00001 17H2.59511C2.28449 17 2.04905 16.7198 2.10259 16.4138L2.27759 15.4138C2.31946 15.1746 2.52722 15 2.77011 15H6.35001L7.41001 9H4.00511C3.69449 9 3.45905 8.71977 3.51259 8.41381L3.68759 7.41381C3.72946 7.17456 3.93722 7 4.18011 7H7.76001L8.39677 3.41262C8.43914 3.17391 8.64664 3 8.88907 3H9.87344C10.1845 3 10.4201 3.28107 10.3657 3.58738L9.76001 7H15.76L16.3968 3.41262C16.4391 3.17391 16.6466 3 16.8891 3H17.8734C18.1845 3 18.4201 3.28107 18.3657 3.58738L17.76 7H21.1649C21.4755 7 21.711 7.28023 21.6574 7.58619L21.4824 8.58619C21.4406 8.82544 21.2328 9 20.9899 9H17.41L16.35 15H19.7549C20.0655 15 20.301 15.2802 20.2474 15.5862L20.0724 16.5862C20.0306 16.8254 19.8228 17 19.5799 17H16L15.3632 20.5874C15.3209 20.8261 15.1134 21 14.8709 21H13.8866C13.5755 21 13.3399 20.7189 13.3943 20.4126L14 17H8.00001L7.36325 20.5874C7.32088 20.8261 7.11337 21 6.87094 21H5.88657ZM9.41045 9L8.35045 15H14.3504L15.4104 9H9.41045Z"/>
        </svg>
        <h2>Welcome!</h2>
        <p>Select a channel to start chatting.</p>
      </div>
    </div>
  {/if}
</div>

<style>
  .chat-area {
    flex: 1;
    display: flex;
    flex-direction: column;
    background-color: var(--bg-primary);
    min-width: 0;
  }

  .chat-header {
    height: 48px;
    padding: 0 16px;
    border-bottom: 2px solid var(--bg-tertiary);
    display: flex;
    align-items: center;
    justify-content: space-between;
    box-shadow: 0 1px 0 rgba(4,4,5,0.2), 0 1.5px 0 rgba(6,6,7,0.05), 0 2px 0 rgba(4,4,5,0.05);
    flex-shrink: 0;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .hash-icon {
    color: var(--text-muted);
  }

  .channel-name {
    font-size: 16px;
    font-weight: 600;
    color: var(--header-primary);
  }

  .header-right {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .header-btn {
    width: 28px;
    height: 28px;
    border-radius: 4px;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .header-btn:hover {
    color: var(--text-normal);
  }

  .search-bar {
    margin-left: 8px;
  }

  .search-bar input {
    width: 140px;
    height: 28px;
    padding: 0 8px;
    border-radius: 4px;
    border: none;
    background: var(--bg-tertiary);
    color: var(--text-normal);
    font-size: 14px;
    outline: none;
  }

  .search-bar input::placeholder {
    color: var(--text-muted);
  }

  .messages {
    flex: 1;
    overflow-y: auto;
    padding: 16px 16px 0 16px;
    display: flex;
    flex-direction: column;
  }

  .welcome {
    padding: 16px 0 32px 0;
    border-bottom: 1px solid var(--bg-modifier-active);
    margin-bottom: 16px;
  }

  .welcome-icon {
    width: 68px;
    height: 68px;
    border-radius: 50%;
    background-color: var(--bg-modifier-active);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--header-primary);
    margin-bottom: 8px;
  }

  .welcome-title {
    font-size: 32px;
    font-weight: 700;
    color: var(--header-primary);
    margin-bottom: 8px;
  }

  .welcome-desc {
    color: var(--text-muted);
    font-size: 14px;
  }

  .message-group {
    display: flex;
    gap: 16px;
    padding: 2px 0;
    margin-top: 17px;
    position: relative;
  }

  .message-group:hover {
    background-color: var(--bg-modifier-hover);
  }

  .message-avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background-color: var(--brand);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 16px;
    color: white;
    flex-shrink: 0;
    cursor: pointer;
    margin-top: 2px;
  }

  .message-content {
    flex: 1;
    min-width: 0;
  }

  .message-header {
    display: flex;
    align-items: baseline;
    gap: 8px;
    margin-bottom: 2px;
  }

  .username {
    font-weight: 600;
    color: var(--header-primary);
    cursor: pointer;
    font-size: 16px;
  }

  .username:hover {
    text-decoration: underline;
  }

  .timestamp {
    font-size: 12px;
    color: var(--text-muted);
    font-weight: 400;
  }

  .message-continued {
    display: flex;
    gap: 16px;
    padding: 2px 0;
    padding-left: 56px;
    position: relative;
  }

  .message-continued:hover {
    background-color: var(--bg-modifier-hover);
  }

  .hover-timestamp {
    position: absolute;
    left: 0;
    width: 56px;
    text-align: right;
    font-size: 11px;
    color: var(--text-muted);
    opacity: 0;
    user-select: none;
    padding-top: 4px;
  }

  .message-continued:hover .hover-timestamp {
    opacity: 1;
  }

  .message-text {
    color: var(--text-normal);
    line-height: 1.375;
    word-break: break-word;
  }

  .scroll-btn {
    position: absolute;
    bottom: 80px;
    right: 24px;
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: var(--bg-tertiary);
    border: none;
    color: var(--text-normal);
    cursor: pointer;
    box-shadow: 0 4px 8px rgba(0,0,0,0.24);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 10;
  }

  .scroll-btn:hover {
    background: var(--bg-modifier-active);
  }

  .input-area {
    padding: 0 16px 24px 16px;
    flex-shrink: 0;
  }

  form {
    display: flex;
    align-items: center;
    background-color: var(--bg-accent);
    border-radius: 8px;
    padding: 0 4px;
  }

  .attach-btn {
    width: 40px;
    height: 44px;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .attach-btn:hover {
    color: var(--text-normal);
  }

  input {
    flex: 1;
    padding: 11px 0;
    border: none;
    background: transparent;
    color: var(--text-normal);
    font-size: 16px;
    outline: none;
  }

  input::placeholder {
    color: var(--text-muted);
  }

  .send-btn {
    width: 40px;
    height: 44px;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    border-radius: 4px;
  }

  .send-btn:hover:not(:disabled) {
    color: var(--text-normal);
  }

  .send-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .no-channel {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .no-channel-inner {
    text-align: center;
    color: var(--text-muted);
  }

  .no-channel-icon {
    margin-bottom: 16px;
    opacity: 0.3;
  }

  .no-channel-inner h2 {
    color: var(--header-primary);
    margin-bottom: 8px;
  }
</style>
