<script>
  import { onMount, afterUpdate } from "svelte";
  import { currentChannel, messages, sendNewMessage } from "../stores/chatStore.js";

  let input = "";
  let chatArea;

  $: if ($messages) {
    scrollToBottom();
  }

  function scrollToBottom() {
    if (chatArea) {
      setTimeout(() => {
        chatArea.scrollTop = chatArea.scrollHeight;
      }, 50);
    }
  }

  async function handleSubmit() {
    if (input.trim() && $currentChannel) {
      await sendNewMessage(input);
      input = "";
    }
  }
</script>

<div class="chat-area">
  {#if $currentChannel}
    <header class="chat-header">
      <span class="channel-hash">#</span>
      <span class="channel-name">{$currentChannel.name}</span>
    </header>

    <div class="messages" bind:this={chatArea}>
      {#each $messages as message (message.id)}
        <div class="message">
          <div class="message-avatar">
            {message.username?.charAt(0)?.toUpperCase() || "U"}
          </div>
          <div class="message-content">
            <div class="message-header">
              <span class="username">{message.username || "Unknown"}</span>
              <span class="timestamp">{message.created_at}</span>
            </div>
            <div class="text">{message.content}</div>
          </div>
        </div>
      {:else}
        <div class="empty-state">
          <div class="empty-icon">#</div>
          <h2>Welcome to #{$currentChannel.name}</h2>
          <p>This is the start of the channel.</p>
        </div>
      {/each}
    </div>

    <div class="input-area">
      <form on:submit|preventDefault={handleSubmit}>
        <input
          type="text"
          placeholder="Message #{$currentChannel.name}"
          bind:value={input}
        />
        <button type="submit" disabled={!input.trim()}>Send</button>
      </form>
    </div>
  {:else}
    <div class="no-channel">
      <h2>Select a channel to start chatting</h2>
    </div>
  {/if}
</div>

<style>
  .chat-area {
    flex: 1;
    display: flex;
    flex-direction: column;
    background-color: #313338;
  }

  .chat-header {
    padding: 16px;
    border-bottom: 1px solid #1e1f22;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .channel-hash {
    color: #80848e;
    font-size: 24px;
  }

  .channel-name {
    font-size: 16px;
    font-weight: 600;
    color: #f2f3f5;
  }

  .messages {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .message {
    display: flex;
    gap: 16px;
  }

  .message-avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background-color: #5865f2;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    flex-shrink: 0;
  }

  .message-content {
    flex: 1;
  }

  .message-header {
    display: flex;
    align-items: baseline;
    gap: 8px;
    margin-bottom: 4px;
  }

  .username {
    font-weight: 600;
    color: #f2f3f5;
  }

  .timestamp {
    font-size: 12px;
    color: #949ba4;
  }

  .text {
    color: #dbdee1;
    line-height: 1.4;
    word-break: break-word;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #949ba4;
  }

  .empty-icon {
    font-size: 48px;
    margin-bottom: 16px;
    color: #5865f2;
  }

  .empty-state h2 {
    color: #f2f3f5;
    margin-bottom: 8px;
  }

  .input-area {
    padding: 16px;
    background-color: #383a40;
  }

  form {
    display: flex;
    gap: 8px;
  }

  input {
    flex: 1;
    padding: 12px 16px;
    border-radius: 8px;
    border: none;
    background-color: #1e1f22;
    color: #dbdee1;
    font-size: 16px;
    outline: none;
  }

  input::placeholder {
    color: #6d6f78;
  }

  button {
    padding: 12px 24px;
    border-radius: 8px;
    border: none;
    background-color: #5865f2;
    color: white;
    font-size: 16px;
    font-weight: 600;
    cursor: pointer;
  }

  button:hover:not(:disabled) {
    background-color: #4752c4;
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .no-channel {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #949ba4;
  }
</style>
