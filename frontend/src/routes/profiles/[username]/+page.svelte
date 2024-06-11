<script lang="ts">
  import type { PageData } from "../$types";
  import * as Avatar from "$lib/components/ui/avatar/index.js";
  import * as Card from "$lib/components/ui/card";
  export let data: PageData;
  $: image_data = data.picture ?? "";
  $: links = data.links ?? [];
</script>

<div class="p-2 flex flex-col">
  <main class="flex-1">
    <div class="flex space-y-5 px-2 space-x-2">
      <Avatar.Root class="w-[150px] h-[150px] ring-2">
        <!-- TODO: use CDN hosted link instead of b64 string -->
        <Avatar.Image src={`data:image/png;base64,${image_data}`} alt="" />
        <Avatar.Fallback></Avatar.Fallback>
      </Avatar.Root>
      <div class="pl-2">
        <p class="px-4 py-3 h-[50px] rounded-xl shadow-lg w-[350px] border">
          {data.display_name}
        </p>
        <!-- follower/following component -->
        <div class="flex space-x-6 p-2 pt-4">
          <p>followers: {data.followers}</p>
          <p>following: {data.following}</p>
        </div>
      </div>
    </div>
    <div>
      <p class="text-neutral-500 pl-2">bio:</p>
      <div
        class="w-[750px] min-h-[75px] h-[100px] max-h-[250px]
        overflow-auto border rounded-xl border-primary
        p-2"
      >
        <p>{data.bio}</p>
      </div>
    </div>
  </main>
  <article class="overflow-y-auto max-h-[50vh]">
    <div class="flex-1 flex-col space-y-4 pt-4">
      {#each links as link}
        <Card.Root class="h-[150px] rounded-xl">
          <Card.Header>
            <div class="flex space-x-4">
              <div>
                <Avatar.Root class="w-[50px] h-[50px] ring-2">
                  <!-- TODO: use CDN hosted link instead of b64 string -->
                  <Avatar.Image
                    src={`data:image/png;base64,${link.picture}`}
                    alt=""
                  />
                  <Avatar.Fallback></Avatar.Fallback>
                </Avatar.Root>
              </div>

              <div class="flex-1">
                <a href={link.href} class="font-semibold">{link.title}</a>
                <Card.Description class="overflow-y-auto line-clamp-2"
                  >{link.description}</Card.Description
                >
              </div>
            </div>
          </Card.Header>
        </Card.Root>
      {/each}
    </div>
  </article>
</div>
