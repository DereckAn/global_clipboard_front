<script lang="ts">
  import type { ContentType } from "$lib/types";
  import FilterDropdown from "./FilterDropdown.svelte";
  import ProfileDropdown from "./ProfileDropdown.svelte";
  import SearchBar from "./SearchBar.svelte";

  interface Props {
    searchQuery?: string;
    filterType?: "all" | "favorites" | ContentType;
    isAuthenticated?: boolean;
    onSearchChange?: (query: string) => void;
    onFilterChange?: (filter: "all" | "favorites" | ContentType) => void;
  }

  let {
    searchQuery = $bindable(""),
    filterType = $bindable<"all" | "favorites" | ContentType>("all"),
    isAuthenticated = false,
    onSearchChange,
    onFilterChange,
  }: Props = $props();

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
  class="h-14 bg-surface border-b border-border flex items-center gap-4 px-4"
>   
  <!-- Profile dropdown -->
  <ProfileDropdown {isAuthenticated} />

  <!-- Search bar -->
  <div class="flex-1">
    <SearchBar bind:value={searchQuery} />
  </div>

  <!-- Filter dropdown -->
  <FilterDropdown selected={filterType} onSelect={handleFilterSelect} />
</header>
