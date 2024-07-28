<script lang="ts">
  import { EllipsisVertical } from "lucide-svelte";

  let isDropDownOpen = false;

  const handleDropdownFocusLoss = ({
    relatedTarget,
    currentTarget,
  }: FocusEvent & { currentTarget: HTMLDivElement }) => {
    if (
      relatedTarget instanceof HTMLElement &&
      currentTarget.contains(relatedTarget)
    ) {
      return;
    }
    isDropDownOpen = false;
  };
</script>

<div class="relative" on:focusout={handleDropdownFocusLoss}>
  <button class="m-0 p-0" on:click={() => (isDropDownOpen = true)}>
    <EllipsisVertical />
  </button>

  {#if isDropDownOpen}
    <div
      class="-translate-x-full -translate-y-full absolute top-full w-auto divide-y-1"
    >
      <slot name="menu" />
    </div>
  {/if}
</div>
