<script lang="ts" context="module">
  import type { TErrorContext } from "$lib/types/ErrorTypes";
  import { errorStore } from "../../stores/stores";

  let errors: TErrorContext[];

  export function addError(message: string, statusCode: number) {
    errorStore.update((errs) => {
      return [...errs, { id: errs.length, message, statusCode }];
    });
  }

  /**
   * removes the stored error in the queue at position index
   * NOOP if index is out of bounds
   * @param index
   */
  export function removeAt(index: number) {
    if (index < 0 || index >= errors.length) return;
    const newErrs: TErrorContext[] = [];
    errors.forEach((el, id) => {
      if (id != index) newErrs.push({ ...el, id: newErrs.length });
    });
    errors = newErrs;
  }
</script>
