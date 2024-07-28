<script lang="ts">
  import {
    MAX_USERNAME_LENGTH,
    MIN_USERNAME_LENGTH,
  } from "$lib/modules/Constants.svelte";

  export let isUsernameChanged: boolean;
  export let username: string;
  export let id: string;
</script>

<label
  for="login-username-text"
  class="block text-sm font-medium text-slate-800"
>
  <span
    class="block text-sm font-medium text-slate-800 after:content-['*'] after:ml-0.5 after:text-red-500"
  >
    Username
  </span>
  <input
    {id}
    type="text"
    minlength={MIN_USERNAME_LENGTH}
    maxlength={MAX_USERNAME_LENGTH}
    class="bg-background p-2 peer w-full rounded-md text-sm shadow-sm border border-slate-300 mt-1
      focus:outline-none focus:invalid:border-error focus:invalid:ring-error
      invalid:border-error invalid:text-error
      "
    required={isUsernameChanged}
    on:input={() => (isUsernameChanged = true)}
    bind:value={username}
  />
  <p class="opacity-0 peer-invalid:opacity-100 text-error text-sm pt-2">
    Username must have at least {MIN_USERNAME_LENGTH}
    characters.
  </p>
</label>
