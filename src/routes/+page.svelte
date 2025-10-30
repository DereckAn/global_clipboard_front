<script lang="ts">
  import ContentViewer from "$lib/components/content/ContentViewer.svelte";
  import ItemInfo from "$lib/components/content/ItemInfo.svelte";
  import Header from "$lib/components/header/Header.svelte";
  import Sidebar from "$lib/components/sidebar/Sidebar.svelte";
  import { clipboardStore } from "$lib/stores/clipboard.svelte";
  import { uiStore } from "$lib/stores/ui.svelte";
  import type { ClipboardItem, ContentType } from "$lib/types";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";

  // Local state
  let searchQuery = $state("");
  let filterType = $state<"all" | "favorites" | ContentType>("all");
  let debouncedSearchQuery = $state("");
  let searchTimeoutId: number | null = null;
  let unlistenClipboard: UnlistenFn | null = null;

  // Debounced search
  $effect(() => {
    if (searchTimeoutId !== null) {
      clearTimeout(searchTimeoutId);
    }

    searchTimeoutId = window.setTimeout(() => {
      debouncedSearchQuery = searchQuery;
    }, 300);

    return () => {
      if (searchTimeoutId !== null) {
        clearTimeout(searchTimeoutId);
      }
    };
  });

  // Filtered items based on search and filter type
  const filteredItems = $derived.by(() => {
    let items = clipboardStore.items;

    // Apply filter type
    if (filterType === "favorites") {
      items = items.filter((item) => item.isFavorite);
    } else if (filterType !== "all") {
      items = items.filter((item) => item.contentType === filterType);
    }

    // Apply search query - ACTUALIZADO para usar la búsqueda paginada
    if (debouncedSearchQuery.trim()) {
      const query = debouncedSearchQuery.toLowerCase();
      // Usar la búsqueda en items ya cargados (rápido)
      items = items.filter((item) =>
        item.contentText?.toLowerCase().includes(query)
      );
    }

    return items;
  });

  // Selected item
  const selectedItem = $derived(
    clipboardStore.items.find((item) => item.id === uiStore.selectedItemId) ||
      null
  );

  // Helper to deserialize clipboard item from event
  function deserializeClipboardItem(item: any): ClipboardItem {
    return {
      ...item,
      createdAt: new Date(item.created_at),
      updatedAt: new Date(item.updated_at),
      contentType: item.content_type,
      contentText: item.content_text,
      contentMetadata: item.content_metadata
        ? JSON.parse(item.content_metadata)
        : {},
      sourceApp: item.source_app,
      fileUrl: item.file_url,
      fileName: item.file_name,
      fileSizeBytes: item.file_size_bytes,
      fileMimeType: item.file_mime_type,
      isFavorite: item.is_favorite,
      isSnippet: item.is_snippet,
      snippetName: item.snippet_name,
      synced: item.synced,
      serverId: item.server_id,
    };
  }

  // Load items on mount
  onMount(async () => {
    // Load initial clipboard items
    await clipboardStore.loadItems();

    // Listening for new clipboard items
    unlistenClipboard = await listen<any>("clipboard-item-added", (event) => {
      console.log("New clipboard item received:", event.payload);
      const newItem = deserializeClipboardItem(event.payload);

      // Add to store (prepend to beginning)
      clipboardStore.items = [newItem, ...clipboardStore.items];
    });
  });

  // Cleanup on destroy
  onDestroy(() => {
    if (unlistenClipboard) {
      unlistenClipboard();
    }
  });
</script>

<div class="h-screen w-screen overflow-hidden flex flex-col bg-background">
  <!-- Header -->
  <Header bind:searchQuery bind:filterType isAuthenticated={false} />

  <!-- Main content area -->
  <div class="flex-1 flex overflow-hidden">
    <!-- Sidebar (30%) -->
    <Sidebar items={filteredItems} isLoading={clipboardStore.isLoading} />

    <!-- Right panel (70%) -->
    <div class="flex-1 flex flex-col">
      <!-- Content viewer (top) -->
      <ContentViewer item={selectedItem} />

      <!-- Item info (bottom) -->
      <ItemInfo item={selectedItem} />
    </div>
  </div>
</div>
