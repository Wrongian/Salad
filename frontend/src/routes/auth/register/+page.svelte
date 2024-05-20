<script lang="ts">
  import RegisterForm from "$lib/components/RegisterForm.svelte";
</script>

<header class="p-3">
  <h1 class="text-center">Register to start mixing!</h1>
</header>

<main
  class="flex flex-col border border-black items-center p-3 min-h-[500px] h-[80vh]"
>
  <div class="bg-slate-200 rounded-xl p-3 w-[450px]">
    <RegisterForm />
    <div class="flex justify-center">
      <a href="/auth/login">
        <span class="text-neutral-500 hover:text-green-950"
          >Already have an account? Log in!</span
        >
      </a>
    </div>
  </div>
</main>
