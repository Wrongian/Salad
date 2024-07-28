<script lang="ts">
  import { login } from "$lib/scripts/queries";
  import { goto } from "$app/navigation";

  import UsernameFormField from "./forms/UsernameFormField.svelte";
  import PasswordFormField from "./forms/PasswordFormField.svelte";
  import FormSubmitButton from "./forms/FormSubmitButton.svelte";
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

    return (
      usernameElement.validity.valid &&
      passwordElement.validity.valid &&
      isPasswordChanged &&
      isUsernameChanged
    );
  };
  // let next = "";
  // afterNavigate(({ from }) => {
  //   next = from?.url.pathname || next;
  //   // change later to dynamic route
  //   // or later use svelte store to do this instead in the outermost layout route
  //   if (next == "/auth/login") {
  //     next = "/";
  //   }
  // });
</script>

<div class="form">
  <UsernameFormField
    bind:isUsernameChanged
    bind:username
    id="login-username-text"
  />

  <PasswordFormField
    bind:isPasswordChanged
    bind:password
    id="login-password-text"
  >
    <slot slot="forgot-password" name="forgot-password" />
  </PasswordFormField>

  <FormSubmitButton
    onSubmit={() =>
      login(username, password).then((success) => {
        if (success) {
          goto(`/profiles/${username}`, { invalidateAll: true });
        }
      })}
    bind:canSubmit
    buttonLabel="Log in"
  />
  <slot name="footer" />
</div>
