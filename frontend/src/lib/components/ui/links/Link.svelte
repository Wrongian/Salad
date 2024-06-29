<script lang="ts">
    import type { ListData } from "$lib/types/Profile";
    import type { TLink } from "$lib/scripts/response-validator";
    import * as Avatar from "$lib/components/ui/avatar/index.js";
    import type { Callback } from "$lib/types/Callback";
    export let modalCallback : Callback;
    export let listData : ListData;
    let link: TLink;
    let isFocused = false;
    $: link = listData.linkData;
    $: dragClass = listData.isDragged ? "opacity-0" : "";

    // placeholder for now
    const submitLinkName = (id : number) => {
      isFocused = false;
    } 
    // placeholder for now
    const submitDescription = (id : number) => {
      isFocused = false;
    }
    // placeholder for now
    const submitURL = (id : number) => {
      isFocused = false;
    }
</script>

  <div draggable={!isFocused} class="w-[1350px] rounded overflow-hidden mx-auto my-2 border-solid border-4 border-lime-300">
    <div class="flex shadow-lg p-3">
        <div class="flex flex-col items-center min-w-[100px] justify-center mr-4">
            <div class="px-4 pt-2 relative">
        <button  on:click={() => {modalCallback()}} >
        <Avatar.Root class="w-[100px] h-[100px] ring-2 ring-lime-300 ">
            <Avatar.Image class="z-4 hover:brightness-50 peer-hover/image" src={link.href} alt="">
            </Avatar.Image>
            <Avatar.Fallback></Avatar.Fallback>
        </Avatar.Root>
        </button> 
            <!-- <span class="z-0 select-none invisible peer-hover:visible absolute mx-12 my-6 left-0 w-1/4 text-wrap text-center text-m text-medium text-white">
            Edit Image
            </span> -->
         </div>
            <div class="text-center flex gap-1 flex-wrap justify-center">
            </div>
        </div>
        <div class="lg:w-4/5 md:w-3/4">
            <div class="mb-1 py-1 w-[1100px]">
                <label for="change-name-{link.id}" class="block mb-2 font-medium text-xl text-gray-900">Change Link Name:</label>
                <input on:focus={() => {isFocused = true}} on:focusout={() => {submitLinkName(link.id)}} type="text" id="change-name-{link.id}" bind:value={link.title} class="block font-medium px-0 w-full text-2xl text-gray-900 border-lime-300 bg-transparent border-b-2 focus:outline-none focus:ring-0 focus:border-lime-600" placeholder="" />
            </div>
            <div class="mb-1 py-1 w-[1100px]">
                <!--Might need validation for URL -->
                <label for="change-url-{link.id}" class="block mb-2 font-medium text-xl text-gray-900">Change Link URL:</label>
                <input on:focus={() => {isFocused = true}} on:focusout={() => {submitURL(link.id)}} type="text" id="change-url-{link.id}" bind:value={link.href} class="block font-medium px-0 w-full text-2xl text-gray-900 border-lime-300 bg-transparent border-b-2 focus:outline-none focus:ring-0 focus:border-lime-600" placeholder="" />
            </div>
            <label for="change-description-{link.id}" class="block mb-2 font-medium text-xl text-gray-900">Change Description:</label>
            <textarea on:focus={() => {isFocused = true}} on:focusout={() => {submitDescription(link.id)}} bind:value={link.description} id="change-description-{link.id}" class="block w-[1100px] h-[100px] py-2 px-2 "/>
          </div>
    </div>
</div>