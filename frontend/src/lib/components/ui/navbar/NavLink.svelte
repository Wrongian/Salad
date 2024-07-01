<script lang="ts">
  export let linkName: string = "Placeholder";
  export let link: string = "/";
  //data-sveltekit-preload-data="off"
</script>

<a
  href={link}
  class="rounded-md bg-lime-200 px-3 py-1 text-sm font-medium text-black hover:bg-lime-300"
  aria-current="page">{linkName}</a
>
