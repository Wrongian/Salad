<script lang="ts">
    import Link from "./Link.svelte";
    import type { ListData } from "$lib/types/Profile";
    import type { TLink } from "$lib/scripts/response-validator";
    import PictureModal from "$lib/components/ui/modals/ImageModal.svelte";
    import { reorderLink } from "$lib/scripts/queries";
    import { updateLinkPicture } from "$lib/scripts/queries";
    export let links : TLink[];
    let list : ListData[] = [];
    let currentlyDraggedIndex:number | null = null;
    let overIndex : number | null = null;
    let isModalShown = false;
    // difference between mouse and mid of screen
    let diff : number = 0;
    const SMOOTH_SCROLL = 0.05

    let modalLinkId: number| null = null;
    
    // todo scrolls to the top everytume elements are inserted

    links.forEach(ele => {
        let listElement : ListData = {
            isDragged : false,
            linkData : ele,
        };
        list.push(listElement);
    });
    const modalCallback = (id: number) => {
        isModalShown = true;
        modalLinkId = id;
    }

    const modalSubmitFunction = async (image: Blob, filetype: string) => {
        if (modalLinkId != null) {
            await updateLinkPicture(image, filetype, modalLinkId)
        }
    }
    const startDrag = (e : DragEvent, listData: ListData, index: number) => {
        // sneaky way of seeing the dragged list
        currentlyDraggedIndex = index
        overIndex = null;
        setTimeout(() => {
            listData.isDragged = true;
            // for object reactivity
            list = list;
        }, 0)
    }

    const endDrag = async (e : DragEvent, listData: ListData, index: number) => {
        listData.isDragged = false;
        // for object reactivity
        if (currentlyDraggedIndex != null && overIndex != null) { 
            if (currentlyDraggedIndex != overIndex) {
                // this is ok since we cap total list elements
                let oldElements = list[currentlyDraggedIndex];
                // delete
                list.splice(currentlyDraggedIndex, 1)
                let addIndex = overIndex;
                list.splice(addIndex, 0, oldElements);
                
                let new_pos : number | null = null;
                // new index 
                if (currentlyDraggedIndex > overIndex) {
                    new_pos = overIndex;
                }
                else if (overIndex == links.length - 1) {
                    new_pos = null;
                }
                else {
                    new_pos = overIndex + 1;
                }

                
                if (new_pos != null) {
                    await reorderLink({
                    link_id : links[currentlyDraggedIndex].id,
                    new_position_id: links[overIndex].id,
                    })
                }
                else {
                    await reorderLink({
                    link_id : links[currentlyDraggedIndex].id,
                    new_position_id: null,
                    })
                }
                
            }
        }
        currentlyDraggedIndex = null;
        overIndex = null;
        list = list
    }

    const dragOver = (e : DragEvent, listData: ListData, index: number) => {
            if (currentlyDraggedIndex != null && currentlyDraggedIndex != index) {
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
        if (currentlyDraggedIndex != null)  {
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
    <div id ="linkdiv-{listData.linkData.id}" on:dragover|preventDefault={(e) => {dragOver(e, listData, index)}} on:dragend={async (e) => {await endDrag(e, listData, index)}} on:dragstart={(e) => {startDrag(e, listData, index)} }>
    <Link modalCallback={modalCallback} listData={listData}> </Link>
    </div>
    {/each}
    </ul>
</div>

{#if isModalShown}
<PictureModal bind:isModalShown imageSubmitFunction={modalSubmitFunction} modalText="Upload Link Picture" > 
</PictureModal>
{/if}