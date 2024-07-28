<script lang="ts">
  import { Lock } from "lucide-svelte";
</script>

<div class="h-auto p-20">
  <div class="flex justify-center">
    <Lock size={100} />
  </div>
  <div class="flex flex-col items-center md:gap-y-4 gap-y-2">
    <p class="font-semibold md:text-4xl text-xl">This account is private</p>
    <p class="font-light md:text-xl text-sm">
      Follow to see their bio, links & their follow counts.
    </p>
  </div>
</div>
