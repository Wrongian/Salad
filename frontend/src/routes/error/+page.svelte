<script lang="ts">
  import type { PageData } from "../$types";
  import { err } from "../../stores/stores";
  import { goto } from "$app/navigation";

  export let data: PageData;
  $: console.log(data);

  err.subscribe(async () => {
    await goto("/error");
  });
  // listen for err store change
</script>

<header class="p-5">
  <h1 class="text-center">Error</h1>
</header>

<!--Description of the Error-->
<main>
  <div>
    <div class="flex items-center justify-center h-16">
      <div class="p-4">Status Code:</div>
    </div>
  </div>
</main>
