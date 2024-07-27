<script lang="ts">
  import * as Avatar from "../ui/avatar";
  import * as Pagination from "../ui/pagination";
  import { X, Check } from "lucide-svelte";
  import {
    FOLLOW_REQUEST_TYPES,
    type TFollowRequest,
    type TPaginatedFollowRequestProfile,
  } from "$lib/scripts/validation/response";
  import {
    completeFollowRequest,
    removeFollowRequest,
  } from "$lib/scripts/queries";
  import { invalidateAll } from "$app/navigation";

  export let paginatedFollowRecords: TPaginatedFollowRequestProfile[];
  export let totalRecords: number;
  export let recordsPerPage: number;
  export let currentPageIndex: number;

  async function handleFollowRequest(
    requestType: TFollowRequest,
    accept: boolean,
    userId: number,
  ) {
    if (requestType === "INCOMING") {
      await completeFollowRequest({ accept, from_id: userId });
    } else {
      await removeFollowRequest(userId);
    }
    await invalidateAll();
  }
</script>

<div class="flex flex-col justify-between h-[60vh]">
  <div class="max-h-[350px] overflow-auto">
    {#each paginatedFollowRecords as { username, img_src, display_name, request_type, id }, i}
      <div
        class="h-auto p-2 flex gap-x-4 items-center justify-between shadow-sm rounded-xl border border-gray-100 mb-2 hover:shadow-lg hover:font-semibold"
      >
        <Avatar.Root class="w-10 h-10 ring-2 ring-lime-300">
          <Avatar.Image src={img_src} alt="" />
          <Avatar.Fallback></Avatar.Fallback>
        </Avatar.Root>
        <div class="flex-1">
          <a
            href={`../profiles/${username}`}
            data-sveltekit-preload-data="tap"
            class="text-left select-none cursor-pointer"
          >
            {display_name}
          </a>
        </div>

        <div class="flex gap-x-2">
          <p>{request_type.toLowerCase()} request</p>
          {#if request_type === "INCOMING"}
            <button
              class="m-0 p-0 rounded-full bg-lime-600 hover:bg-lime-500 hover:text-white"
              on:click={async () => {
                await handleFollowRequest(request_type, true, id);
              }}
            >
              <Check />
            </button>
          {/if}
          <button
            class="m-0 p-0 rounded-full bg-lime-600 hover:bg-lime-500 hover:text-white"
            on:click={async () => {
              await handleFollowRequest(request_type, false, id);
            }}
          >
            <X />
          </button>
        </div>
      </div>
    {/each}
  </div>
  <div class="flex justify-center w-full">
    <!-- count musts be >= 1 otherwise shadcn pagination breaks :(-->
    <Pagination.Root
      count={Math.max(totalRecords, 1)}
      perPage={recordsPerPage}
      bind:page={currentPageIndex}
      let:pages
      let:currentPage
    >
      <Pagination.Content>
        <Pagination.Item>
          <Pagination.PrevButton />
        </Pagination.Item>
        {#each pages as page (page.key)}
          {#if page.type === "ellipsis"}
            <Pagination.Item>
              <Pagination.Ellipsis />
            </Pagination.Item>
          {:else}
            <Pagination.Item>
              <Pagination.Link {page} isActive={currentPage == page.value}>
                {page.value}
              </Pagination.Link>
            </Pagination.Item>
          {/if}
        {/each}
        <Pagination.Item>
          <Pagination.NextButton />
        </Pagination.Item>
      </Pagination.Content>
    </Pagination.Root>
  </div>
</div>
