<script lang="ts">
  import Errors from "$lib/components/Errors.svelte";
  import DropDownLink from "$lib/components/ui/dropdown/DropDownLink.svelte";
  import DropDownLinkNoPreload from "$lib/components/ui/dropdown/DropDownLinkNoPreload.svelte";
  import NavLink from "$lib/components/ui/navbar/NavLink.svelte";
  import "../app.css";
  import { onMount } from "svelte";
  import favicon from "$lib/assets/favicon.ico";
  import type { PageData } from "./$types";
  export let data: PageData;


  

  let doneLoad = false;
  onMount(async () => {
    // TODO: logic to render all the necessary components
    // previous url
        doneLoad = true;
  });
  // let next = "/";
  // afterNavigate(({from}) => {
  //   next = from?.url.pathname || next
  // })
  // dropdown menu
  // default
  let isDropdownOpen : boolean = false;

  const dropdownToggle = () => {
    isDropdownOpen = !isDropdownOpen;
  }

  const handleDropdownFocusLoss = ({ relatedTarget, currentTarget } : any) => {
    // dont remove dropdown if its the parent element 
    if (relatedTarget instanceof HTMLElement && currentTarget.contains(relatedTarget)) 
    {
      return
    }
    isDropdownOpen = false
  }
</script>

<svelte:head>
  <!--placeholder title-->
  <title>Welcome to Saladify!</title>
</svelte:head>
<Errors />
<div class="h-screen">
  <nav class="bg-lime-100">
    <div class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
      <div class="relative flex h-8 items-center justify-between">
        <div class="absolute inset-y-0 left-0 flex items-center sm:hidden">
          <!-- Mobile -->
          <button type="button" class="relative inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white" aria-controls="mobile-menu" aria-expanded="false">
            <span class="absolute -inset-0.5"></span>
            <span class="sr-only">Open main menu</span>
            <svg class="block h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
            </svg>
            <svg class="hidden h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
        <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
          <div class="flex flex-shrink-0 items-center">
            <a href = "/">
            <img class="h-6 w-auto" src={favicon} alt = "app icon">
            </a>
          </div>
          <div class="hidden sm:ml-6 sm:block">
            <div class="flex space-x-4">

              {#if data.isLoggedIn}
              <NavLink linkName={`My Profile`} link={`/profiles`}></NavLink>
              {/if}
              <NavLink linkName={`Search`} link = {`/search`}></NavLink>
            </div>
          </div>
        </div>
        {#if !data.isLoggedIn}
        <div class="absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0">
          <div class="hidden sm:ml-6 sm:block">
            <div class="flex space-x-4">
              <NavLink linkName={`Register`} link={`/auth/register`}></NavLink>
              <NavLink linkName={`Login`} link = {`/auth/login`}></NavLink>
            </div>
          </div>
        </div>
        {:else}
        <!--Notifications todo later-->
        <div class="absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0">
          <button type="button" class="relative rounded-full bg-gray-800 p-1 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800">
            <span class="absolute -inset-1.5"></span>
            <span class="sr-only">View notifications</span>
            <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0" />
            </svg>
          </button>

          <!-- Profile dropdown -->
          <div on:focusout={handleDropdownFocusLoss} class="relative ml-3">
            <div>
              <button type="button" on:click={dropdownToggle} class="relative flex rounded-full bg-white text-sm focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800" id="user-menu-button" aria-expanded="false" aria-haspopup="true">
                <span class="absolute -inset-1.5"></span>
                <span class="sr-only">Open user menu</span>
                <!--Image is a placeholder-->
                <img class="h-6 w-6 rounded-full" src={favicon} alt="">
              </button>
            </div>

            {#if isDropdownOpen && data.isLoggedIn}
            <div class="absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none" role="menu" aria-orientation="vertical" aria-labelledby="user-menu-button" tabindex="-1">
               <DropDownLink linkName = "Settings" link = "/settings"></DropDownLink>
               <DropDownLinkNoPreload linkName = "Logout" link = "/logout"></DropDownLinkNoPreload>
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
        {#if data.isLoggedIn == true}
        <NavLink linkName={`My Profile`} link={`/profiles`}></NavLink>
        {/if}
        <NavLink linkName={`Search`} link = {`/search`}></NavLink>
      </div>
    </div>
  </nav>
  {#if doneLoad}
    <slot />
  {/if}
  <footer class="p-2 bg-primary">
    <p class="text-center text-300 text-primary-500">
      <span> All rights reserved @Saladify. </span>
    </p>
  </footer>
</div>
