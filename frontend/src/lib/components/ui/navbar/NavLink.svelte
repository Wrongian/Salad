<script lang="ts">
  export let linkName: string = "Placeholder";
  export let link: string = "/";
  //data-sveltekit-preload-data="off"
</script>

<a
  href={link}
  class="flex items-center rounded-md px-3 py-1 text-sm font-medium hover:bg-primary-500 hover:text-white"
  aria-current="page">{linkName}</a
>
