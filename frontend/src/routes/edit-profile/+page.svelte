<script lang="ts">
  import Tab from "$lib/components/ui/tabs/Tab.svelte";
  import * as Avatar from "$lib/components/ui/avatar/index.js";
  import PictureModal from "$lib/components/ui/modals/ImageModal.svelte";
  import AddLinksForm from "$lib/components/ui/links/AddLinksForm.svelte";
  import type { PageData } from "./$types";
  import DraggableLinks from "$lib/components/ui/links/DraggableLinks.svelte";
  import { updateTextProfile } from "$lib/scripts/queries";
  import type { TLink } from "$lib/scripts/validation/response";
  import { updateProfilePicture } from "$lib/scripts/queries";
  import { invalidateAll } from "$app/navigation";
  export let data: PageData;
  export let links: TLink[] = data.links;
  $: links = data.links;

  let displayNameData = data.display_name || "";
  let bioData = data.bio || "";
  let imageURL = data.picture ?? "";
  let tabSelector: number = 1;
  // modal
  let isModalShown = false;

  // placeholder for now
  const submitDisplayName = () => {
    updateTextProfile({ display_name: displayNameData });
  };
  // placeholder for now
  const submitBio = () => {
    updateTextProfile({ bio: bioData });
  };

  const updateProfileImage = async (image: Blob, filetype: string) => {
    const payload = await updateProfilePicture(image, filetype);
    // skip if updating profile image failed
    if (!payload) {
      return;
    }
    imageURL = payload.href;
  };
</script>

<!--Icons from https://flowbite.com/icons/ -->

<!--todo change brightness when modal is focused-->
<div class="bg-lime-50 px-6">
  <div
    class="font-medium text-2xl px-4 py-3 relative flex justify-start border-lime-100 border-b-2 border-solid"
  >
    <h1>Edit Your Profile</h1>
  </div>
  <div class="border-lime-100 border-b-2 border-solid">
    <div class="relative right-0 w-1/4">
      <ul
        class="relative flex flex-wrap justify-start p-1 list-none rounded-xl"
        data-tabs="tabs"
        role="list"
      >
        <Tab
          tabCallback={() => {
            tabSelector = 1;
          }}
          icon="M12 4a4 4 0 1 0 0 8 4 4 0 0 0 0-8Zm-2 9a4 4 0 0 0-4 4v1a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2v-1a4 4 0 0 0-4-4h-4Z"
          name="Profile"
        ></Tab>
        <Tab
          tabCallback={() => {
            tabSelector = 2;
          }}
          icon="M9 8h10M9 12h10M9 16h10M4.99 8H5m-.02 4h.01m0 4H5"
          name="Links"
        ></Tab>
        <Tab
          tabCallback={() => {
            tabSelector = 3;
          }}
          icon="M5 12h14m-7 7V5"
          name="Add Links"
        ></Tab>
      </ul>
    </div>
  </div>

  {#if tabSelector == 1}
    <!--profile-->
    <!--Outer box for the content-->
    <div
      class="flex justify-start py-4 px-4 rounded-md border-lime-100 border-b-2 border-solid"
    >
      <!--Avatar Div-->
      <div class="px-4 pt-2 relative">
        <button
          on:click={() => {
            isModalShown = true;
          }}
        >
          <Avatar.Root class="w-[100px] h-[100px] ring-2 ring-lime-300 ">
            <Avatar.Image
              class="z-4 hover:brightness-50 peer-hover/image"
              src={imageURL}
              alt=""
            ></Avatar.Image>
            <Avatar.Fallback></Avatar.Fallback>
          </Avatar.Root>
        </button>
        <!--Wip text-->
        <!-- <span class="z-0 select-none invisible peer-hover:visible absolute mx-12 my-6 left-0 w-1/4 text-wrap text-center text-m text-medium text-white">
          Edit Image
        </span> -->
      </div>
      <div class="justify-start my-auto">
        <!--Display name form-->
        <div class="mb-2 py-4">
          <label
            for="change-name"
            class="block mb-2 font-medium text-xl text-gray-900"
            >Change your Name:</label
          >
          <input
            on:focusout={() => {
              submitDisplayName();
            }}
            type="text"
            id="change-name"
            bind:value={displayNameData}
            class="block font-medium px-0 w-full text-2xl text-gray-900 border-lime-300 bg-transparent border-b-2 focus:outline-none focus:ring-0 focus:border-lime-600"
            placeholder=""
          />
        </div>
        <div
          on:focusout={() => {
            submitBio();
          }}
          class="mb-2 py-4"
        >
          <!--Bio Form-->
          <label
            for="change-bio"
            class="block mb-2 font-medium text-xl text-gray-900"
            >Change your Bio:</label
          >
          <textarea
            bind:value={bioData}
            id="change-bio"
            rows="6"
            class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-lime-300 focus:ring-lime-600 outline-lime-600 focus:border-lime-600 border-solid"
            placeholder=""
          ></textarea>
        </div>
      </div>
    </div>
  {/if}

  {#if tabSelector == 2}
    <DraggableLinks {links}></DraggableLinks>
  {/if}

  {#if tabSelector == 3}
    <AddLinksForm></AddLinksForm>
  {/if}
</div>
{#if isModalShown}
  <PictureModal
    imageSubmitFunction={updateProfileImage}
    modalText="Upload Profile Picture"
    bind:isModalShown
  ></PictureModal>
{/if}
