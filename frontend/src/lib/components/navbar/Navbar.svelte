<script lang="ts">
  import { goto, invalidateAll } from "$app/navigation";
  import favicon from "$lib/assets/favicon.ico";
  import { Trash } from "lucide-svelte";
  import {
    completeFollowRequest,
    deleteAllNotifications,
    logout,
    readNotification,
  } from "$lib/scripts/queries";
  import type {
    TNotification,
    TNotificationsPayload,
  } from "$lib/scripts/validation/response";
  import Searchbar from "../search/Searchbar.svelte";
  import DropDownButton from "../ui/dropdown/DropDownButton.svelte";
  import DropDownLink from "../ui/dropdown/DropDownLink.svelte";
  import NavLink from "../ui/navbar/NavLink.svelte";
  import {
    Popover,
    PopoverContent,
    PopoverTrigger,
  } from "$lib/components/ui/popover";
  import { Separator } from "$lib/components/ui/separator";
  import type TReadNotification from "$lib/scripts/query";

  export let isLoggedIn = false;
  export let profileRoute = "";
  export let notifications: TNotificationsPayload = { notifications: [] };
  $: notifs = notifications.notifications;
  $: has_unread =
    notifs.filter((notif) => {
      notif.is_read == false;
    }).length > 0;

  let isDropdownOpen: boolean = false;

  const dropdownToggle = () => {
    isDropdownOpen = !isDropdownOpen;
  };

  const handleDropdownFocusLoss = ({ relatedTarget, currentTarget }: any) => {
    // dont remove dropdown if its the parent element
    if (
      relatedTarget instanceof HTMLElement &&
      currentTarget.contains(relatedTarget)
    ) {
      return;
    }
    isDropdownOpen = false;
  };

  async function handleFollowRequest(
    hasAccepted: boolean,
    fromID: number,
    notification: TNotification,
  ) {
    completeFollowRequest({ accept: hasAccepted, from_id: fromID });
    if (hasAccepted) {
      notification.is_read = true;
    }
    const ele = document.getElementById("notif-" + notification.id);
    ele?.remove();
    // await invalidateAll();
  }

  async function handleReadNotification(notif: TNotification) {
    if (!notif.is_read) {
      let query: TReadNotification = { notification_id: notif.id };
      notif.is_read = false;
      notifs = notifs;
      await invalidateAll();
      readNotification(query);
    }
  }

  async function handleDeleteNotifications() {
    deleteAllNotifications();
    notifs = [];
  }
</script>

<nav class="bg-primary">
  <div class="mx-auto max-w-9xl px-2 sm:px-6 lg:px-8 shadow-lg">
    <div class="relative flex h-10 items-center justify-between">
      <div class="absolute inset-y-0 left-0 flex items-center sm:hidden">
        <!-- Mobile -->
        <button
          type="button"
          class="relative inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white"
          aria-controls="mobile-menu"
          aria-expanded="false"
        >
          <span class="absolute -inset-0.5"></span>
          <span class="sr-only">Open main menu</span>
          <svg
            class="block h-6 w-6"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            aria-hidden="true"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
            />
          </svg>
          <svg
            class="hidden h-6 w-6"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            aria-hidden="true"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      </div>
      <div
        class="z-10 flex flex-1 items-center justify-center sm:items-stretch sm:justify-start"
      >
        <div class="flex flex-shrink-0 items-center">
          <a href="/">
            <img class="h-6 w-auto" src={favicon} alt="app icon" />
          </a>
        </div>
        <div class="hidden sm:ml-6 sm:block">
          <div class="flex space-x-4">
            {#if isLoggedIn}
              <NavLink linkName={`My Profile`} link={profileRoute}></NavLink>
              <NavLink linkName={`Edit Profile`} link={`/edit-profile`}
              ></NavLink>
            {/if}
            <Searchbar />
          </div>
        </div>
      </div>
      {#if !isLoggedIn}
        <div
          class="absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0"
        >
          <div class="hidden sm:ml-6 sm:block">
            <div class="flex space-x-4">
              <NavLink linkName={`Register`} link={`/auth/register`}></NavLink>
              <NavLink linkName={`Login`} link={`/auth/login`}></NavLink>
            </div>
          </div>
        </div>
      {:else}
        <div
          class="absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0"
        >
          <Popover
            ><PopoverTrigger>
              <button
                type="button"
                class="bg-gray-800 relative rounded-full p-1 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800"
              >
                <span class="absolute -inset-1.5"></span>
                <span class="sr-only">View notifications</span>
                <svg
                  class="h-4 w-4"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                  aria-hidden="true"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0"
                  />
                </svg>
              </button>
            </PopoverTrigger>
            <PopoverContent class="w-56 bg-lime-50">
              <div class="flex flex-row space-x-4">
                <div class="">
                  <h1>Notifications</h1>
                </div>
                <div class="">
                  <button
                    on:click={() => {
                      handleDeleteNotifications();
                    }}
                  >
                    <Trash /></button
                  >
                </div>
              </div>
              <Separator />
              <div class="py-4">
                {#each notifs as notif}
                  <div
                    role="listitem"
                    on:mouseenter={() => {
                      handleReadNotification(notif);
                    }}
                    id="notif-{notif.id}"
                    class="border border-3 p-2 {notif.is_read
                      ? 'bg-lime-50'
                      : 'bg-lime-100'}"
                  >
                    {#if notif.notification_type == 1}
                      <div class="max-w-56">{notif.msg}</div>
                    {/if}
                    {#if notif.notification_type == 2}
                      <div class="max-w-56">
                        <div class="flex flex-col space-y-2">
                          <div class="overflow-y-auto">
                            <h3>{notif.msg}</h3>
                          </div>
                          <div
                            class="flex-row flex space-x-1 justify-center space-between"
                          >
                            <button
                              on:click={() =>
                                handleFollowRequest(
                                  true,
                                  notif.trigger_id,
                                  notif,
                                )}
                              class="px-4 py-1 text-white bg-lime-700 hover:bg-lime-800 font-medium rounded-lg text-base"
                              >Accept</button
                            >
                            <button
                              on:click={() =>
                                handleFollowRequest(
                                  false,
                                  notif.trigger_id,
                                  notif,
                                )}
                              class="px-4 py-1 text-white bg-lime-700 hover:bg-lime-800 font-medium rounded-lg text-base"
                              >Reject</button
                            >
                          </div>
                        </div>
                      </div>
                    {/if}
                  </div>
                {/each}
              </div>
            </PopoverContent>
          </Popover>
          <!-- Profile dropdown -->
          <div on:focusout={handleDropdownFocusLoss} class="relative ml-3">
            <div>
              <button
                type="button"
                on:click={dropdownToggle}
                class="relative flex rounded-full bg-white text-sm focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800"
                id="user-menu-button"
                aria-expanded="false"
                aria-haspopup="true"
              >
                <span class="absolute -inset-1.5"></span>
                <span class="sr-only">Open user menu</span>
                <!--Image is a placeholder-->
                <img class="h-6 w-6 rounded-full" src={favicon} alt="" />
              </button>
            </div>

            {#if isDropdownOpen && isLoggedIn}
              <div
                class="absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-lime-50 py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
                role="menu"
                aria-orientation="vertical"
                aria-labelledby="user-menu-button"
                tabindex="-1"
              >
                <DropDownLink linkName="Settings" link="/settings"
                ></DropDownLink>
                <DropDownLink linkName="Insights" link="/insights"
                ></DropDownLink>
                <DropDownButton
                  buttonTitle="Log out"
                  onButtonClick={() =>
                    logout().then((_) =>
                      goto("/auth/login", { invalidateAll: true }),
                    )}
                />
              </div>
            {/if}
          </div>
        </div>
      {/if}
    </div>
  </div>
  <!-- Mobile Navbar-->
  <div class="sm:hidden" id="mobile-menu">
    <div class="space-y-1 px-2 pb-3 pt-2">
      {#if isLoggedIn == true}
        <NavLink linkName={`My Profile`} link={profileRoute}></NavLink>
        <NavLink linkName={`Edit Profile`} link={`/edit-profile`}></NavLink>
      {/if}
      <!--No implementation yet-->
      <NavLink linkName={`Search`} link={`/search/users`}></NavLink>
    </div>
  </div>
</nav>
