<script>
  import ServerSidebar from "./lib/components/ServerSidebar.svelte";
  import ChannelList from "./lib/components/ChannelList.svelte";
  import ChatArea from "./lib/components/ChatArea.svelte";

  let currentServer = "My Server";
  let currentChannel = "general";

  const servers = [
    { id: 1, name: "My Server", icon: "MS" },
    { id: 2, name: "Gaming", icon: "GA" },
    { id: 3, name: "Music", icon: "MU" },
  ];

  const channels = [
    { id: 1, name: "general" },
    { id: 2, name: "random" },
    { id: 3, name: "announcements" },
  ];

  const messages = [
    { id: 1, user: "Alice", content: "Hey everyone!", time: "10:30 AM" },
    { id: 2, user: "Bob", content: "What's up?", time: "10:32 AM" },
    { id: 3, user: "Charlie", content: "Welcome to the server!", time: "10:35 AM" },
  ];

  function selectServer(server) {
    currentServer = server.name;
  }

  function selectChannel(channel) {
    currentChannel = channel.name;
  }
</script>

<main class="app">
  <ServerSidebar {servers} {currentServer} on:select={selectServer} />
  <ChannelList {channels} {currentChannel} on:select={selectChannel} />
  <ChatArea {messages} {currentChannel} />
</main>

<style>
  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  :global(body) {
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
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
