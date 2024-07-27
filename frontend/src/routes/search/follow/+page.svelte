<script lang="ts">
  import * as Tabs from "$lib/components/ui/tabs";
  import { Input } from "$lib/components/ui/input/index.js";
  import { getFollowers, getFollowings } from "$lib/scripts/queries";
  import type { TPaginatedProfile } from "$lib/scripts/validation/response";
  import type { PageData } from "./$types";
  import { twMerge } from "tailwind-merge";
  import FollowTabContent from "$lib/components/follow/FollowTabContent.svelte";
  const VIEW_MODES = ["Followers", "Following"] as const;
  type ViewModes = (typeof VIEW_MODES)[number];
  const PER_PAGE = 8;
  export let data: PageData;
  let viewMode: ViewModes = VIEW_MODES[0];

  let currFollowerPageIndex = 1;
  let currFollowingPageIndex = 1;

  let searchQuery = "";
  let followers: TPaginatedProfile[] = data.followers;
  let followings: TPaginatedProfile[] = data.followings;

  let totalFollowers = data.totalFollowers;
  let totalFollowing = data.totalFollowing;

  async function updateFollowerProfiles() {
    const getFollowerResult = await getFollowers(
      searchQuery,
      currFollowerPageIndex,
    );
    followers = getFollowerResult?.profiles ?? followers;
    totalFollowers = getFollowerResult?.total_size ?? totalFollowers;
  }

  async function updateFollowingProfiles() {
    const getFollowingResult = await getFollowings(
      searchQuery,
      currFollowingPageIndex,
    );
    followings = getFollowingResult?.profiles ?? followings;
    totalFollowing = getFollowingResult?.total_size ?? totalFollowing;
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

  $: currFollowerPageIndex, onCurrentPageIndexChange();
  $: currFollowingPageIndex, onCurrentPageIndexChange();
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

    <div class="flex flex-col justify-between h-[65vh] relative">
      <Tabs.Content value={VIEW_MODES[0]}>
        <FollowTabContent
          paginatedFollowRecords={followers}
          totalRecords={totalFollowers}
          recordsPerPage={PER_PAGE}
          currentPageIndex={currFollowerPageIndex}
        />
      </Tabs.Content>

      <Tabs.Content value={VIEW_MODES[1]}>
        <FollowTabContent
          paginatedFollowRecords={followings}
          totalRecords={totalFollowing}
          recordsPerPage={PER_PAGE}
          currentPageIndex={currFollowingPageIndex}
        />
      </Tabs.Content>
    </div>
  </Tabs.Root>
</div>
