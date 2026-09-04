import { writable, derived } from "svelte/store";

export const servers = writable([
  { id: 1, name: "My Server", icon: "MS", channels: ["general", "random", "announcements"] },
  { id: 2, name: "Gaming", icon: "GA", channels: ["valorant", "minecraft", "chat"] },
  { id: 3, name: "Music", icon: "MU", channels: ["recommendations", "share", "lyrics"] },
]);

export const currentServerId = writable(1);
export const currentChannel = writable("general");

export const currentServer = derived(
  [servers, currentServerId],
  ([$servers, $currentServerId]) => $servers.find((s) => s.id === $currentServerId)
);

export const channels = derived(currentServer, ($currentServer) =>
  $currentServer ? $currentServer.channels : []
);

export const messages = writable({
  general: [
    { id: 1, user: "Alice", content: "Hey everyone!", time: "10:30 AM", avatar: "A" },
    { id: 2, user: "Bob", content: "What's up?", time: "10:32 AM", avatar: "B" },
    { id: 3, user: "Charlie", content: "Welcome to the server!", time: "10:35 AM", avatar: "C" },
  ],
  random: [
    { id: 1, user: "Dave", content: "Anyone playing tonight?", time: "9:00 PM", avatar: "D" },
  ],
  announcements: [
    { id: 1, user: "Admin", content: "Server rules: Be nice!", time: "8:00 AM", avatar: "A" },
  ],
  valorant: [
    { id: 1, user: "Pro", content: "Need 2 more for ranked", time: "11:00 PM", avatar: "P" },
  ],
  minecraft: [],
  chat: [],
  recommendations: [],
  share: [],
  lyrics: [],
});

export function selectServer(id) {
  currentServerId.set(id);
  const unsub = currentServer.subscribe(($server) => {
    if ($server && $server.channels.length > 0) {
      currentChannel.set($server.channels[0]);
    }
  });
  unsub();
}

export function selectChannel(name) {
  currentChannel.set(name);
}

export function sendMessage(content) {
  let channelName;
  currentChannel.subscribe((c) => channelName = c)();

  const now = new Date();
  const time = now.toLocaleTimeString("en-US", { hour: "numeric", minute: "2-digit" });

  messages.update(($msgs) => {
    const channelMsgs = $msgs[channelName] || [];
    return {
      ...$msgs,
      [channelName]: [
        ...channelMsgs,
        {
          id: Date.now(),
          user: "You",
          content,
          time,
          avatar: "Y",
        },
      ],
    };
  });
}
