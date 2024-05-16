<script lang="ts">
    import { onMount } from 'svelte';

    let doneLoad = false;
    onMount(async () => {
        // TODO: logic to render all the necessary components

        doneLoad = true;
    })

</script>

<svelte:head>
    <title>Welcome to Salad!</title>
</svelte:head>

{#if doneLoad}
    <slot/>
{/if}