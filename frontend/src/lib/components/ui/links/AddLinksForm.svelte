<script lang="ts">
  import { invalidateAll } from "$app/navigation";
  import { addLinks } from "$lib/scripts/queries";

  let linkName = "";
  let linkUrl = "";
  let linkDescription = "";

  const submitLink = async () => {
    await addLinks({
      href: linkUrl,
      title: linkName ?? undefined,
      bio: linkDescription ?? undefined,
    });
    linkName = "";
    linkUrl = "";
    linkDescription = "";
    await invalidateAll();
  };
</script>

<div
  class="w-[90vw] overflow-hidden mx-auto my-2 border-2 border-lime-300 shadow-xl rounded-xl"
>
  <div class="py-3 px-6">
    <div class="mb-1 py-1 w-full">
      <label for="add-name" class="block mb-2 font-medium text-xl text-gray-900"
        >Link Name:</label
      >
      <input
        type="text"
        id="add-name"
        bind:value={linkName}
        class="block font-medium px-0 w-full text-2xl text-gray-900 border-lime-300 bg-transparent border-b-2 focus:outline-none focus:ring-0 focus:border-lime-600"
        placeholder=""
      />
    </div>
    <div class="mb-1 py-1 w-full">
      <!--Might need validation for URL -->
      <label for="add-url" class="block mb-2 font-medium text-xl text-gray-900"
        >Link URL:</label
      >
      <input
        type="text"
        id="add-url"
        bind:value={linkUrl}
        class="block font-medium px-0 w-full text-2xl text-gray-900 border-lime-300 bg-transparent border-b-2 focus:outline-none focus:ring-0 focus:border-lime-600"
        placeholder=""
      />
    </div>
    <label
      for="add-description"
      class="block mb-2 font-medium text-xl text-gray-900">Description:</label
    >
    <textarea
      bind:value={linkDescription}
      id="add-description"
      class="block w-full h-[100px] py-2 px-2"
    />
  </div>
</div>
<div class="mx-8 justify-end flex">
  <button
    on:click={async () => {
      await submitLink();
    }}
    type="button"
    class="text-white bg-green-600 hover:bg-green-800 font-medium rounded-lg text-sm mx-4 px-5 py-2.5 focus:outline-none"
    >Submit</button
  >
</div>
