<script lang="ts">
  import { afterNavigate, goto, invalidateAll } from "$app/navigation";
  import { getResetEmail, resetPassword } from "$lib/scripts/queries";
  import {
    type TGetEmailBody,
    type TResetPasswordBody,
  } from "$lib/scripts/query.d";
  import { ArrowLeft } from "lucide-svelte";
  import FormSubmitButton from "./forms/FormSubmitButton.svelte";
  import InputFormField from "./forms/InputFormField.svelte";
  import PasswordFormField from "./forms/PasswordFormField.svelte";
  import EmailFormField from "./forms/EmailFormField.svelte";
  let verificationCode: string = "";
  let toggle: boolean = false;
  let canSubmit = false;

  let isPasswordChanged = false;
  let password: string = "";

  let email: string = "";

  $: (canSubmit = checkValid()), password;

  const checkValid = () => {
    // new password
    const passwordElement = document.getElementById("reset-password-text");
    if (!passwordElement || !(passwordElement instanceof HTMLInputElement))
      return false;

    return passwordElement.validity.valid && isPasswordChanged;
  };

  async function sendRequest() {
    let emailBody: TGetEmailBody = {
      email: email,
    };
    let isSent: boolean = await getResetEmail(emailBody);
    if (isSent) {
      toggle = true;
    }
  }
  async function sendResetPassword() {
    let resetBody: TResetPasswordBody = {
      email: email,
      code: verificationCode,
      password: password,
    };

    let isReset: boolean = await resetPassword(resetBody);
    if (isReset) {
      await invalidateAll();
      goto(next);
    }
    verificationCode = "";
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

<div class="form">
  {#if !toggle}
    <EmailFormField
      bind:email
      id="reset-email-text"
      emailInputLabel="Enter your email"
    />
    <FormSubmitButton canSubmit onSubmit={sendRequest} buttonLabel="Submit" />
  {:else}
    <InputFormField
      bind:value={verificationCode}
      id="reset-code-text"
      formInputLabel="Verification code"
    />

    <PasswordFormField
      bind:password
      bind:isPasswordChanged
      id="reset-password-text"
      passwordInputLabel="New password"
    />
    <FormSubmitButton
      bind:canSubmit
      onSubmit={() => sendResetPassword()}
      buttonLabel="Reset password"
    />
    <div class="py-3 flex justify-center">
      <button
        type="submit"
        class="justify-center rounded-full p-2 bg-primary-700 text-white opacity-90 hover:opacity-100"
        on:click={async () => {
          email = "";
          toggle = false;
        }}
      >
        <ArrowLeft />
      </button>
    </div>
  {/if}
</div>
