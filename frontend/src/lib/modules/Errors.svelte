<script lang="ts" context="module">
  import { browser } from "$app/environment";
  import { goto } from "$app/navigation";
  import type { TErrorContext } from "$lib/types/ErrorTypes";
  import { blackSwanError, errorStore } from "../../stores/stores";

  export function addError(message: string, statusCode: number) {
    errorStore.update((errs) => {
      return [...errs, { id: errs.length, message, statusCode }];
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
   * @param index
   */
  export function removeAt(index: number) {
    errorStore.update((errors) => {
      const newErrs: TErrorContext[] = [];
      if (index < 0 || index >= errors.length) return errors;
      errors.forEach((el, id) => {
        if (id != index) newErrs.push({ ...el, id: newErrs.length });
      });
      return newErrs;
    });
  }
</script>
