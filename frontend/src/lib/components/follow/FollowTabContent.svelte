<script lang="ts">
  import { EllipsisVertical } from "lucide-svelte";
  import * as Avatar from "../ui/avatar";
  import * as Pagination from "../ui/pagination";
  import type { TPaginatedProfile } from "$lib/scripts/validation/response";

  export let paginatedFollowRecords: TPaginatedProfile[];
  export let totalRecords: number;
  export let recordsPerPage: number;
  export let currentPageIndex: number;
</script>

<div class="flex flex-col justify-between h-[60vh]">
  <div class="max-h-[350px] overflow-auto">
    {#each paginatedFollowRecords as { username, img_src, display_name }, i}
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

        <button class="m-0 p-0">
          <EllipsisVertical />
        </button>
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
