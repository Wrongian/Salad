<script lang="ts">
  import type { PageData } from "./$types";
  import * as Avatar from "$lib/components/ui/avatar/index.js";
  import * as Card from "$lib/components/ui/card";
  import { UserPlus, UserMinus, X } from "lucide-svelte";
  import {
    createFollowRequest,
    removeFollowing,
    removeFollowRequest,
  } from "$lib/scripts/queries";
  import { addError } from "$lib/modules/Errors.svelte";
  import { invalidateAll } from "$app/navigation";
  export let data: PageData;
  $: links = data.links ?? [];
  $: isOwner = data.is_owner ?? false;
  $: followStatus = data.followStatus ?? "none";
  $: userId = data.id ?? NaN;

  async function followUser() {
    if (Number.isNaN(userId)) {
      addError("Error in following user. Please try again later.");
      return;
    }
    await createFollowRequest({ pending_follow_id: userId });
    await invalidateAll();
  }

  async function unfollowUser() {
    if (Number.isNaN(userId)) {
      addError("Error in unfollowing user. Please try again later.");
      return;
    }

    await removeFollowing(userId);
    await invalidateAll();
  }

  async function cancelFollowRequest() {
    if (Number.isNaN(userId)) {
      addError("Error in unfollowing user. Please try again later.");
      return;
    }

    await removeFollowRequest(userId);
    await invalidateAll();
  }
</script>

<div class="p-2 flex flex-col">
  <main class="flex-1">
    <div class="flex space-y-5 px-2 space-x-2">
      <Avatar.Root class="w-[150px] h-[150px] ring-2">
        <Avatar.Image src={data.picture} alt="" />
        <Avatar.Fallback></Avatar.Fallback>
      </Avatar.Root>
      <div class="pl-2">
        <p class="px-4 py-3 h-[50px] rounded-xl shadow-lg w-[350px] border">
          {data.display_name}
        </p>
        <!-- follower/following component -->
        <div class="flex space-x-6 p-2 pt-4">
          <p>followers: {data.followers}</p>
          <p>following: {data.following}</p>
        </div>
        {#if !isOwner && followStatus === "none"}
          <button
            class="flex gap-x-2 hover:bg-lime-500 hover:text-white rounded-xl bg-green p-2 shadow-md ring-1 ring-lime-500"
            on:click={followUser}
          >
            <UserPlus />
            <p>Follow</p>
          </button>
        {:else if !isOwner && followStatus === "pending"}
          <div class="flex gap-x-2">
            <button
              class="flex hover:bg-lime-500 hover:text-white px-2 py-2 rounded-xl shadow-md ring-1 ring-lime-500"
              on:click={cancelFollowRequest}
            >
              <X />
              <p>Cancel</p>
            </button>
            <p class="py-2 font-semibold">Request sent</p>
          </div>
        {:else if !isOwner && followStatus === "following"}
          <button
            class="flex gap-x-2 hover:bg-lime-500 hover:text-white rounded-xl bg-green p-2 shadow-md ring-1 ring-lime-500"
            on:click={unfollowUser}
          >
            <UserMinus />
            <p>Unfollow</p>
          </button>
        {/if}
      </div>
    </div>
    <div>
      <p class="text-neutral-500 pl-2">bio:</p>
      <div
        class="w-[750px] min-h-[75px] h-[100px] max-h-[250px]
        overflow-auto border rounded-xl border-primary
        p-2"
      >
        <p>{data.bio}</p>
      </div>
    </div>
  </main>
  <article class="overflow-y-auto max-h-[50vh]">
    <div class="flex-1 flex-col space-y-4 pt-4">
      {#each links as link}
        <Card.Root class="h-[150px] rounded-xl">
          <Card.Header>
            <div class="flex space-x-4">
              <div>
                <Avatar.Root class="w-[50px] h-[50px] ring-2">
                  <Avatar.Image src={link.img_src} alt="" />
                  <Avatar.Fallback></Avatar.Fallback>
                </Avatar.Root>
              </div>

              <div class="flex-1">
                <a
                  href={"//" + link.href}
                  data-sveltekit-preload-data="tap"
                  class="font-semibold">{link.title}</a
                >
                <Card.Description class="overflow-y-auto line-clamp-2"
                  >{link.description}</Card.Description
                >
              </div>
            </div>
          </Card.Header>
        </Card.Root>
      {/each}
    </div>
  </article>
</div>
