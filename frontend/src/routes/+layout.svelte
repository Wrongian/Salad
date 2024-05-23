<script lang="ts">
  import Errors from "$lib/components/Errors.svelte";
  import "../app.css";
  import { onMount } from "svelte";

  let doneLoad = false;
  onMount(async () => {
    // TODO: logic to render all the necessary components

    doneLoad = true;
  });
</script>

<svelte:head>
  <title>Welcome to Salad!</title>
</svelte:head>

<Errors />
{#if doneLoad}
  <slot />
{/if}

<footer class="p-2">
  <p class="text-center text-300 text-neutral-500">
    <span> All rights reserved @Salad. </span>
  </p>
</footer>
