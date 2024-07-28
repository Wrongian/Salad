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
      class="origin-top-right right-0 absolute top-full w-[150px] bg-lime-50 shadow-lg ring-1 ring-black ring-opacity-5 divide-y-1"
    >
      <slot name="menu" />
    </div>
  {/if}
</div>
