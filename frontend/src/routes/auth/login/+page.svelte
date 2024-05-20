<script lang="ts">
  import LoginForm from "$lib/components/LoginForm.svelte";
</script>

<header class="p-3">
  <h1 class="text-center">Login to continue</h1>
</header>

<main
  class="flex flex-col border border-black items-center p-3 min-h-[500px] h-[80vh]"
>
  <div class="bg-slate-200 rounded-xl p-3 w-[450px]">
    <LoginForm />
    <div class="flex justify-center">
      <a href="/auth/register">
        <span class="text-neutral-500 hover:text-green-950"
          >Forgot password?</span
        >
      </a>
    </div>
  </div>
</main>
