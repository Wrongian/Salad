<script lang="ts">
  import { twMerge } from "tailwind-merge";
  import { register } from "../../scripts/queries";
  import {
    MAX_PASSWORD_LENGTH,
    MAX_USERNAME_LENGTH,
    MIN_PASSWORD_LENGTH,
    MIN_USERNAME_LENGTH,
  } from "$lib/modules/Constants.svelte";

  let email = "";
  let username = "";
  let password = "";

  let canSubmit = false;
  let isPasswordChanged = false;
  let isUsernameChanged = false;
  let isEmailChanged = false;

  // re run statement whenever username, password or email state changes
  $: (canSubmit = checkValid()), username, password, email;

  const checkValid = () => {
    // email
    const emailElement = document.getElementById("register-email-text");
    if (!emailElement || !(emailElement instanceof HTMLInputElement))
      return false;

    // username
    const usernameElement = document.getElementById("register-username-text");
    if (!usernameElement || !(usernameElement instanceof HTMLInputElement))
      return false;

    // password
    const passwordElement = document.getElementById("register-password-text");
    if (!passwordElement || !(passwordElement instanceof HTMLInputElement))
      return false;

    return (
      emailElement.validity.valid &&
      usernameElement.validity.valid &&
      passwordElement.validity.valid &&
      !!isPasswordChanged &&
      !!isUsernameChanged &&
      !!isEmailChanged
    );
  };
</script>

<div class="form">
  <label for="register-email-text" class="block text-sm font-medium py-2">
    <span
      class="block text-sm font-medium text-slate-800 after:content-['*'] after:ml-0.5 after:text-red-500"
    >
      Email
    </span>
    <input
      id="register-email-text"
      type="email"
      class="block peer mt-1 w-full bg-primary border border-slate-300 rounded-md text-sm shadow-sm
      focus:outline-none focus:invalid:border-invalid focus:invalid:ring-invalid invalid:border-invalid invalid:text-invalid
      "
      required={isEmailChanged}
      on:input={() => (isEmailChanged = true)}
      bind:value={email}
    />
    <p class="hidden peer-invalid:block text-invalid text-sm pt-2">
      Please provide a valid email.
    </p>
  </label>

  <label for="register-username-text" class="block text-sm font-medium py-2">
    <span
      class="block text-sm font-medium text-slate-800 after:content-['*'] after:ml-0.5 after:text-red-500"
    >
      Username</span
    >
    <input
      id="register-username-text"
      type="text"
      minlength={MIN_USERNAME_LENGTH}
      maxlength={MAX_USERNAME_LENGTH}
      class="peer block mt-1 w-full rounded-md text-sm shadow-sm bg-primary border border-slate-300
      focus:outline-none focus:invalid:border-invalid focus:invalid:ring-invalid invalid:border-invalid invalid:text-invalid
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

  <label for="register-password-text" class="block text-sm font-medium py-2">
    <span
      class="block text-sm font-medium text-slate-800 after:content-['*'] after:ml-0.5 after:text-red-500"
    >
      Password</span
    >
    <input
      id="register-password-text"
      type="password"
      minlength={MIN_PASSWORD_LENGTH}
      maxlength={MAX_PASSWORD_LENGTH}
      class="peer block mt-1 w-full rounded-md text-sm shadow-sm bg-primary border border-slate-300
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
      on:click={() => register(email, username, password)}
    >
      <span>Register</span>
    </button>
  </div>
</div>
