import { writable } from "svelte/store";
import * as api from "../api.js";

export const user = writable(null);
export const isAuthenticated = writable(api.isLoggedIn());

export async function doLogin(username, password) {
  const data = await api.login(username, password);
  user.set(data.user);
  isAuthenticated.set(true);
  return data;
}

export async function doRegister(username, password) {
  const data = await api.register(username, password);
  user.set(data.user);
  isAuthenticated.set(true);
  return data;
}

export function doLogout() {
  api.setToken(null);
  user.set(null);
  isAuthenticated.set(false);
}

export async function loadUser() {
  if (!api.isLoggedIn()) return;
  try {
    const u = await api.getMe();
    user.set(u);
    isAuthenticated.set(true);
  } catch {
    api.setToken(null);
    user.set(null);
    isAuthenticated.set(false);
  }
}
