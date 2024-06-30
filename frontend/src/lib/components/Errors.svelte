<script lang="ts">
  import type { TErrorContext } from "$lib/types/ErrorTypes";
  import ErrorFlashCard from "./ErrorFlashCard.svelte";
  import { errorStore } from "../../stores/stores";
  import { removeAt } from "$lib/modules/Errors.svelte";

  let errors: TErrorContext[] = []
  $: errors = Array.from($errorStore.values())

</script>

<div class="z-50 fixed left-2 top-2">
  {#each errors as error (error.id)}
    <ErrorFlashCard
      message={error.message}
      status={error.statusCode}
      onClose={() => removeAt(error.id)}
    />
  {/each}
</div>
