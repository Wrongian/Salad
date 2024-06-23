<script lang="ts">
  import { twMerge } from "tailwind-merge";
  import { login } from "$lib/scripts/queries";
  import {
    MAX_PASSWORD_LENGTH,
    MAX_USERNAME_LENGTH,
    MIN_PASSWORD_LENGTH,
    MIN_USERNAME_LENGTH,
  } from "$lib/modules/Constants.svelte";
  let username: string = "";
  let password: string = "";

  let canSubmit = false;
  let isPasswordChanged = false;
  let isUsernameChanged = false;
  $: (canSubmit = checkValid()), username, password;

  const checkValid = () => {
    // username
    const usernameElement = document.getElementById("login-username-text");
    if (!usernameElement || !(usernameElement instanceof HTMLInputElement))
      return false;

    // password
    const passwordElement = document.getElementById("login-password-text");
    if (!passwordElement || !(passwordElement instanceof HTMLInputElement))
      return false;

    return usernameElement.validity.valid && passwordElement.validity.valid;
  };
</script>

<div class="form">
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
      id="login-username-text"
      type="text"
      minlength={MIN_USERNAME_LENGTH}
      maxlength={MAX_USERNAME_LENGTH}
      class="peer block w-full rounded-md text-sm shadow-sm border border-slate-300 bg-primary mt-1
      focus:outline-none focus:invalid:border-destructive focus:invalid:ring-destructive invalid:border-destructive invalid:text-destructive
      "
      required={isUsernameChanged}
      on:input={() => (isUsernameChanged = true)}
      bind:value={username}
    />
    <p class="hidden peer-invalid:block text-invalid text-sm pt-2">
      Username must have at least {MIN_USERNAME_LENGTH}
      characters.
    </p>
  </label>

  <label for="login-password-text" class="block text-sm font-medium py-2">
    <span
      class="block text-sm font-medium text-slate-800 after:content-['*'] after:ml-0.5 after:text-red-500"
    >
      Password
    </span>
    <input
      id="login-password-text"
      type="password"
      minlength={MIN_PASSWORD_LENGTH}
      maxlength={MAX_PASSWORD_LENGTH}
      class="peer block w-full rounded-md text-sm shadow-sm border border-slate-300 bg-primary mt-1
      focus:outline-none focus:invalid:border-invalid focus:invalid:ring-invalid invalid:border-invalid invalid:text-invalid
      "
      required={isPasswordChanged}
      on:input={() => (isPasswordChanged = true)}
      bind:value={password}
    />
    <p class="hidden peer-invalid:block text-invalid text-sm pt-2">
      Password must have at least {MIN_PASSWORD_LENGTH} characters.
    </p>
  </label>

  <div class="py-3 flex justify-center">
    <button
      type="submit"
      class={twMerge(
        "justify-center rounded-md w-[200px] bg-primary ring-1 ring-secondary shadow-lg",
        !canSubmit && "opacity-40 pointer-events-none"
      )}
      disabled={!canSubmit}
      on:click={() => login(username, password)}
    >
      <span>Submit</span>
    </button>
  </div>
</div>
