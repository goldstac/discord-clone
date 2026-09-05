<script>
  import { createEventDispatcher } from "svelte";
  import { doLogin } from "../stores/authStore.js";

  const dispatch = createEventDispatcher();

  let username = "";
  let password = "";
  let error = "";
  let loading = false;

  async function handleSubmit() {
    error = "";
    loading = true;
    try {
      await doLogin(username, password);
      dispatch("success");
    } catch (e) {
      error = typeof e.message === "string" ? e.message : "Login failed";
    } finally {
      loading = false;
    }
  }
</script>

<div class="auth-container">
  <div class="auth-card">
    <h1>Welcome Back!</h1>
    <p class="subtitle">Log in to continue</p>

    {#if error}
      <div class="error">{error}</div>
    {/if}

    <form on:submit|preventDefault={handleSubmit}>
      <div class="field">
        <label for="username">Username</label>
        <input
          id="username"
          type="text"
          bind:value={username}
          placeholder="Enter username"
          required
        />
      </div>

      <div class="field">
        <label for="password">Password</label>
        <input
          id="password"
          type="password"
          bind:value={password}
          placeholder="Enter password"
          required
        />
      </div>

      <button type="submit" disabled={loading}>
        {loading ? "Logging in..." : "Log In"}
      </button>
    </form>

    <p class="switch">
      Need an account?
      <button class="link" on:click={() => dispatch("switchToRegister")}>
        Register
      </button>
    </p>
  </div>
</div>

<style>
  .auth-container {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
    background-color: #313338;
  }

  .auth-card {
    background-color: #2b2d31;
    border-radius: 8px;
    padding: 32px;
    width: 400px;
    text-align: center;
  }

  h1 {
    color: #f2f3f5;
    margin-bottom: 4px;
  }

  .subtitle {
    color: #949ba4;
    margin-bottom: 24px;
  }

  .error {
    background-color: #58272f;
    color: #faa61a;
    padding: 8px 12px;
    border-radius: 4px;
    margin-bottom: 16px;
    font-size: 14px;
  }

  .field {
    text-align: left;
    margin-bottom: 16px;
  }

  label {
    display: block;
    color: #b5bac1;
    font-size: 12px;
    font-weight: 700;
    text-transform: uppercase;
    margin-bottom: 6px;
  }

  input {
    width: 100%;
    padding: 10px 12px;
    border-radius: 4px;
    border: none;
    background-color: #1e1f22;
    color: #dbdee1;
    font-size: 16px;
    outline: none;
  }

  input:focus {
    outline: 2px solid #5865f2;
  }

  button[type="submit"] {
    width: 100%;
    padding: 12px;
    border-radius: 4px;
    border: none;
    background-color: #5865f2;
    color: white;
    font-size: 16px;
    font-weight: 600;
    cursor: pointer;
    margin-top: 8px;
  }

  button[type="submit"]:hover {
    background-color: #4752c4;
  }

  button[type="submit"]:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .switch {
    color: #949ba4;
    font-size: 14px;
    margin-top: 16px;
  }

  .link {
    background: none;
    border: none;
    color: #00a8fc;
    cursor: pointer;
    font-size: 14px;
    padding: 0;
  }

  .link:hover {
    text-decoration: underline;
  }
</style>
