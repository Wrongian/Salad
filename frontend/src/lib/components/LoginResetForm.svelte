<script lang="ts">
  import { resetPassword } from "$lib/scripts/queries";
  let email: string = "";
</script>

<div class="form">
  <label for="login-reset-text" class="block text-sm font-medium"
    >Enter your email:</label
  >
  <input
    type="text"
    id="login-reset-text"
    class="block text-sm font-medium w-full rounded"
    bind:value={email}
  />
  <div class="py-3 flex justify-center">
    <button
      type="submit"
      class="justify-center rounded-md w-[200px]"
      on:click={() => resetPassword(email)}
    >
      <span>Submit</span>
    </button>
  </div>
</div>
