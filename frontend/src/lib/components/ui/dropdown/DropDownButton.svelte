<script lang="ts">
  import type { MouseEventHandler } from "svelte/elements";

  export let buttonTitle: string;
  export let onButtonClick: MouseEventHandler<HTMLButtonElement>;
</script>

<button
  class="hover:bg-lime-200 hover:text-gray-800 block px-4 py-2 text-sm text-gray-700 w-full text-left"
  on:click={onButtonClick}
>
  {buttonTitle}
</button>
