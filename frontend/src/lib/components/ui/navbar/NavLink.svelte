<script lang="ts">
    export let linkName : string = "Placeholder";
    export let link : string = "/";
</script>
<a href={link} class="rounded-md bg-lime-100 px-3 py-1 text-sm font-medium text-black hover:bg-lime-200" aria-current="page">{linkName}</a>