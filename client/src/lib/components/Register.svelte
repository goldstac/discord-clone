<script>
  import { createEventDispatcher } from "svelte";
  import { doRegister } from "../stores/authStore.js";

  const dispatch = createEventDispatcher();

  let username = "";
  let password = "";
  let confirmPassword = "";
  let error = "";
  let loading = false;

  async function handleSubmit() {
    error = "";

    if (password !== confirmPassword) {
      error = "Passwords do not match";
      return;
    }

    if (username.length < 3) {
      error = "Username must be at least 3 characters";
      return;
    }

    if (password.length < 6) {
      error = "Password must be at least 6 characters";
      return;
    }

    loading = true;
    try {
      await doRegister(username, password);
      dispatch("success");
    } catch (e) {
      error = typeof e.message === "string" ? e.message : "Registration failed";
    } finally {
      loading = false;
    }
  }
</script>

<div class="auth-container">
  <div class="auth-card">
    <h1>Create an account</h1>

    {#if error}
      <div class="error">
        <span class="error-icon">!</span>
        {error}
      </div>
    {/if}

    <form on:submit|preventDefault={handleSubmit}>
      <div class="field">
        <label for="username">
          Username <span class="required">*</span>
        </label>
        <input
          id="username"
          type="text"
          bind:value={username}
          required
          class:error={error}
        />
      </div>

      <div class="field">
        <label for="password">
          Password <span class="required">*</span>
        </label>
        <input
          id="password"
          type="password"
          bind:value={password}
          required
          class:error={error}
        />
      </div>

      <div class="field">
        <label for="confirm">
          Confirm Password <span class="required">*</span>
        </label>
        <input
          id="confirm"
          type="password"
          bind:value={confirmPassword}
          required
          class:error={error}
        />
      </div>

      <button type="submit" disabled={loading}>
        {loading ? "Creating Account..." : "Continue"}
      </button>

      <p class="tos">
        By registering, you agree to Discord Clone's Terms of Service and Privacy Policy.
      </p>
    </form>

    <p class="switch">
      <button class="link" on:click={() => dispatch("switchToLogin")}>
        Already have an account?
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
    background-color: var(--bg-primary);
  }

  .auth-card {
    background-color: var(--bg-secondary);
    border-radius: 8px;
    padding: 32px;
    width: 480px;
    box-shadow: 0 2px 10px 0 rgba(0,0,0,0.2);
  }

  h1 {
    color: var(--header-primary);
    font-size: 24px;
    font-weight: 600;
    margin-bottom: 8px;
    text-align: center;
  }

  .error {
    background-color: rgba(218, 55, 60, 0.15);
    border-left: 4px solid var(--red);
    color: var(--text-normal);
    padding: 10px 16px;
    border-radius: 4px;
    margin-bottom: 20px;
    font-size: 14px;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .error-icon {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: var(--red);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    font-weight: 700;
    flex-shrink: 0;
  }

  .field {
    margin-bottom: 20px;
  }

  label {
    display: block;
    color: var(--header-secondary);
    font-size: 12px;
    font-weight: 700;
    text-transform: uppercase;
    margin-bottom: 8px;
    letter-spacing: 0.02em;
  }

  .required {
    color: var(--red);
  }

  input {
    width: 100%;
    padding: 10px;
    border-radius: 4px;
    border: none;
    background-color: var(--input-bg);
    color: var(--text-normal);
    font-size: 16px;
    outline: none;
    transition: border-color 0.15s ease;
  }

  input:focus {
    outline: none;
    box-shadow: 0 0 0 2px var(--brand);
  }

  input.error {
    box-shadow: 0 0 0 2px var(--red);
  }

  button[type="submit"] {
    width: 100%;
    padding: 12px;
    border-radius: 4px;
    border: none;
    background-color: var(--brand);
    color: white;
    font-size: 16px;
    font-weight: 500;
    cursor: pointer;
    margin-top: 4px;
    transition: background-color 0.17s ease, color 0.17s ease;
  }

  button[type="submit"]:hover:not(:disabled) {
    background-color: var(--brand-hover);
  }

  button[type="submit"]:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .tos {
    color: var(--text-muted);
    font-size: 12px;
    margin-top: 12px;
    text-align: center;
    line-height: 1.4;
  }

  .switch {
    color: var(--text-muted);
    font-size: 14px;
    margin-top: 8px;
    text-align: center;
  }

  .link {
    background: none;
    border: none;
    color: var(--text-link);
    cursor: pointer;
    font-size: 14px;
    padding: 0;
    text-decoration: none;
  }

  .link:hover {
    text-decoration: underline;
  }
</style>
