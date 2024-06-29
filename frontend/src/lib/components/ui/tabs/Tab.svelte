<script lang="ts">
    import type { Callback } from "$lib/types/Callback";
    export let icon = "";
    export let name = "placeholder";
    export let tabCallback : Callback;
</script>
<li on:click= {() => {tabCallback()}} class="z-30 flex-1 flex-grow text-center">
    <div class="">
        <span class="border-lime-400 border-solid hover:border-b-2 z-30 flex items-center justify-center w-full px-0 py-1 mb-0 transition-all ease-in-out rounded-lg cursor-pointer text-slate-700 bg-inherit hover:text-gray-500"
            data-tab-target="" role="tab" aria-selected="false">
            <svg class="w-6 h-6 text-gray-800 " aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" stroke-linecap="round" stroke-width="2" d={icon}/>
            </svg>
            <span class="ml-1">{name}</span>
        </span>
    </div>
</li>