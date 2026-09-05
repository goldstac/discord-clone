import { writable, derived, get } from "svelte/store";
import * as api from "../api.js";

export const servers = writable([]);
export const currentServerId = writable(null);
export const currentChannel = writable(null);
export const channels = writable([]);
export const messages = writable([]);
export const wsMessages = writable([]);

export const currentServer = derived(
  [servers, currentServerId],
  ([$servers, $currentServerId]) =>
    $servers.find((s) => s.id === $currentServerId)
);

export async function loadServers() {
  try {
    const data = await api.getServers();
    servers.set(data);
    if (data.length > 0 && !get(currentServerId)) {
      await selectServer(data[0].id);
    }
  } catch (e) {
    console.error("Failed to load servers:", e);
  }
}

export async function selectServer(id) {
  currentServerId.set(id);
  currentChannel.set(null);
  channels.set([]);
  messages.set([]);

  try {
    const data = await api.getChannels(id);
    channels.set(data);
    if (data.length > 0) {
      await selectChannel(data[0].id, data[0].name);
    }
  } catch (e) {
    console.error("Failed to load channels:", e);
  }
}

export async function selectChannel(id, name) {
  currentChannel.set({ id, name });
  messages.set([]);

  try {
    const data = await api.getMessages(id);
    messages.set(data);
  } catch (e) {
    console.error("Failed to load messages:", e);
  }
}

export async function sendNewMessage(content) {
  const channel = get(currentChannel);
  if (!channel || !content.trim()) return;

  try {
    await api.sendMessage(channel.id, content);
  } catch (e) {
    console.error("Failed to send message:", e);
  }
}

export async function createNewServer(name) {
  try {
    await api.createServer(name);
    await loadServers();
  } catch (e) {
    console.error("Failed to create server:", e);
  }
}

export async function createNewChannel(serverId, name) {
  try {
    await api.createChannel(serverId, name);
    const data = await api.getChannels(serverId);
    channels.set(data);
  } catch (e) {
    console.error("Failed to create channel:", e);
  }
}

export async function joinWithInvite(code) {
  try {
    await api.joinServer(code);
    await loadServers();
  } catch (e) {
    console.error("Failed to join server:", e);
  }
}

export function addWsMessage(msg) {
  const channel = get(currentChannel);
  if (channel && msg.channel_id === channel.id) {
    messages.update((msgs) => [
      ...msgs,
      {
        id: Date.now(),
        user_id: msg.user_id,
        username: msg.username,
        content: msg.content,
        created_at: msg.timestamp,
      },
    ]);
  }
}
