<script lang="ts" context="module">
  import { browser } from "$app/environment";
  import { goto } from "$app/navigation";
  import type { TErrorContext } from "$lib/types/ErrorTypes";
  import { blackSwanError, errorStore } from "../../stores/stores";
  import {v4 as uuidv4 } from 'uuid';

  const ERROR_TIMEOUT_MS = 5000;

  export function addError(message: string, statusCode: number) {

    errorStore.update((errs) => {
      const errorHash = uuidv4();

      setTimeout(() => {
        removeAt(errorHash)
      }, ERROR_TIMEOUT_MS)

      return errs.set(errorHash, {
        id: errorHash,
        statusCode,
        message
      })
    });

    
  }
  // blackswan error logic hook here (on client only)
  if (browser) {
    blackSwanError.subscribe((errLike) => {
      if (errLike) {
        goto("/error");
      }
    });
  }

  /**
   * removes the stored error in the queue at position index
   * NOOP if index is out of bounds
   * @param errorId
   */
  export function removeAt(errorHash: string) {
    errorStore.update((errors) => {
      errors.delete(errorHash);
      return errors
    });
  }
</script>
