<script lang="ts">
  import { goto } from "$app/navigation";
  import {
    get_reset_email,
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

  async function send_request() {
    let email_body: TGetEmailBody = {
      email: email,
    };
    toggle = true;
    await get_reset_email(email_body);
  }
  async function send_reset_password() {
    let reset_body: TResetPasswordBody = {
      email: email,
      code: code,
      password: password,
    };

    await resetPassword(reset_body);
    goto("/");
  }
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
        on:click={async () => await send_request()}
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
        class="peer block w-full rounded-md text-sm shadow-sm border border-slate-300 bg-primary mt-1
      focus:outline-none focus:invalid:border-invalid focus:invalid:ring-invalid invalid:border-invalid invalid:text-invalid
      "
        bind:value={password}
      />
    </label>
    <div class="py-3 flex justify-center">
      <button
        type="submit"
        class="justify-center rounded-md w-[200px]"
        on:click={async () => await send_reset_password()}
      >
        <span>Submit</span>
      </button>
    </div>
  </div>
{/if}
