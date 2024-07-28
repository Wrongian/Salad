<script lang="ts">
  // set to true once user has started typing
  export let isFormValueChanged: boolean = false;

  export let value: string;
  export let id: string;
  export let formInputLabel: string;
</script>

<label for={id} class="block text-sm font-medium py-2">
  <span
    class="block text-sm font-medium text-slate-800 after:content-['*'] after:ml-0.5 after:text-red-500"
  >
    {formInputLabel}</span
  >
  <input
    {id}
    type="text"
    class="p-2 block peer mt-1 w-full border border-slate-300 rounded-md text-sm shadow-sm
      focus:outline-none focus:invalid:border-destructive focus:invalid:ring-destructive invalid:border-destructive invalid:text-destructive
      "
    bind:value
    on:input={() => (isFormValueChanged = true)}
  />
</label>
