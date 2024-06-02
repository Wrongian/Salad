<script lang="ts">
  import RegisterForm from "$lib/components/RegisterForm.svelte";
</script>

<svelte:head>
  <title>Register</title>
</svelte:head>

<header class="p-3">
  <h1 class="text-center">Register to join us!</h1>
</header>

<main
  class="flex flex-col border border-black items-center p-3 min-h-[500px] h-[80vh]"
>
  <div class="bg-primary shadow-lg rounded-xl p-3 w-[450px]">
    <RegisterForm />
    <div class="flex justify-center">
      <a href="/auth/login">
        <span class="text-slate-500 hover:text-secondary"
          >Already have an account? Log in!</span
        >
      </a>
    </div>
  </div>
</main>
