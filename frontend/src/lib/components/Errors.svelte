<script lang="ts">
  import type { TRenderErrorProp } from "$lib/types/ErrorTypes";
  import { setContext, getContext, onMount } from "svelte";
  import ErrorFlashCard from "./ErrorFlashCard.svelte";

  setContext("error", {
    addError,
    removeAt,
  });
  let errors: TRenderErrorProp[];
  $: errors = [];

  function addError(message: string, statusCode: number) {
    errors = [...errors, { id: errors.length, message, statusCode }];
  }

  /**
   * removes the stored error in the queue at position index
   * NOOP if index is out of bounds
   * @param index
   */
  function removeAt(index: number) {
    if (index < 0 || index >= errors.length) return;
    const newErrs: TRenderErrorProp[] = [];
    errors.forEach((el, id) => {
      if (id != index) newErrs.push({ ...el, id: newErrs.length });
    });
    errors = newErrs;
  }
</script>

<div class="fixed inset-x-[25vw] w-[50vw] border border-black">
  {#each errors as error (error.id)}
    <ErrorFlashCard message={error.message} status={error.statusCode} />
  {/each}
</div>
