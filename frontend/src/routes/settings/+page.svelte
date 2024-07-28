<script lang="ts">
  import {
    changeEmail,
    changePassword,
    changeUsername,
    updatePrivacy,
  } from "$lib/scripts/queries";
  import type {
    TChangeEmailBody,
    TChangePasswordBody,
    TChangeUsernameBody,
    TUpdatePrivacyBody,
  } from "$lib/scripts/query";
  import type { PageData } from "./$types";

  export let data: PageData;
  let username: string = "";
  let password: string = "";
  let email: string = "";
  console.log(data);
  let privacy: boolean = data.profileData?.is_private ?? false;

  async function submitUsername() {
    let query: TChangeUsernameBody = { username: username };
    await changeUsername(query);
    username = "";
  }

  async function submitPassword() {
    let query: TChangePasswordBody = { password: password };
    await changePassword(query);
    password = "";
  }

  async function submitEmail() {
    let query: TChangeEmailBody = { email: email };
    await changeEmail(query);
    email = "";
  }

  function submitPrivacy() {
    let query: TUpdatePrivacyBody = { is_private: privacy };
    updatePrivacy(query);
  }
</script>

<div class="justify-start my-auto px-4 py-4">
  <!--Username form-->
  <form>
    <label for="change-username" class="mb-2 text-sm font-medium text-gray-900"
      >Change Username</label
    >
    <div class="relative">
      <input
        type="text"
        id="change-username"
        class="block w-full p-4 text-sm border border-2 border-lime-300 rounded-lg bg-lime-50 focus:border-lime-400"
        placeholder=""
        required
        bind:value={username}
      />
      <button
        type="submit"
        on:click={async () => await submitUsername()}
        class="text-white absolute end-2.5 bottom-2.5 bg-lime-700 hover:bg-lime-800 font-medium rounded-lg text-sm px-4 py-2"
        >Submit</button
      >
    </div>
  </form>

  <!--Password form-->
  <form>
    <label for="change-password" class="mb-2 text-sm font-medium text-gray-900"
      >Change Password</label
    >
    <div class="relative">
      <input
        type="password"
        id="change-password"
        class="block w-full p-4 text-sm border border-2 border-lime-300 rounded-lg bg-lime-50 focus:border-lime-400"
        placeholder=""
        required
        bind:value={password}
      />
      <button
        type="submit"
        on:click={async () => await submitPassword()}
        class="text-white absolute end-2.5 bottom-2.5 bg-lime-700 hover:bg-lime-800 font-medium rounded-lg text-sm px-4 py-2"
        >Submit</button
      >
    </div>
  </form>
  <form>
    <label for="change-email" class="mb-2 text-sm font-medium text-gray-900"
      >Change Email</label
    >
    <div class="relative">
      <input
        type="text"
        id="change-email"
        class="block w-full p-4 text-sm border border-2 border-lime-300 rounded-lg bg-lime-50 focus:border-lime-400"
        placeholder=""
        required
        bind:value={email}
      />
      <button
        type="submit"
        on:click={async () => await submitEmail()}
        class="text-white absolute end-2.5 bottom-2.5 bg-lime-700 hover:bg-lime-800 font-medium rounded-lg text-sm px-4 py-2"
        >Submit</button
      >
    </div>
  </form>

  <form>
    <div class="flex items-center mb-2">
      <input
        id="change_privacy"
        type="checkbox"
        bind:checked={privacy}
        on:focusout={() => {
          submitPrivacy();
        }}
        class="w-4 h-4 bg-lime-100 border-gray-300 rounded focus:ring-lime-500 focus:ring-2 accent-lime-500"
      />
      <label for="change-privacy" class="ms-2 text-sm font-medium text-gray-900"
        >Private Profile Setting</label
      >
    </div>
  </form>
</div>
