<script lang="ts">
  import * as Tabs from "$lib/components/ui/tabs";
  import * as Avatar from "$lib/components/ui/avatar";
  import * as Pagination from "$lib/components/ui/pagination";
  import { Input } from "$lib/components/ui/input/index.js";
  import { getFollowers, getFollowings } from "$lib/scripts/queries";
  import type { TPaginatedProfile } from "$lib/scripts/validation/response";
  import { EllipsisVertical } from "lucide-svelte";
  import type { PageData } from "./$types";
  import { twMerge } from "tailwind-merge";
  const VIEW_MODES = ["Followers", "Following"] as const;
  type ViewModes = (typeof VIEW_MODES)[number];
  const PER_PAGE = 8;
  export let data: PageData;
  let viewMode: ViewModes = VIEW_MODES[0];
  let currIndex = 1;
  let searchQuery = "";
  let followers: TPaginatedProfile[] = data.followers;
  let followings: TPaginatedProfile[] = data.followings;

  async function updateFollowerProfiles() {
    followers =
      (await getFollowers(searchQuery, currIndex))?.profiles ?? followers;
  }

  async function updateFollowingProfiles() {
    followings =
      (await getFollowings(searchQuery, currIndex))?.profiles ?? followings;
  }

  function changeViewMode(newView: ViewModes) {
    viewMode = newView;
  }

  function onCurrentPageIndexChange() {
    viewMode === VIEW_MODES[0]
      ? updateFollowerProfiles()
      : updateFollowingProfiles();
  }

  function onSearchQueryChange() {
    viewMode === VIEW_MODES[0]
      ? updateFollowerProfiles()
      : updateFollowingProfiles();
  }

  $: currIndex, onCurrentPageIndexChange();
  $: searchQuery, onSearchQueryChange();
</script>

<div class="flex justify-center">
  <Tabs.Root value={viewMode} class="w-[50vw] h-[90vh] mt-2 min-w-60 max-w-90">
    <Tabs.List
      class="h-[50px] grid w-full grid-cols-2 bg-lime-200 rounded-[4px]"
    >
      <button
        class={twMerge(
          "w-full h-3/4 rounded-[4px]",
          viewMode === VIEW_MODES[0] && "bg-lime-600 text-white",
        )}
        on:click={async () => {
          await updateFollowerProfiles();
          changeViewMode(VIEW_MODES[0]);
        }}
      >
        {VIEW_MODES[0]}
      </button>

      <button
        class={twMerge(
          "w-full h-3/4 rounded-[4px]",
          viewMode === VIEW_MODES[1] && "bg-lime-600 text-white",
        )}
        on:click={async () => {
          await updateFollowingProfiles();
          changeViewMode(VIEW_MODES[1]);
        }}
      >
        {VIEW_MODES[1]}
      </button>
    </Tabs.List>

    <div class="flex justify-center">
      <Input
        class="rounded-[0.25rem] mt-2 w-4/5"
        type="search"
        placeholder={`Search ${viewMode}...`}
        bind:value={searchQuery}
      />
    </div>

    <div class="flex flex-col justify-between h-[65vh]">
      <Tabs.Content value={VIEW_MODES[0]}>
        <div>
          {#each followers as { username, img_src, display_name }, i}
            <div
              class="h-20 p-8 flex gap-x-4 items-center justify-between shadow-sm rounded-xl border border-gray-100 mb-2"
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
      </Tabs.Content>

      <Tabs.Content value={VIEW_MODES[1]}>
        {#each followings as { username, img_src, display_name }, i}
          <div
            class="h-20 p-8 flex gap-x-4 items-center justify-between shadow-sm rounded-xl border border-gray-100 mb-2"
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
      </Tabs.Content>
    </div>
    <div>
      <Pagination.Root
        count={100}
        perPage={PER_PAGE}
        bind:page={currIndex}
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
  </Tabs.Root>
</div>
