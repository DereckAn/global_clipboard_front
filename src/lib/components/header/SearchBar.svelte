<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";
  import { cn } from "$lib/utils/cn";

  interface Props {
    value?: string;
    placeholder?: string;
    class?: string;
    isSearching?: boolean;
    resultCount?: number;
  }

  let {
    value = $bindable(""),
    placeholder = "Search clipboard...",
    class: className,
    isSearching = false,
    resultCount,
  }: Props = $props();

  const handleClear = () => {
    value = "";
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === "Escape") {
      value = "";
    }
  };
</script>

<div class={cn("relative flex items-center border-0", className)}>
  <!-- Search icon or loading spinner -->
  <div class="absolute left-3 pointer-events-none">
    {#if isSearching}
      <div class="animate-spin">
        <Icon name="loader" size={16} class="text-text-muted" />
      </div>
    {:else}
      <Icon name="search" size={16} class="text-text-muted" />
    {/if}
  </div>

  <!-- Input -->
  <input
    type="text"
    bind:value
    onkeydown={handleKeyDown}
    {placeholder}
    class={`w-full h-10 pl-10 pr-10 rounded-md text-md bg-background
  text-text placeholder:text-text-muted focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent transition-colors`}
  />

  <!-- Result count or Clear button -->
  <div class="absolute right-3 flex items-center gap-2">
    {#if value && resultCount !== undefined && !isSearching}
      <span class="text-xs text-text-muted">
        {resultCount} {resultCount === 1 ? 'result' : 'results'}
      </span>
    {/if}

    {#if value}
      <button
        onclick={handleClear}
        class="p-1 hover:bg-surface-hover rounded transition-colors"
        title="Clear search (Esc)"
      >
        <Icon name="x" size={16} class="text-text-muted" />
      </button>
    {/if}
  </div>
</div>
