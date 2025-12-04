<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";
  import { settingsStore } from "$lib/stores/settings.svelte";
  import {
    tauriCaptureFullScreenshot,
    tauriCaptureRegionScreenshot,
  } from "$lib/tauri/commands";
  import type { ContentType } from "$lib/types";
  import { onMount } from "svelte";
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
  let isCapturing = $state(false);
  let screenshotMenuOpen = $state(false);

  const screenshotFeature = $derived.by(() =>
    settingsStore.labFeatures.find((f) => f.id === "screenshot")
  );

  const showScreenshotButton = $derived.by(
    () =>
      !!(
        screenshotFeature &&
        screenshotFeature.status === "installed" &&
        screenshotFeature.enabled
      )
  );

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

  onMount(() => {
    ensureLabState();
  });

  const ensureLabState = async () => {
    if (!settingsStore.labFeatures.length) {
      try {
        await settingsStore.refreshLabFeatures();
      } catch (err) {
        console.error("Failed to refresh lab features:", err);
      }
    }
  };

  const runCapture = async (mode: "full" | "region") => {
    if (isCapturing) return;
    isCapturing = true;
    screenshotMenuOpen = false;
    await ensureLabState();
    try {
      if (mode === "full") {
        await tauriCaptureFullScreenshot();
      } else {
        await tauriCaptureRegionScreenshot();
      }
    } catch (err) {
      console.error("Capture failed:", err);
    } finally {
      isCapturing = false;
    }
  };
</script>

<header
  data-tauri-drag-region
  class="h-12  border-border flex items-center gap-4 px-3 z-50"
>
  <!-- Profile dropdown -->
  <ProfileDropdown {isAuthenticated} />

  <!-- Search bar -->
  <SearchBar bind:value={searchQuery} {isSearching} {resultCount} />

  {#if showScreenshotButton}
    <div class="relative">
      <div class="flex items-center gap-1 bg-surface-200/30 border border-border/70 rounded-xl overflow-hidden">
        <button
          class="h-8 px-3 text-sm flex items-center gap-2 hover:bg-surface-100/60 transition-colors"
          onclick={() => runCapture("full")}
          disabled={isCapturing}
        >
          <Icon name="image" size={16} />
          {isCapturing ? "Capturing..." : "Screenshot"}
        </button>
        <button
          class="h-8 px-2 border-l border-border/60 hover:bg-surface-100/60"
          onclick={() => (screenshotMenuOpen = !screenshotMenuOpen)}
          aria-label="Screenshot options"
          disabled={isCapturing}
        >
          <Icon name="chevronDown" size={14} />
        </button>
      </div>
      {#if screenshotMenuOpen}
        <div class="absolute right-0 mt-2 w-56 bg-surface border border-border/70 rounded-lg shadow-lg z-50">
          <button
            class="w-full text-left px-3 py-2 text-sm hover:bg-surface-100/60 flex items-center gap-2"
            onclick={() => runCapture("full")}
          >
            <Icon name="image" size={16} />
            Full screen (⌘+⇧+3 / PrtSc)
          </button>
          <button
            class="w-full text-left px-3 py-2 text-sm hover:bg-surface-100/60 flex items-center gap-2"
            onclick={() => runCapture("region")}
          >
            <Icon name="color" size={16} />
            Selection (⌘+⇧+4 / Snip)
          </button>
        </div>
      {/if}
    </div>
  {/if}

  <!-- Filter dropdown -->
  <FilterDropdown selected={filterType} onSelect={handleFilterSelect} />
</header>
