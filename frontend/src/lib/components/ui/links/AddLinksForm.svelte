<script lang="ts">
    import PictureModal from "$lib/components/ui/modals/ImageModal.svelte";
    import * as Avatar from "$lib/components/ui/avatar/index.js";

    let isModalShown = false;
    let linkName = "";
    let linkUrl = "";
    let linkDescription = "";

    const submitLink = () => {

    }
</script>

<div class="w-[1350px] rounded overflow-hidden mx-auto my-2 border-solid border-4 border-lime-300">
    <div class="flex shadow-lg p-3">
        <div class="lg:w-4/5 md:w-3/4">
            <div class="mb-1 py-1 w-[1300px]">
                <label for="add-name" class="block mb-2 font-medium text-xl text-gray-900">Link Name:</label>
                <input type="text" id="add-name" bind:value={linkName} class="block font-medium px-0 w-full text-2xl text-gray-900 border-lime-300 bg-transparent border-b-2 focus:outline-none focus:ring-0 focus:border-lime-600" placeholder="" />
            </div>
            <div class="mb-1 py-1 w-[1300px]">
                <!--Might need validation for URL -->
                <label for="add-url" class="block mb-2 font-medium text-xl text-gray-900">Link URL:</label>
                <input type="text" id="add-url" bind:value={linkUrl} class="block font-medium px-0 w-full text-2xl text-gray-900 border-lime-300 bg-transparent border-b-2 focus:outline-none focus:ring-0 focus:border-lime-600" placeholder="" />
            </div>
            <label for="add-description" class="block mb-2 font-medium text-xl text-gray-900">Description:</label>
            <textarea bind:value={linkDescription} id="add-description" class="block w-[1300px] h-[100px] py-2 px-2 "/>
          </div>
      </div>
    </div> 
<div class = "mx-8 justify-end flex">
<button on:click={() => {submitLink()}} type="button" class="text-white bg-green-600 hover:bg-green-800 font-medium rounded-lg text-sm px-5 py-2.5 focus:outline-none ">Submit</button>
</div>
{#if isModalShown}
<PictureModal modalText="Upload Link Picture" bind:isModalShown> 
</PictureModal>
{/if}