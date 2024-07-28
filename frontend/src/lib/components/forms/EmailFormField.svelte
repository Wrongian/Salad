<script lang="ts">
  export let isEmailChanged: boolean;
  export let email: string;
  export let id: string;
</script>

<label for="register-email-text" class="block text-sm font-medium py-2">
  <span
    class="block text-sm font-medium text-slate-800 after:content-['*'] after:ml-0.5 after:text-red-500"
  >
    Email
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
