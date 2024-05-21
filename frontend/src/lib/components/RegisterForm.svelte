<script lang="ts">
  import { register } from "../../scripts/queries";

  let email = "";
  let username = "";
  let password = "";
</script>

<div class="form">
  <label for="register-email-text" class="block text-sm font-medium py-2"
    >Email</label
  >
  <input
    id="register-email-text"
    type="text"
    class="w-full rounded"
    bind:value={email}
  />

  <label for="register-username-text" class="block text-sm font-medium py-2"
    >Username</label
  >
  <input
    id="register-username-text"
    type="text"
    class="w-full rounded"
    bind:value={username}
  />

  <label for="password" class="block text-sm font-medium py-2">Password</label>
  <input
    id="password"
    type="password"
    class="w-full rounded"
    bind:value={password}
  />
  <div class="py-3 flex justify-center">
    <button
      type="submit"
      class="justify-center rounded-md w-[200px]"
      on:click={() => register(email, username, password)}
    >
      <span>Register</span>
    </button>
  </div>
</div>
