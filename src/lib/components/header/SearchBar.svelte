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
    placeholder = "Search clipboard... | 2025-06-15",
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

<div class={cn("relative flex flex-1 items-center pointer-events-none", className)}>
  <!-- Input -->
  <input
    type="text"
    bind:value
    onkeydown={handleKeyDown}
    {placeholder}
    class={`w-fit h-7  pr-10 rounded-md text-md text-text placeholder:text-text-muted placeholder:text-sm focus:outline-none pointer-events-auto`}
  />

  <!-- Result count or Clear button -->
  <div class="absolute right-3 flex items-center gap-2">
    {#if value && resultCount !== undefined && !isSearching}
      <span class="text-xs text-text-muted">
        {resultCount}
        {resultCount === 1 ? "result" : "results"}
      </span>
    {/if}

    {#if value}
      <button
        onclick={handleClear}
        class="p-1 hover:bg-surface-hover rounded transition-colors pointer-events-auto"
        title="Clear search (Esc)"
      >
        <Icon name="x" size={16} class="text-text-muted" />
      </button>
    {/if}
  </div>
</div>
