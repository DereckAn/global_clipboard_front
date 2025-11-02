<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";
  import { clipboardStore } from "$lib/stores/clipboard.svelte";
  import { uiStore } from "$lib/stores/ui.svelte";
  import {
    tauriExtractDomain,
    tauriWriteToClipboard,
  } from "$lib/tauri/commands";
  import type { ClipboardItem } from "$lib/types";
  import { cn } from "$lib/utils/cn";
  import { getContentTypeIcon } from "$lib/utils/format";
  import { getLanguageInfo } from "$lib/utils/languages";

  interface Props {
    item: ClipboardItem;
  }

  let { item }: Props = $props();

  // Local state
  let isHoveringIcon = $state(false);
  let showDeleteConfirm = $state(false);
  let isDeleting = $state(false);
  let faviconUrl = $state<string>("");

  // Computed
  const isSelected = $derived(uiStore.selectedItemId === item.id);
  const displayText = $derived(item.contentText?.slice(0, 50) || "Empty");
  const iconName = $derived(getContentTypeIcon(item.contentType));
  const isColor = $derived(item.contentType === "color");
  const isLink = $derived(item.contentType === "link");
  const isCode = $derived(item.contentType === "code");
  const languageInfo = $derived(
    isCode ? getLanguageInfo(item.codeLanguage) : null
  );

  // Load favicon for links
  $effect(() => {
    if (isLink && item.contentText) {
      tauriExtractDomain(item.contentText)
        .then((domain) => {
          faviconUrl = `https://www.google.com/s2/favicons?domain=${domain}&sz=32`;
        })
        .catch(() => {
          faviconUrl = "";
        });
    } else {
      faviconUrl = "";
    }
  });

  // Handlers
  const handleClick = () => {
    uiStore.selectItem(item.id);
  };

  const handleDoubleClick = async () => {
    if (item.contentText) {
      try {
        // Delete current item first
        await clipboardStore.deleteItem(item.id);

        // Copy to clipboard (monitor will create new item automatically)
        await tauriWriteToClipboard(item.contentText);
        console.log("Copied to clipboard");
      } catch (err) {
        console.error("Failed to copy:", err);
      }
    }
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      handleClick();
    }
  };

  const handleToggleFavorite = async (e: MouseEvent) => {
    e.stopPropagation();
    await clipboardStore.toggleFavorite(item.id);
  };

  const handleFavoriteKeyDown = (e: KeyboardEvent) => {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      e.stopPropagation();
      clipboardStore.toggleFavorite(item.id);
    }
  };

  const handleDeleteClick = (e: MouseEvent) => {
    e.stopPropagation();
    showDeleteConfirm = true;
  };

  const handleConfirmDelete = async (e: MouseEvent) => {
    e.stopPropagation();
    isDeleting = true;
    try {
      await clipboardStore.deleteItem(item.id);
      if (isSelected) {
        uiStore.selectItem(null);
      }
    } catch (err) {
      console.error("Failed to delete:", err);
    } finally {
      isDeleting = false;
      showDeleteConfirm = false;
    }
  };

  const handleCancelDelete = (e: MouseEvent) => {
    e.stopPropagation();
    showDeleteConfirm = false;
  };
</script>

<div
  class={cn(
    "group relative flex items-center gap-3 px-3 py-1 text-sm cursor-pointer transition-colors mx-2",
    isSelected
      ? "bg-surface-hover rounded-xl"
      : " border-transparent hover:bg-surface-hover rounded-xl"
  )}
  onclick={handleClick}
  ondblclick={handleDoubleClick}
  onkeydown={handleKeyDown}
  role="button"
  tabindex="0"
>
  <!-- Icon with favorite toggle -->
  <div
    class="relative shrink-0 w-8 h-8 flex items-center justify-center transition-colors"
    onmouseenter={() => (isHoveringIcon = true)}
    onmouseleave={() => (isHoveringIcon = false)}
    onclick={handleToggleFavorite}
    onkeydown={handleFavoriteKeyDown}
    role="button"
    tabindex="0"
    aria-label={item.isFavorite ? "Remove from favorites" : "Add to favorites"}
  >
    {#if isHoveringIcon && !item.isFavorite}
      <!-- Show white star on hover -->
      <Icon name="star" size={20} class="text-white" />
    {:else if item.isFavorite}
      <!-- Show filled yellow star if favorite -->
      <Icon
        name="starFilled"
        size={20}
        class="text-favorite"
        fill="currentColor"
      />
    {:else if isColor && item.contentText}
      <!-- Show color circle for color type -->
      <div
        class="w-6 h-6 rounded-full border-2 border-border"
        style="background-color: {item.contentText};"
      ></div>
    {:else if isLink && faviconUrl}
      <!-- Show favicon for link type -->
      <img
        src={faviconUrl}
        alt="Site icon"
        class="w-5 h-5 rounded"
        onerror={(e) => {
          (e.currentTarget as HTMLImageElement).style.display = "none";
          (e.currentTarget as HTMLImageElement).parentElement
            ?.querySelector(".fallback-icon")
            ?.classList.remove("hidden");
        }}
      />
      <Icon name="link" size={20} class="text-text hidden fallback-icon" />
    {:else if isCode && languageInfo}
      <!-- Show language SVG icon for code -->
      <div
        class="w-7 h-7 rounded flex items-center justify-center p-1"
        style="background-color: {languageInfo.color};"
        title={languageInfo.name}
      >
        <img
          src={languageInfo.svgPath}
          alt={languageInfo.name}
          class="w-full h-full object-contain"
        />
      </div>
    {:else}
      <!-- Show type icon -->
      <Icon name={iconName} size={20} class="text-text" />
    {/if}
  </div>

  <!-- Content text -->
  <div class="flex-1 min-w-0">
    <p class="text-sm text-text truncate">
      {displayText}
    </p>
  </div>

  <!-- Delete button or confirmation -->
  {#if !showDeleteConfirm}
    <button
      onclick={handleDeleteClick}
      class="shrink-0 opacity-0 group-hover:opacity-100 transition-opacity p-1 hover:bg-danger/20 rounded"
      aria-label="Delete item"
    >
      <Icon name="trash" size={16} class="text-text-muted hover:text-danger" />
    </button>
  {:else}
    <!-- Confirmation buttons -->
    <div class="flex items-center gap-1 shrink-0">
      <button
        onclick={handleConfirmDelete}
        disabled={isDeleting}
        class="p-1 hover:bg-danger/20 rounded transition-colors disabled:opacity-50"
        aria-label="Confirm delete"
      >
        <Icon name="check" size={16} class="text-danger" />
      </button>
      <button
        onclick={handleCancelDelete}
        disabled={isDeleting}
        class="p-1 hover:bg-surface rounded transition-colors disabled:opacity-50"
        aria-label="Cancel delete"
      >
        <Icon name="x" size={16} class="text-text-muted" />
      </button>
    </div>
  {/if}
</div>
