<script lang="ts">
  import LoginForm from "$lib/components/LoginForm.svelte";
  import LoginResetForm from "$lib/components/LoginResetForm.svelte";
  import type { PageData } from "./$types";
  export let data: PageData;
  $: reset = data.reset;
</script>

<svelte:head>
  <title>Login</title>
</svelte:head>

<header class="p-3">
  {#if reset}
    <h1 class="text-center font-semibold text-xl">Forgot password</h1>
  {:else}
    <h1 class="text-center font-semibold text-xl">Login to continue</h1>
  {/if}
</header>

<main class="flex flex-col items-center p-3 min-h-[500px] h-[80vh]">
  <div class="shadow-lg rounded-xl p-3 w-[450px] bg-background">
    {#if reset}
      <LoginResetForm />
      <div class="flex justify-center">
        <a href="/auth/login" on:click={() => (reset = false)}>
          <span class="text-neutral-500 hover:text-green-950"
            >Already have an account? Log in!</span
          >
        </a>
      </div>
    {:else}
      <LoginForm>
        <a
          href="/auth/reset-password"
          slot="forgot-password"
          class="text-nowrap text-sm"
        >
          <span class="text-neutral-500 hover:text-primary-600"
            >Forgot Password?</span
          >
        </a>

        <div class="flex justify-center pt-2" slot="footer">
          <p>
            <span class="text-neutral-500"> Don't have an account? </span>
            <a
              href="/auth/register"
              on:click={() => (reset = false)}
              class="text-primary font-medium"
            >
              Join us!
            </a>
          </p>
        </div>
      </LoginForm>
    {/if}
  </div>
</main>
