<script lang="ts">
    import type { LinkData } from "$lib/types/Profile";
    import Link from "./Link.svelte";
    import type { ListData } from "$lib/types/Profile";
    import type { TLink } from "$lib/scripts/response-validator";
    import PictureModal from "$lib/components/ui/modals/ImageModal.svelte";
    export let links : TLink[];
    let list : ListData[] = [];
    let currentlyDragged:number | null = null;
    let overIndex : number | null = null;
    let isModalShown = false;
    // difference between mouse and mid of screen
    let diff : number = 0;
    const SMOOTH_SCROLL = 0.05
    
    // todo scrolls to the top everytume elements are inserted

    links.forEach(ele => {
        let listElement : ListData = {
            isDragged : false,
            linkData : ele,
        };
        list.push(listElement);
    });
    const modalCallback = () => {
        isModalShown = true;
    }
    const startDrag = (e : DragEvent, listData: ListData, index: number) => {
        // sneaky way of seeing the dragged list
        currentlyDragged = index
        overIndex = null;
        setTimeout(() => {
            listData.isDragged = true;
            // for object reactivity
            list = list;
        }, 0)
    }

    const endDrag = (e : DragEvent, listData: ListData, index: number) => {
        listData.isDragged = false;
        // for object reactivity
        if (currentlyDragged != null && overIndex != null) { 
            if (currentlyDragged != overIndex) {
                // switch
                // this is ok since we cap total list elements
                let oldEle = list[currentlyDragged];
                // delete
                list.splice(currentlyDragged, 1)
                let addIndex = overIndex;
                // offset
                // if (addIndex > currentlyDragged){
                //     addIndex -= 1;
                // }
                list.splice(addIndex, 0, oldEle);

                // offset currentlyDragged
                // if (index > currentlyDragged) {
                    
                // }


                // update
            }
        }
        currentlyDragged = null;
        overIndex = null;
        list = list
    }

    const dragOver = (e : DragEvent, listData: ListData, index: number) => {
            if (currentlyDragged != null && currentlyDragged != index) {
                overIndex = index    
            }
            
          
    }

    const mouseMove = (e : MouseEvent) => {
        // auto scrolling
        // possible improvement is to add a threshold for dragging so low thresholds result in the screen not scrolling
        // const element = document.getElementById("linkdiv-" + index)
        let middleY = window.innerHeight/2;
        let mouseY = e.screenY;
        diff = mouseY - middleY;
    }

    // mouse move is disabled when dragging
    const onDrag = (e: DragEvent) => {
        let middleY = window.innerHeight/2;
        let mouseY = e.screenY;
        diff = mouseY - middleY;

    }


    window.setInterval(() => {
        // if something is being dragged
        if (currentlyDragged != null)  {
            window.scrollBy(0,diff * SMOOTH_SCROLL);
        }
    },10)
    

</script>
<div on:drag={(e) => {onDrag(e)}} on:mousemove={(e) => mouseMove(e)} class = "p-6">
    <!-- <Links></Links> -->
    <ul  class="max-w-md ">
    
    {#each list as listData, index (listData)}
    <!-- <EditableLink 
    title={link.title} bio={link.bio} imageLink={link.imageLink}></EditableLink> -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div id ="linkdiv-{listData.linkData.id}" on:dragover|preventDefault={(e) => {dragOver(e, listData, index)}} on:dragend={(e) => {endDrag(e, listData, index)}} on:dragstart={(e) => {startDrag(e, listData, index)} }>
    <Link modalCallback={modalCallback} listData={listData}> </Link>
    </div>
    {/each}
    </ul>
</div>

{#if isModalShown}
<PictureModal modalText="Upload Link Picture" bind:isModalShown> 
</PictureModal>
{/if}