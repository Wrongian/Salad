<script lang="ts">
  import Errors from "$lib/components/Errors.svelte";
  import "../app.css";
  import { onMount } from "svelte";
  import type { PageData } from "./$types";
  import Navbar from "$lib/components/navbar/Navbar.svelte";
  export let data: PageData;

  let profileRoute: string;
  $: if (data.username === "") {
    // go to normal login page
    profileRoute = "/auth/login";
  } else {
    profileRoute = `/profiles/` + data.username;
  }

  let doneLoad = false;
  onMount(async () => {
    // TODO: logic to render all the necessary components
    // previous url
    doneLoad = true;
  });
  // let next = "/";
  // afterNavigate(({from}) => {
  //   next = from?.url.pathname || next
  // })
  // dropdown menu
  // default
</script>

<svelte:head>
  <!--placeholder title-->
  <title>Welcome to Saladify!</title>
</svelte:head>
<Errors />
<div class="h-fit min-h-screen bg-white">
  <Navbar
    {profileRoute}
    isLoggedIn={data.isLoggedIn}
    notifications={data.notifications}
  />

  {#if doneLoad}
    <div>
      <slot />
      <footer class="h-auto text-center">
        <span> All rights reserved @Saladify. </span>
      </footer>
    </div>
  {/if}
</div>
