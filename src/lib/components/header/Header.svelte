<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";
  import { clipboardStore } from "$lib/stores/clipboard.svelte";
  import type { ContentType } from "$lib/types";
  import FilterDropdown from "./FilterDropdown.svelte";
  import ProfileDropdown from "./ProfileDropdown.svelte";
  import SearchBar from "./SearchBar.svelte";

  interface Props {
    searchQuery?: string;
    filterType?: "all" | "favorites" | ContentType;
    isAuthenticated?: boolean;
    isSearching?: boolean;
    resultCount?: number;
    onSearchChange?: (query: string) => void;
    onFilterChange?: (filter: "all" | "favorites" | ContentType) => void;
  }

  let {
    searchQuery = $bindable(""),
    filterType = $bindable<"all" | "favorites" | ContentType>("all"),
    isAuthenticated = false,
    isSearching = false,
    resultCount,
    onSearchChange,
    onFilterChange,
  }: Props = $props();

  let isCleaningDuplicates = $state(false);

  // Handle search changes
  $effect(() => {
    if (onSearchChange) {
      onSearchChange(searchQuery);
    }
  });

  const handleFilterSelect = (value: "all" | "favorites" | ContentType) => {
    filterType = value;
    if (onFilterChange) {
      onFilterChange(value);
    }
  };

  const handleCleanDuplicates = async () => {
    if (isCleaningDuplicates) return;

    isCleaningDuplicates = true;
    try {
      const deleted = await clipboardStore.removeDuplicates();
      console.log(`Removed ${deleted} duplicate items`);
    } catch (err) {
      console.error("Failed to clean duplicates:", err);
    } finally {
      isCleaningDuplicates = false;
    }
  };
</script>

<header
  data-tauri-drag-region
  class="h-14 bg-surface border-b border-border flex items-center gap-4 px-4 z-50"
>
  <!-- Profile dropdown -->
  <ProfileDropdown {isAuthenticated} />

  <!-- Search bar -->
  <div class="flex-1">
    <SearchBar bind:value={searchQuery} {isSearching} {resultCount} />
  </div>

  <!-- Clean duplicates button -->
  <button
    onclick={handleCleanDuplicates}
    disabled={isCleaningDuplicates}
    class="p-2 hover:bg-surface-hover rounded transition-colors disabled:opacity-50"
    title="Remove duplicate items"
  >
    <Icon name="trash" size={18} class="text-text-muted" />
  </button>

  <!-- Filter dropdown -->
  <FilterDropdown selected={filterType} onSelect={handleFilterSelect} />
</header>
