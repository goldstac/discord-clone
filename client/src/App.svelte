<script>
  import ServerSidebar from "./lib/components/ServerSidebar.svelte";
  import ChannelList from "./lib/components/ChannelList.svelte";
  import ChatArea from "./lib/components/ChatArea.svelte";
</script>

<main class="app">
  <ServerSidebar />
  <ChannelList />
  <ChatArea />
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
