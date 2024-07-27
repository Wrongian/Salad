<script lang="ts">
  import * as Tabs from "$lib/components/ui/tabs";
  import { Input } from "$lib/components/ui/input/index.js";
  import {
    getFollowers,
    getFollowings,
    getFollowRequests,
    removeFollower,
    removeFollowing,
  } from "$lib/scripts/queries";
  import type {
    TPaginatedFollowRequestProfile,
    TPaginatedProfile,
  } from "$lib/scripts/validation/response";
  import type { PageData } from "./$types";
  import { twMerge } from "tailwind-merge";
  import FollowTabContent from "$lib/components/follow/FollowTabContent.svelte";
  import PendingTabContent from "$lib/components/follow/PendingTabContent.svelte";
  import { invalidateAll } from "$app/navigation";

  const VIEW_MODES = ["Followers", "Following", "Pending"] as const;
  type TViewMode = (typeof VIEW_MODES)[number];
  const UPDATE_MAPPINGS: Record<TViewMode, Function> = {
    Followers: updateFollowerProfiles,
    Following: updateFollowingProfiles,
    Pending: updatePendingProfiles,
  };

  const PER_PAGE = 8;
  export let data: PageData;
  let viewMode: TViewMode = VIEW_MODES[0];

  let currFollowerPageIndex = 1;
  let currFollowingPageIndex = 1;
  let currPendingPageIndex = 1;

  let searchQuery = "";
  let followers: TPaginatedProfile[] = data.followers;
  let followings: TPaginatedProfile[] = data.followings;
  let pendingRequests: TPaginatedFollowRequestProfile[] = data.pendingRequests;

  let totalFollowers = data.totalFollowers;
  let totalFollowing = data.totalFollowing;
  let totalPending = data.totalPending;

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

  async function updatePendingProfiles() {
    const getFollowRequestResult = await getFollowRequests(
      searchQuery,
      currPendingPageIndex,
    );
    pendingRequests = getFollowRequestResult?.profiles ?? pendingRequests;
    totalPending = getFollowRequestResult?.total_size ?? totalPending;
  }

  function changeViewMode(newView: TViewMode) {
    viewMode = newView;
  }

  function onSearchQueryChange() {
    return UPDATE_MAPPINGS[viewMode]();
  }

  function refreshPendingRequests() {
    pendingRequests = data.pendingRequests;
    totalPending = data.totalPending;
  }

  function refreshFollowerProfiles() {
    followers = data.followers;
    totalFollowers = data.totalFollowers;
  }

  function refreshFollowingProfiles() {
    followings = data.followings;
    totalFollowing = data.totalFollowing;
  }

  function deleteFollowers(profile: TPaginatedProfile | undefined) {
    if (!profile) return;
    removeFollower(profile?.id).then((_) => invalidateAll());
  }

  function deleteFollowings(profile: TPaginatedProfile | undefined) {
    if (!profile) return;
    removeFollowing(profile?.id).then((_) => invalidateAll());
  }

  // listens for load fn reruns and updates data accordingly
  $: data.pendingRequests, refreshPendingRequests();
  $: data.followers, refreshFollowerProfiles();
  $: data.followings, refreshFollowingProfiles();

  // listens for page index changes & updates data accordingly
  $: currFollowerPageIndex, updateFollowerProfiles();
  $: currFollowingPageIndex, updateFollowingProfiles();
  $: searchQuery, onSearchQueryChange();
</script>

<div class="flex justify-center h-fit">
  <Tabs.Root value={viewMode} class="w-[50vw] h-[90vh] mt-2 min-w-60 max-w-90">
    <Tabs.List
      class="h-[50px] grid w-full grid-cols-3 bg-lime-200 rounded-[4px]"
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
      <button
        class={twMerge(
          "w-full h-3/4 rounded-[4px]",
          viewMode === VIEW_MODES[2] && "bg-lime-600 text-white",
        )}
        on:click={async () => {
          await updatePendingProfiles();
          changeViewMode(VIEW_MODES[2]);
        }}
      >
        {VIEW_MODES[2]}
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

    <div class="flex flex-col justify-between">
      <Tabs.Content value={VIEW_MODES[0]}>
        <FollowTabContent
          paginatedFollowRecords={followers}
          totalRecords={totalFollowers}
          recordsPerPage={PER_PAGE}
          removeRecordDialogMessage="This action cannot be undone. Do you want to remove follower?"
          removeRecordMenuButtonLabel="Remove follower"
          onConfirmRecordRemove={deleteFollowers}
          bind:currentPageIndex={currFollowerPageIndex}
        />
      </Tabs.Content>

      <Tabs.Content value={VIEW_MODES[1]}>
        <FollowTabContent
          paginatedFollowRecords={followings}
          totalRecords={totalFollowing}
          recordsPerPage={PER_PAGE}
          removeRecordDialogMessage="This action cannot be undone. Do you want to remove following?"
          removeRecordMenuButtonLabel="Unfollow"
          onConfirmRecordRemove={deleteFollowings}
          bind:currentPageIndex={currFollowingPageIndex}
        />
      </Tabs.Content>

      <Tabs.Content value={VIEW_MODES[2]}>
        <PendingTabContent
          paginatedFollowRecords={pendingRequests}
          totalRecords={totalPending}
          recordsPerPage={PER_PAGE}
          bind:currentPageIndex={currPendingPageIndex}
        />
      </Tabs.Content>
    </div>
  </Tabs.Root>
</div>
