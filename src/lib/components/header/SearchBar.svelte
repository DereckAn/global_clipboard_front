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

<div class={cn("relative flex items-center", className)}>
  <!-- Search icon -->
  <div class="absolute left-3 pointer-events-none">
    <Icon name="search" size={18} class="text-text-muted" />
  </div>

  <!-- Input -->
  <input
    type="text"
    bind:value
    {placeholder}
    class={`w-full h-10 pl-10 pr-10 rounded-md border border-border bg-surface text-sm 
  text-text placeholder:text-text-muted focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary transition-colors`}
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
