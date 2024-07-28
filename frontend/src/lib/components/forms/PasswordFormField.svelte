<script lang="ts">
  import {
    MAX_PASSWORD_LENGTH,
    MIN_PASSWORD_LENGTH,
  } from "$lib/modules/Constants.svelte";
  // set to true once user has started typing
  export let isPasswordChanged: boolean = false;

  export let password: string;
  export let id: string;
  export let passwordInputLabel: string = "Password";
</script>

<label for={id} class="grid grid-cols-5 text-sm font-medium py-2">
  <span
    class="col-span-full text-sm font-medium text-slate-800 after:content-['*'] after:ml-0.5 after:text-red-500"
  >
    {passwordInputLabel}
  </span>

  <input
    {id}
    type="password"
    minlength={MIN_PASSWORD_LENGTH}
    maxlength={MAX_PASSWORD_LENGTH}
    class="bg-background p-2 col-span-full peer rounded-md text-sm shadow-sm border border-slate-300 mt-1
      focus:outline-none focus:invalid:border-error focus:invalid:ring-error invalid:border-error
      invalid:text-error
      "
    required={isPasswordChanged}
    on:input={() => (isPasswordChanged = true)}
    bind:value={password}
  />
  <p
    class="col-span-3 opacity-0 peer-invalid:opacity-100 text-error text-sm pt-2"
  >
    Password must have at least {MIN_PASSWORD_LENGTH} characters.
  </p>
  <div class="col-span-2 text-end pt-2">
    <slot name="forgot-password" />
  </div>
</label>
