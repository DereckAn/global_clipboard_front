<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";
  import type { ClipboardItem } from "$lib/types";
  import {
    detectColorFormat,
    formatDate,
    formatFileSize,
    getCharacterCount,
    getContentTypeIcon,
    getWordCount,
  } from "$lib/utils/format";

  interface Props {
    item: ClipboardItem | null;
  }

  let { item }: Props = $props();

  const characterCount = $derived(getCharacterCount(item?.contentText ?? null));
  const wordCount = $derived(getWordCount(item?.contentText ?? null));
  const colorFormat = $derived(
    item?.contentType === "color" && item.contentText
      ? detectColorFormat(item.contentText)
      : null
  );
</script>

<div class="h-64 bg-surface flex flex-col">
  {#if !item}
    <!-- Empty state -->
    <div class="flex-1 flex items-center justify-center px-6 text-center">
      <p class="text-sm text-text-muted">No information to display</p>
    </div>
  {:else}
    <!-- Info header -->
    <div class="px-6 py-3 border-b border-border">
      <h3 class="text-sm font-semibold text-text uppercase tracking-wide">
        Information
      </h3>
    </div>

    <!-- Info grid -->
    <div class="flex-1 overflow-y-auto px-6 py-4">
      <dl class="grid grid-cols-2 gap-x-4 gap-y-4 text-sm">
        <!-- Content Type -->
        <div class="col-span-2 flex items-center gap-2">
          <dt class="text-text-muted">Content type</dt>
          <dd class="flex items-center gap-2 text-text font-medium">
            <Icon name={getContentTypeIcon(item.contentType)} size={16} />
            <span class="capitalize">{item.contentType}</span>
          </dd>
        </div>

        <!-- Created At -->
        <div class="flex items-start gap-2">
          <dt class="text-text-muted flex items-center gap-1">
            <Icon name="clock" size={14} />
            <span>Created</span>
          </dt>
          <dd class="text-text">{formatDate(item.createdAt)}</dd>
        </div>

        <!-- Characters -->
        <div class="flex items-start gap-2">
          <dt class="text-text-muted">Characters</dt>
          <dd class="text-text">{characterCount.toLocaleString()}</dd>
        </div>

        <!-- Words (only for text/code) -->
        {#if item.contentType === "text" || item.contentType === "code"}
          <div class="flex items-start gap-2">
            <dt class="text-text-muted">Words</dt>
            <dd class="text-text">{wordCount.toLocaleString()}</dd>
          </div>
        {/if}

        <!-- Source App -->
        {#if item.sourceApp}
          <div class="flex items-start gap-2">
            <dt class="text-text-muted">Source</dt>
            <dd class="text-text">{item.sourceApp}</dd>
          </div>
        {/if}

        <!-- File size (if available) -->
        {#if item.fileSizeBytes !== null}
          <div class="flex items-start gap-2">
            <dt class="text-text-muted">Size</dt>
            <dd class="text-text">{formatFileSize(item.fileSizeBytes)}</dd>
          </div>
        {/if}

        <!-- File name (if available) -->
        {#if item.fileName}
          <div class="col-span-2 flex items-start gap-2">
            <dt class="text-text-muted">File name</dt>
            <dd class="text-text break-all">{item.fileName}</dd>
          </div>
        {/if}

        <!-- Color format (if color) -->
        {#if colorFormat}
          <div class="flex items-start gap-2">
            <dt class="text-text-muted">Format</dt>
            <dd class="text-text">{colorFormat}</dd>
          </div>
        {/if}

        <!-- Favorite status -->
        <div class="flex items-start gap-2">
          <dt class="text-text-muted">Favorite</dt>
          <dd class="text-text flex items-center gap-1">
            {#if item.isFavorite}
              <Icon
                name="starFilled"
                size={14}
                class="text-favorite"
                fill="currentColor"
                strokeWidth={0}
              />
              <span>Yes</span>
            {:else}
              <span>No</span>
            {/if}
          </dd>
        </div>
      </dl>
    </div>
  {/if}
</div>
