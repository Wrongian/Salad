<script lang="ts">
  import { afterNavigate, goto, invalidateAll } from "$app/navigation";
  import {
    getResetEmail,
    getUsername,
    resetPassword,
  } from "$lib/scripts/queries";
  import {
    type TGetEmailBody,
    type TResetPasswordBody,
  } from "$lib/scripts/query.d";
  let email: string = "";
  let code: string = "";
  let password: string = "";
  let toggle: boolean = false;

  async function sendRequest() {
    let email_body: TGetEmailBody = {
      email: email,
    };
    let is_sent: boolean = await getResetEmail(email_body);
    if (is_sent) {
      toggle = true;
    }
  }
  async function sendResetPassword() {
    let reset_body: TResetPasswordBody = {
      email: email,
      code: code,
      password: password,
    };

    let is_reset: boolean = await resetPassword(reset_body);
    if (is_reset) {
      await invalidateAll();
      goto(next);
    }
    code = "";
    password = "";
  }

  let next = "";
  afterNavigate(({ from }) => {
    next = from?.url.pathname || next;
    // change later to dynamic route
    // or later use svelte store to do this instead in the outermost layout route
    if (next == "/auth/reset-password") {
      next = "/";
    }
  });
</script>

{#if !toggle}
  <div class="form">
    <label for="login-reset-text" class="block text-sm font-medium"
      >Enter your email:</label
    >
    <input
      type="text"
      id="login-reset-text"
      class="px-1 peer block mt-1 w-full rounded-md text-sm shadow-sm bg-primary border border-slate-300"
      bind:value={email}
    />

    <div class="py-3 flex justify-center">
      <button
        type="submit"
        class="justify-center rounded-md w-[200px]"
        on:click={async () => await sendRequest()}
      >
        <span>Submit</span>
      </button>
    </div>
  </div>
{/if}

{#if toggle}
  <div class="form">
    <label for="code-text" class="block text-sm font-medium py-2">
      <span
        class="block text-sm font-medium text-slate-800 after:content-['*'] after:ml-0.5 after:text-red-500"
      >
        Verification Code</span
      >
      <input
        id="code-text"
        type="text"
        class="px-1 peer block mt-1 w-full rounded-md text-sm shadow-sm bg-primary border border-slate-300
      focus:outline-none focus:invalid:border-invalid focus:invalid:ring-invalid invalid:border-invalid invalid:text-invalid
      "
        bind:value={code}
      />
    </label>

    <label for="new-password-text" class="block text-sm font-medium py-2">
      <span
        class="block text-sm font-medium text-slate-800 after:content-['*'] after:ml-0.5 after:text-red-500"
      >
        New Password
      </span>
      <input
        id="new-password-text"
        type="password"
        class="px-1 peer block w-full rounded-md text-sm shadow-sm border border-slate-300 bg-primary mt-1
      focus:outline-none focus:invalid:border-invalid focus:invalid:ring-invalid invalid:border-invalid invalid:text-invalid
      "
        bind:value={password}
      />
    </label>
    <div class="py-3 flex justify-center">
      <button
        type="submit"
        class="justify-center rounded-md w-[200px]"
        on:click={async () => await sendResetPassword()}
      >
        <span>Submit</span>
      </button>
    </div>
    <div class="py-3 flex justify-center">
      <button
        type="submit"
        class="justify-center rounded-md w-[200px]"
        on:click={async () => {
          email = "";
          toggle = false;
        }}
      >
        <span>Back</span>
      </button>
    </div>
  </div>
{/if}
