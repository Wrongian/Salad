<script lang="ts">
  import type { MouseEventHandler } from "svelte/elements";
  import { twMerge } from "tailwind-merge";

  export let canSubmit: boolean;
  export let onSubmit: MouseEventHandler<HTMLButtonElement>;
  export let buttonLabel: string;
</script>

<button
  type="submit"
  class={twMerge(
    "justify-center rounded-full py-2 w-full bg-primary-700 shadow-lg hover:bg-primary-600 text-muted hover:text-white",
    !canSubmit && "opacity-20 pointer-events-none hover:cursor-pointer",
  )}
  disabled={!canSubmit}
  on:click={onSubmit}
>
  <span>{buttonLabel}</span>
</button>
