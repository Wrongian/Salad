<script lang="ts">
  import { register } from "$lib/scripts/queries";

  import { afterNavigate, goto } from "$app/navigation";
  import EmailFormField from "./forms/EmailFormField.svelte";
  import UsernameFormField from "./forms/UsernameFormField.svelte";
  import PasswordFormField from "./forms/PasswordFormField.svelte";
  import FormSubmitButton from "./forms/FormSubmitButton.svelte";

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
      isPasswordChanged &&
      isUsernameChanged &&
      isEmailChanged
    );
  };
  // let next = "";
  // afterNavigate(({ from }) => {
  //   next = from?.url.pathname || next;
  //   // change later to dynamic route
  //   if (next == "/auth/register") {
  //     next = "/";
  //   }
  // });
</script>

<div class="form">
  <EmailFormField id="register-email-text" bind:isEmailChanged bind:email />

  <UsernameFormField
    bind:isUsernameChanged
    bind:username
    id="register-username-text"
  />

  <PasswordFormField
    bind:isPasswordChanged
    bind:password
    id="register-password-text"
  >
    <slot name="forgot-password" slot="forgot-password" />
  </PasswordFormField>

  <FormSubmitButton
    bind:canSubmit
    onSubmit={() =>
      register(email, username, password).then((result) =>
        result ? goto(`/profiles/${username}`, { invalidateAll: true }) : {},
      )}
    buttonLabel="Register"
  />

  <slot name="footer" />
</div>
