<script lang="ts">
  import LoginResetForm from "$lib/components/LoginResetForm.svelte";
</script>

<svelte:head>
  <title>Reset Password</title>
</svelte:head>

<header class="p-3">
  <h1 class="text-center font-semibold text-xl">Reset your password</h1>
</header>

<main class="flex flex-col items-center p-3 min-h-[500px] h-[80vh]">
  <div class="shadow-lg rounded-xl p-3 w-[450px] bg-background">
    <LoginResetForm />
  </div>
</main>
