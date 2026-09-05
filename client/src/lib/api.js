const API_URL = import.meta.env.VITE_API_URL || "http://localhost:3000";
const WS_URL = import.meta.env.VITE_WS_URL || "ws://localhost:3000";

let authToken = localStorage.getItem("token") || null;

function getHeaders() {
  const headers = { "Content-Type": "application/json" };
  if (authToken) {
    headers["Authorization"] = `Bearer ${authToken}`;
  }
  return headers;
}

export function setToken(token) {
  authToken = token;
  if (token) {
    localStorage.setItem("token", token);
  } else {
    localStorage.removeItem("token");
  }
}

export function getToken() {
  return authToken;
}

export function isLoggedIn() {
  return !!authToken;
}

export async function register(username, password) {
  const res = await fetch(`${API_URL}/api/auth/register`, {
    method: "POST",
    headers: getHeaders(),
    body: JSON.stringify({ username, password }),
  });
  const data = await res.json();
  if (!res.ok) throw new Error(data.error || "Request failed");
  setToken(data.token);
  return data;
}

export async function login(username, password) {
  const res = await fetch(`${API_URL}/api/auth/login`, {
    method: "POST",
    headers: getHeaders(),
    body: JSON.stringify({ username, password }),
  });
  const data = await res.json();
  if (!res.ok) throw new Error(data.error || "Request failed");
  setToken(data.token);
  return data;
}

export async function getMe() {
  const res = await fetch(`${API_URL}/api/users/me`, { headers: getHeaders() });
  const data = await res.json();
  if (!res.ok) throw new Error(data.error || "Request failed");
  return data;
}

export async function getServers() {
  const res = await fetch(`${API_URL}/api/servers`, { headers: getHeaders() });
  const data = await res.json();
  if (!res.ok) throw new Error(data.error || "Request failed");
  return data;
}

export async function createServer(name) {
  const res = await fetch(`${API_URL}/api/servers`, {
    method: "POST",
    headers: getHeaders(),
    body: JSON.stringify({ name }),
  });
  const data = await res.json();
  if (!res.ok) throw new Error(data.error || "Request failed");
  return data;
}

export async function joinServer(code) {
  const res = await fetch(`${API_URL}/api/servers/1/join`, {
    method: "POST",
    headers: getHeaders(),
    body: JSON.stringify({ code }),
  });
  const data = await res.json();
  if (!res.ok) throw new Error(data.error || "Request failed");
  return data;
}

export async function getChannels(serverId) {
  const res = await fetch(`${API_URL}/api/servers/${serverId}/channels`, {
    headers: getHeaders(),
  });
  const data = await res.json();
  if (!res.ok) throw new Error(data.error || "Request failed");
  return data;
}

export async function createChannel(serverId, name) {
  const res = await fetch(`${API_URL}/api/servers/${serverId}/channels`, {
    method: "POST",
    headers: getHeaders(),
    body: JSON.stringify({ name }),
  });
  const data = await res.json();
  if (!res.ok) throw new Error(data.error || "Request failed");
  return data;
}

export async function getMessages(channelId) {
  const res = await fetch(`${API_URL}/api/channels/${channelId}/messages`, {
    headers: getHeaders(),
  });
  const data = await res.json();
  if (!res.ok) throw new Error(data.error || "Request failed");
  return data;
}

export async function sendMessage(channelId, content) {
  const res = await fetch(`${API_URL}/api/channels/${channelId}/messages`, {
    method: "POST",
    headers: getHeaders(),
    body: JSON.stringify({ content }),
  });
  const data = await res.json();
  if (!res.ok) throw new Error(data.error || "Request failed");
  return data;
}

export async function createInvite(serverId) {
  const res = await fetch(`${API_URL}/api/servers/${serverId}/invites`, {
    method: "POST",
    headers: getHeaders(),
  });
  const data = await res.json();
  if (!res.ok) throw new Error(data.error || "Request failed");
  return data;
}

export function connectWebSocket(onMessage, onOpen) {
  if (!authToken) return null;

  const ws = new WebSocket(`${WS_URL}/ws?token=${authToken}`);

  ws.onopen = () => {
    if (onOpen) onOpen();
  };

  ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    if (onMessage) onMessage(data);
  };

  ws.onclose = () => {
    setTimeout(() => {
      if (authToken) connectWebSocket(onMessage, onOpen);
    }, 3000);
  };

  return ws;
}
