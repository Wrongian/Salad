<script>
  import { goto } from "$app/navigation";
  import * as Avatar from "$lib/components/ui/avatar";
  import * as Pagination from "$lib/components/ui/pagination";

  const PER_PAGE = 8; //backend paginates with PER_PAGE=8

  export let data;
  let users = data.users;
  let currIndex = data.pageIndex;
  let searchQuery = data.searchQuery;
  let totalSize = data.totalSize;

  const refreshPageIndex = () => {
    currIndex = data.pageIndex;
  };
  const refreshUsers = () => {
    users = data.users.slice(0, Math.min(PER_PAGE, data.users.length));
  };

  const refreshSearchQuery = () => {
    searchQuery = data.searchQuery;
  };

  const refreshTotalSize = () => {
    totalSize = data.totalSize;
  };

  // implements refresh logic for when a page fetch has been made
  $: data.users, refreshUsers();
  $: data.pageIndex, refreshPageIndex();
  $: data.searchQuery, refreshSearchQuery();
  $: data.totalSize, refreshTotalSize();
  // on page change, query the next page
  $: currIndex, goto(`/search/users?index=${currIndex}&query=${searchQuery}`);
</script>

<div class="flex flex-col justify-between h-[90vh]">
  <div>
    {#each users as { username, img_src, display_name }, i}
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
      </div>
    {/each}
  </div>
  <Pagination.Root
    count={totalSize}
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
