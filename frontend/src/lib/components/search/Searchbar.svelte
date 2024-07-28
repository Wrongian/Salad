<script lang="ts">
  import type { TPaginatedProfile } from "$lib/scripts/validation/response";
  import { Input } from "../ui/input";
  import * as Avatar from "../ui/avatar";
  import { UserSearch } from "lucide-svelte";
  import { goto } from "$app/navigation";
  import { searchUsers } from "$lib/scripts/queries";

  export let searchQuery: string = "";
  let isSearchDropdownOpen = false;

  let results: TPaginatedProfile[] = [];
  const handleDropdownFocusLoss = ({
    relatedTarget,
    currentTarget,
  }: FocusEvent & { currentTarget: HTMLDivElement }) => {
    if (
      relatedTarget instanceof HTMLElement &&
      currentTarget.contains(relatedTarget)
    ) {
      return;
    }
    isSearchDropdownOpen = false;
  };

  async function onSearchQueryChange() {
    results = (await searchUsers(searchQuery, 1))?.profiles ?? [];
  }
</script>

<div class="p-0 w-80 relative" on:focusout={handleDropdownFocusLoss}>
  <div class="flex gap-x-2 w-full">
    <Input
      class="rounded-[0.25rem] h-8 flex-1"
      type="search"
      placeholder="Search users..."
      bind:value={searchQuery}
      on:input={onSearchQueryChange}
      on:focusin={() => {
        isSearchDropdownOpen = true;
      }}
    />

    <button
      class="hover:text-white"
      on:click={() => {
        goto(`/search/users?index=1&query=${searchQuery}`, {
          invalidateAll: true,
        });
      }}
    >
      <UserSearch />
    </button>
  </div>

  {#if isSearchDropdownOpen}
    <div
      class="group w-80 absolute z-3 h-fit top-full translate-y-2 bg-lime-50 shadow-lg ring-1 ring-black ring-opacity-5 divide-y-1"
      role="menu"
    >
      {#each results as { username, img_src, display_name }}
        <div
          class="h-10 p-4 flex gap-x-4 items-center justify-between border border-gray-100 hover:bg-lime-200 hover:cursor-pointer"
          role="menuitem"
        >
          <Avatar.Root class="w-5 h-5 ring-2 ring-lime-300">
            <Avatar.Image src={img_src} alt="" />
            <Avatar.Fallback></Avatar.Fallback>
          </Avatar.Root>
          <a
            href={`../profiles/${username}`}
            data-sveltekit-preload-data="tap"
            class="text-left select-none cursor-pointer flex-1 block"
          >
            {display_name}
          </a>
        </div>
      {/each}
      <a
        href="/search/users"
        role="menuitem"
        on:click={() => {}}
        class="h-10 p-4 flex items-center hover:bg-lime-200">... See more</a
      >
    </div>
  {/if}
</div>
