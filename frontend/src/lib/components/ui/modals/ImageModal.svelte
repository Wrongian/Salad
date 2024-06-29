<script lang="ts">
    import type { ImageSubmitFunction } from "$lib/types/Callback";
    export let isModalShown: boolean = false;
    export let modalText : string = "";
    export let imageSubmitFunction: ImageSubmitFunction;
    let files : FileList;
    const ALLOWEDFILETYPES = ["image/png", "image/jpeg", "image/jpg"]; 
    // placeholder for now
    const submitPicture = async () => {
        if (files && files[0]) {
            let file: File = files[0];
            let mimetype: string = file.type
            let filetypeArray: string[] = mimetype.split("/");
            let filetype :string = filetypeArray[filetypeArray.length - 1];
            if (ALLOWEDFILETYPES.includes(mimetype)) {
                // let arrayBuffer :ArrayBuffer = await files[0].arrayBuffer()
                await imageSubmitFunction(file, filetype);
                location.reload()
            }
        }
    } 

    // todo
    const handleModalFocusLoss = ({ relatedTarget, currentTarget } : any) => {
        // dont remove dropdown if its the parent element 
        if (relatedTarget instanceof HTMLElement && currentTarget.contains(relatedTarget)) 
        {
        return
        }
        isModalShown = false
    }
</script>

<!-- Container -->
<div on:focusout={() => {handleModalFocusLoss}} class="overflow-y-auto overflow-x-hidden z-50 fixed top-0 right-0 left-0 justify-center items-center w-full md:inset-0 h-full flex" tabindex="-1">
    <div  class="relative p-4 w-full max-w-md max-h-full">
        <!-- Modal content -->
        <div  class="relative bg-white rounded-lg shadow">
            <span class="inline-block m-4 text-gray-500 text-medium text-xl">{modalText}</span>
            <button on:click={() => {isModalShown = false}} type="button" class="absolute top-4 end-3 bg-transparent hover:bg-gray-200 hover:text-gray-900 w-8 h-8 inline-flex justify-center items-center">
                <svg class="w-6 h-6 text-gray-800" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18 17.94 6M18 18 6.06 6"/>
                </svg>
            </button>
            <div class="p-1">
                <div class="flex justify-center">
                    <div class="max-w-xl bg-gray-50 rounded-lg shadow-xl">
                        <div class="m-4">
                            <div class="flex items-center justify-center w-full">
                                <label class="flex-col flex w-full h-full border-4 border-lime-400 border-dashed rounded hover:bg-gray-100 hover:border-lime-300">
                                    <div class="flex-col flex pt-7 justify-center items-center">
                                        <svg class="w-6 h-6 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 5v9m-5 0H5a1 1 0 0 0-1 1v4a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1v-4a1 1 0 0 0-1-1h-2M8 9l4-5 4 5m1 8h.01"/>
                                        </svg>
                                        <p class="pt-1 text-sm tracking-wider text-gray-400 group-hover:text-gray-600">
                                            Upload Image</p>
                                    </div>
                                    <input bind:files on:change={async () => {await submitPicture()}} accept="image/png, image/jpeg, image/jpg" type="file" class="opacity-0"  />
                                </label>
                            </div>
                        </div>
                </div>
            </div> 
                <!--WIP Loading Bar--> 
                <!-- <div class="m-8 bg-lime-200 rounded-full h-3 mt-4">
                    <div class="bg-lime-500 h-3 rounded-full" style="width: 30%">
                    </div>
                </div> -->
                <div class="m-8 h-3 mt-4 text-gray-500 font-small">
                    <span>Allowed file types: png, jpeg, jpg</span>
                    </div>
            </div>
        </div>
    </div >
</div> 

