<script lang="ts">
  // set to true once user has started typing
  export let isEmailChanged: boolean = false;

  export let email: string;
  export let id: string;
  export let emailInputLabel: string = "Email";
</script>

<label for={id} class="block text-sm font-medium py-2">
  <span
    class="block text-sm font-medium text-slate-800 after:content-['*'] after:ml-0.5 after:text-red-500"
  >
    {emailInputLabel}
  </span>
  <input
    {id}
    type="email"
    class="p-2 block peer mt-1 w-full border border-slate-300 rounded-md text-sm shadow-sm
      focus:outline-none focus:invalid:border-destructive focus:invalid:ring-destructive invalid:border-destructive invalid:text-destructive
      "
    required={isEmailChanged}
    on:input={() => (isEmailChanged = true)}
    bind:value={email}
  />
  <p class="opacity-0 peer-invalid:opacity-100 text-destructive text-sm pt-2">
    Please provide a valid email.
  </p>
</label>
