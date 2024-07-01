<script lang="ts">
  import type { ListData } from "$lib/types/Profile";
  import type { TLink } from "$lib/scripts/response-validator";
  import * as Avatar from "$lib/components/ui/avatar/index.js";
  import type { ModalCallback } from "$lib/types/Callback";
  import {
    updateLinkBio,
    updateLinkHref,
    updateLinkTitle,
    deleteLink,
  } from "$lib/scripts/queries";
  import { invalidateAll } from "$app/navigation";
  import { Trash2 } from "lucide-svelte";
  import type { Callback } from "$lib/types/Callback";
  export let modalCallback: ModalCallback;

  export let listData: ListData;
  // callback to delete link element
  // replace name later maybe call it link callback
  export let deleteCallback: ModalCallback;

  // the index of the list
  export let index: number;
  let link: TLink;
  let isFocused = false;

  $: link = listData.linkData;
  $: dragClass = listData.isDragged ? "opacity-0" : "";
  const submitLinkName = async (id: number) => {
    isFocused = false;
    await updateLinkTitle(
      {
        title: link.title ?? "",
      },
      id,
    );
  };

  const submitDescription = async (id: number) => {
    isFocused = false;
    await updateLinkBio(
      {
        bio: link.description ?? "",
      },
      id,
    );
  };

  const submitURL = async (id: number) => {
    isFocused = false;
    await updateLinkHref(
      {
        href: link.href ?? "",
      },
      id,
    );
  };

  const deleteLinkCallback = async (id: number) => {
    deleteCallback(index);
    await deleteLink(id);
  };
</script>

<div
  draggable={!isFocused}
  class="w-[90vw] overflow-hidden border-2 shadow-xl rounded-xl border-lime-300 {dragClass} mb-4 py-2"
>
  <div class="relative flex p-3">
    <button
      class="absolute button top-0 right-0 w-[25px] h-[25px] mr-2 mt-2 hover:opacity-100 opacity-40"
      on:click={() => deleteLinkCallback(link.id)}
    >
      <Trash2 />
    </button>
    <div class="ps-1 w-[100px] m-auto">
      <button
        on:click={async () => {
          await modalCallback(link.id);
          await invalidateAll();
        }}
      >
        <Avatar.Root class="w-[100px] h-[100px] ring-2 ring-lime-300 ">
          <Avatar.Image
            class="z-4 hover:brightness-50 peer-hover/image"
            src={link.img_src}
            alt=""
          ></Avatar.Image>
          <Avatar.Fallback></Avatar.Fallback>
        </Avatar.Root>
      </button>
    </div>
    <div class="w-full px-6 min-w-[250px]">
      <div class="mb-1 py-1 w-full">
        <label
          for="change-name-{link.id}"
          class="block mb-2 font-medium text-xl text-gray-900"
          >Change Link Name:</label
        >
        <input
          on:focus={() => {
            isFocused = true;
          }}
          on:focusout={() => {
            submitLinkName(link.id);
          }}
          type="text"
          id="change-name-{link.id}"
          bind:value={link.title}
          class="block font-medium px-0 w-full text-2xl text-gray-900 border-lime-300 bg-transparent border-b-2 focus:outline-none focus:ring-0 focus:border-lime-600"
          placeholder=""
        />
      </div>

      <div class="mb-1 py-1 w-full">
        <label
          for="change-url-{link.id}"
          class="block mb-2 font-medium text-xl text-gray-900"
          >Change Link URL:</label
        >
        <input
          on:focus={() => {
            isFocused = true;
          }}
          on:focusout={() => {
            submitURL(link.id);
          }}
          type="text"
          id="change-url-{link.id}"
          bind:value={link.href}
          class="block font-medium px-0 w-full text-2xl text-gray-900 border-lime-300 bg-transparent border-b-2 focus:outline-none focus:ring-0 focus:border-lime-600"
          placeholder=""
        />
      </div>
      <div class="w-full">
        <label
          for="change-description-{link.id}"
          class="block mb-2 font-medium text-xl text-gray-900"
          >Change Description:</label
        >
        <textarea
          on:focus={() => {
            isFocused = true;
          }}
          on:focusout={() => {
            submitDescription(link.id);
          }}
          bind:value={link.description}
          id="change-description-{link.id}"
          class="block rounded-lg ring-2 ring-lime-300 w-full h-[100px] py-2 px-2"
        />
      </div>
    </div>
  </div>
</div>
