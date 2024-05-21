<script lang="ts">
  import { login } from "../../scripts/queries";
  let username: string = "";
  let password: string = "";
</script>

<div class="form">
  <label for="login-text" class="block text-sm font-medium py-2">Username</label
  >
  <input
    id="login-text"
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
      on:click={() => login(username, password)}
    >
      <span>Submit</span>
    </button>
  </div>
</div>
