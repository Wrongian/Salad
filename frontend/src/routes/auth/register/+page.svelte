<script lang="ts">
  import RegisterForm from "$lib/components/RegisterForm.svelte";
</script>

<svelte:head>
  <title>Register</title>
</svelte:head>

<header class="p-3">
  <h1 class="text-center font-semibold text-xl">Register to join us!</h1>
</header>

<main class="flex flex-col items-center p-3 min-h-[500px] h-[80vh]">
  <div class="shadow-lg rounded-xl p-3 w-[450px] bg-background">
    <RegisterForm>
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
          <span class="text-neutral-500"> Already registered? </span>
          <a href="/auth/login" class="text-primary font-medium"> Log in! </a>
        </p>
      </div>
    </RegisterForm>
  </div>
</main>
