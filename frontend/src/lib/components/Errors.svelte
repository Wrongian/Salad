<script lang="ts">
  import type { TErrorContext } from "$lib/types/ErrorTypes";
  import ErrorFlashCard from "./ErrorFlashCard.svelte";
  import { errorStore } from "../../stores/stores";

  let errors: TErrorContext[] = [];
  errorStore.subscribe((errs) => (errors = errs));
</script>

<div class="fixed inset-x-[25vw] w-[50vw] border border-black">
  {#each errors as error (error.id)}
    <ErrorFlashCard message={error.message} status={error.statusCode} />
  {/each}
</div>
