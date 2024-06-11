<script lang="ts">
  import type { PageData } from "../$types";
  import * as Avatar from "$lib/components/ui/avatar/index.js";
  export let data: PageData;
  $: image_data = data.picture;
  const links = [
    {
      link_id: 0,
      title: "this is a link",
      description: "this is a link description",
      picture: "this is a link picture",
    },
  ];
</script>

<div class="flex flex-col">
  <main class="flex-1">
    <div class="flex space-y-5 px-2 space-x-2">
      <Avatar.Root class="w-[150px] h-[150px] ring-2">
        <!-- TODO: use CDN hosted link instead of b64 string -->
        <Avatar.Image src={`data:image/png;base64,${image_data}`} alt="" />
        <Avatar.Fallback></Avatar.Fallback>
      </Avatar.Root>
      <p class="px-4 py-3 h-[50px] rounded-xl shadow-lg w-[350px] border">
        {data.display_name}
      </p>
    </div>
    <div>
      <p class="text-neutral-500 pl-2">bio:</p>
      <div
        class="w-[500px] min-h-[150px] h-[250px] max-h-[250px]
        overflow-auto border rounded-xl border-primary
        p-2"
      >
        <p>{data.bio}</p>
      </div>
    </div>
  </main>
  <div></div>
</div>
