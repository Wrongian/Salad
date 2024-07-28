<script lang="ts">
  import * as Avatar from "$lib/components/ui/avatar";
  import * as Pagination from "$lib/components/ui/pagination";
  import * as Dialog from "$lib/components/ui/dialog";
  import type { TPaginatedProfile } from "$lib/scripts/validation/response";
  import FollowMenuButton from "./FollowMenuButton.svelte";
  import Button from "$lib/components/ui/button/button.svelte";

  export let paginatedFollowRecords: TPaginatedProfile[];
  export let totalRecords: number;
  export let recordsPerPage: number;
  export let currentPageIndex: number;

  export let removeRecordDialogMessage: string =
    "This action cannot be undone.";
  export let removeRecordMenuButtonLabel: String = "Remove";
  export let removeRecordDialogTitle: string = "Are you sure absolutely sure?";
  export let onConfirmRecordRemove: (
    profile: TPaginatedProfile | undefined,
  ) => void | Promise<void> = (profile) => {};

  let menuDeleteRecord: TPaginatedProfile | undefined;
  let menuDeleteDialogOpen = false;
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

        <FollowMenuButton>
          <button
            slot="menu"
            class="h-10 w-[150px] p-4 text-sm flex items-center justify-center
            hover:bg-primary-500 hover:text-white hover:cursor-pointer
            font-semibold rounded-sm bg-primary"
            on:click={() => {
              menuDeleteDialogOpen = true;
              menuDeleteRecord = paginatedFollowRecords[i];
            }}
            >{removeRecordMenuButtonLabel}
          </button>
        </FollowMenuButton>
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

<Dialog.Root bind:open={menuDeleteDialogOpen}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>{removeRecordDialogTitle}</Dialog.Title>
      <Dialog.Description>
        {removeRecordDialogMessage}
      </Dialog.Description>
    </Dialog.Header>
    <Dialog.Footer>
      <Dialog.Close>
        <Button
          type="submit"
          class="text-black hover:bg-primary-500 hover:text-white font-medium"
          on:click={() => {
            onConfirmRecordRemove(menuDeleteRecord);
            menuDeleteRecord = undefined;
          }}>Confirm</Button
        >
      </Dialog.Close>
      <Dialog.Close>
        <Button
          type="submit"
          on:click={() => (menuDeleteRecord = undefined)}
          class="text-black hover:bg-primary-500 hover:text-white font-medium"
          >Cancel</Button
        >
      </Dialog.Close>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
