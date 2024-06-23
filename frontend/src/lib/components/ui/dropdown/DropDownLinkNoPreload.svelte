<script lang="ts">
    export let linkName : string = "Placeholder";
    export let link : string = "/";
</script>

<a href={link} data-sveltekit-preload-data="tap" class="block px-4 py-2 text-sm text-gray-700" role="menuitem" tabindex="-1" id="user-menu-item-0">{linkName}</a>