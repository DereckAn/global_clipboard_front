<script lang="ts">
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

  
</script>

<header
  data-tauri-drag-region
  class="h-12 bg-surface border-b border-border flex items-center gap-4 px-3 z-50"
>
  <!-- Profile dropdown -->
  <ProfileDropdown {isAuthenticated} />

  <!-- Search bar -->
  <SearchBar bind:value={searchQuery} {isSearching} {resultCount} />

  <!-- Filter dropdown -->
  <FilterDropdown selected={filterType} onSelect={handleFilterSelect} />
</header>
