<script lang="ts">
  import LoginResetForm from "$lib/components/LoginResetForm.svelte";
</script>

<svelte:head>
  <title>Reset Password</title>
</svelte:head>

<header class="p-3">
  <h1 class="text-center">Reset your password</h1>
</header>

<main
  class="flex flex-col border border-black items-center p-3 min-h-[500px] h-[80vh]"
>
  <div class="bg-primary shadow-lg rounded-xl p-3 w-[450px]">
    <LoginResetForm />
    <div class="flex justify-center"></div>
  </div>
</main>
