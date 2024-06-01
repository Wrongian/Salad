<script lang="ts">
  import close from "$lib/assets/close.png";
  import { onMount } from "svelte";
  import { fade, fly } from "svelte/transition";

  export let message: string = "Something really bad has occurred.";
  export let status: number = 500;
  export let onClose: () => void = () => {};
</script>

<div
  in:fly={{ y: 200, duration: 500 }}
  out:fade={{ duration: 500 }}
  class="w-[450px] bg-red-100 border border-red-600 text-red-600 px-2 py-3 rounded relative
  overflow-y-hidden max-w-[350px] max-h-[200px]"
>
  <strong class="font-bold">Oh No!</strong>
  <span class="inline-block">
    {message}
  </span>
  <button
    class="p-0 ml-[90px] mr-0 my-0 bg-inherit hover:bg-inherit"
    on:click={onClose}
  >
    <img src={close} alt="X" class="bg-inherit hover:bg-inherit h-3 w-3" />
  </button>
</div>
