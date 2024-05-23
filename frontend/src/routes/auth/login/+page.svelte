<script lang="ts">
  import LoginForm from "$lib/components/LoginForm.svelte";
  import LoginResetForm from "$lib/components/LoginResetForm.svelte";
  import type { PageData } from "./$types";
  export let data: PageData;
  $: reset = data.reset;
</script>

<header class="p-3">
  {#if reset}
    <h1 class="text-center">Forgot password</h1>
  {:else}
    <h1 class="text-center">Login to continue</h1>
  {/if}
</header>

<main
  class="flex flex-col border border-black items-center p-3 min-h-[500px] h-[80vh]"
>
  <div class="bg-slate-200 rounded-xl p-3 w-[450px]">
    {#if reset}
      <LoginResetForm />
      <!--TODO: Modularise this into a svelte component  -->
      <div class="flex justify-center">
        <a href="/auth/login" on:click={() => (reset = false)}>
          <span class="text-neutral-500 hover:text-green-950"
            >Already have an account? Log in!</span
          >
        </a>
      </div>
    {:else}
      <LoginForm />
      <div class="flex justify-center">
        <a href="/auth/login" on:click={() => (reset = true)}>
          <span class="text-neutral-500 hover:text-green-950"
            >Forgot password?</span
          >
        </a>
      </div>
      <div class="flex justify-center">
        <a href="/auth/register" on:click={() => (reset = false)}>
          <span class="text-neutral-500 hover:text-green-950"
            >Don't have an account? Join us!</span
          >
        </a>
      </div>
    {/if}
  </div>
</main>
