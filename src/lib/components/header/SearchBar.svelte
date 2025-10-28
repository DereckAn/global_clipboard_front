<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";
  import { cn } from "$lib/utils/cn";

  interface Props {
    value?: string;
    placeholder?: string;
    class?: string;
  }

  let {
    value = $bindable(""),
    placeholder = "Search clipboard...",
    class: className,
  }: Props = $props();

  const handleClear = () => {
    value = "";
  };
</script>

<div class={cn("relative flex items-center border-0", className)}>
  <!-- Search icon -->
  

  <!-- Input -->
  <input
    type="text"
    bind:value
    {placeholder}
    class={`w-full h-10 rounded-md text-md
  text-text placeholder:text-text-muted focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-transparent transition-colors`}
  />

  <!-- Clear button -->
  {#if value}
    <button
      onclick={handleClear}
      class="absolute right-3 p-1 hover:bg-surface-hover rounded transition-colors"
    >
      <Icon name="x" size={16} class="text-text-muted" />
    </button>
  {/if}
</div>
