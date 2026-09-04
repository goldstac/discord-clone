<script>
  export let messages = [];
  export let currentChannel = "";

  let newMessage = "";
</script>

<main class="chat-area">
  <header class="chat-header">
    <span class="channel-hash">#</span>
    <h2>{currentChannel}</h2>
  </header>

  <div class="messages">
    {#each messages as message}
      <div class="message">
        <div class="message-avatar">M</div>
        <div class="message-content">
          <div class="message-header">
            <span class="message-user">{message.user}</span>
            <span class="message-time">{message.time}</span>
          </div>
          <p class="message-text">{message.content}</p>
        </div>
      </div>
    {/each}
  </div>

  <div class="message-input">
    <input
      type="text"
      placeholder="Message #channel"
      bind:value={newMessage}
    />
  </div>
</main>

<style>
  .chat-area {
    flex: 1;
    display: flex;
    flex-direction: column;
    background-color: #313338;
  }

  .chat-header {
    padding: 12px 16px;
    border-bottom: 1px solid #1e1f22;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .chat-header h2 {
    font-size: 16px;
    font-weight: 600;
    color: #f2f3f5;
  }

  .channel-hash {
    font-size: 20px;
    color: #949ba4;
  }

  .messages {
    flex: 1;
    padding: 16px;
    overflow-y: auto;
  }

  .message {
    display: flex;
    gap: 16px;
    padding: 8px 0;
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
  }

  .message-user {
    font-weight: 600;
    color: #f2f3f5;
  }

  .message-time {
    font-size: 12px;
    color: #949ba4;
  }

  .message-text {
    color: #dbdee1;
    line-height: 1.4;
  }

  .message-input {
    padding: 0 16px 16px;
  }

  .message-input input {
    width: 100%;
    padding: 12px 16px;
    border-radius: 8px;
    border: none;
    background-color: #383a40;
    color: #dbdee1;
    font-size: 16px;
    outline: none;
  }

  .message-input input::placeholder {
    color: #6d6f78;
  }
</style>
