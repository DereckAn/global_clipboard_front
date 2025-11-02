<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";
  import { clipboardStore } from "$lib/stores/clipboard.svelte";
  import {
    tauriConvertColor,
    tauriExtractDomain,
    tauriFetchLinkMetadata,
    tauriWriteToClipboard,
    type LinkMetadata,
  } from "$lib/tauri/commands";
  import type { ClipboardItem } from "$lib/types";
  import { cn } from "$lib/utils/cn";
  import HighlightedText from "../ui/HighlightedText.svelte";

  interface Props {
    item: ClipboardItem | null;
    class?: string;
    searchQuery?: string; // AGREGAR
  }

  let { item, class: className, searchQuery = "" }: Props = $props();
  // Local state
  let copied = $state(false);
  let colorFormats = $state<
    Array<{ name: string; value: string; original: boolean }>
  >([]);
  let rgbPreview = $state<string>("");
  let domain = $state<string>("");
  let linkMetadata = $state<LinkMetadata | null>(null);
  let loadingMetadata = $state(false);

  // Computed
  const isColor = $derived(item?.contentType === "color");
  const isLink = $derived(item?.contentType === "link");
  const isCode = $derived(item?.contentType === "code");

  // Load color formats when item changes
  $effect(() => {
    if (isColor && item?.contentText) {
      tauriConvertColor(item.contentText)
        .then((result) => {
          colorFormats = result.formats;
          rgbPreview = result.rgb_preview;
        })
        .catch((err) => {
          console.error("Failed to convert color:", err);
          colorFormats = [];
          rgbPreview = item.contentText || "";
        });
    } else {
      colorFormats = [];
      rgbPreview = "";
    }
  });

  // Load link metadata when item changes
  $effect(() => {
    if (isLink && item?.contentText) {
      loadingMetadata = true;
      linkMetadata = null;

      tauriExtractDomain(item.contentText)
        .then((d) => {
          domain = d;
        })
        .catch(() => {
          domain = "";
        });

      tauriFetchLinkMetadata(item.contentText)
        .then((metadata) => {
          linkMetadata = metadata;
        })
        .catch((err) => {
          console.error("Failed to fetch metadata:", err);
          linkMetadata = null;
        })
        .finally(() => {
          loadingMetadata = false;
        });
    } else {
      domain = "";
      linkMetadata = null;
      loadingMetadata = false;
    }
  });

  // Copy handlers
  const handleCopy = async (text: string) => {
    try {
      // Delete current item first
      if (item?.id) {
        await clipboardStore.deleteItem(item.id);
      }

      // Copy to clipboard (monitor will create new item automatically)
      await tauriWriteToClipboard(text);
      copied = true;
      setTimeout(() => {
        copied = false;
      }, 2000);
    } catch (err) {
      console.error("Failed to copy:", err);
    }
  };

  const handleOpenLink = () => {
    if (item?.contentText) {
      window.open(item.contentText, "_blank");
    }
  };
</script>

<div class={cn("flex-1 overflow-y-auto flex flex-col", className)}>
  {#if !item}
    <!-- Empty state -->
    <div class="flex-1 flex items-center justify-center px-6 text-center">
      <div>
        <Icon name="text" size={64} class="text-text-muted mx-auto mb-4" />
        <p class="text-lg text-text">No item selected</p>
        <p class="text-sm text-text-muted mt-2">
          Select an item from the sidebar to view its content
        </p>
      </div>
    </div>
  {:else if isColor && colorFormats.length > 0}
    <!-- Color content -->
    <div class="flex-1 flex flex-col items-center justify-center px-6 py-8">
      <!-- Large color preview -->
      <div
        class="w-48 h-48 rounded-2xl border-4 border-border shadow-lg mb-8"
        style="background-color: {rgbPreview || item.contentText};"
      ></div>

      <!-- Color formats -->
      <div class="w-full max-w-md space-y-3">
        <h4
          class="text-sm font-semibold text-text-muted uppercase tracking-wide mb-4"
        >
          Color Formats
        </h4>
        {#each colorFormats as format}
          <div
            class="flex items-center justify-between gap-4 p-3 bg-surface rounded-lg border border-border hover:border-primary transition-colors group"
          >
            <div class="flex items-center gap-3 flex-1 min-w-0">
              <!-- Small color preview -->
              <div
                class="w-8 h-8 rounded-full border-2 border-border shrink-0"
                style="background-color: {rgbPreview};"
              ></div>

              <!-- Format info -->
              <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2 mb-1">
                  <span class="text-xs font-semibold text-text-muted uppercase">
                    {format.name}
                  </span>
                  {#if format.original}
                    <span
                      class="px-2 py-0.5 text-xs bg-primary/20 text-primary rounded"
                    >
                      Original
                    </span>
                  {/if}
                </div>
                <code class="text-sm text-text font-mono block truncate">
                  {format.value}
                </code>
              </div>
            </div>

            <!-- Copy button -->
            <button
              onclick={() => handleCopy(format.value)}
              class="p-2 hover:bg-surface-hover rounded transition-colors opacity-0 group-hover:opacity-100"
              aria-label="Copy {format.name}"
            >
              <Icon
                name={copied ? "check" : "copy"}
                size={16}
                class={copied ? "text-primary" : "text-text-muted"}
              />
            </button>
          </div>
        {/each}
      </div>
    </div>
  {:else if isLink}
    <!-- Link content with metadata preview -->
    <div class="flex flex-col px-6 py-8 flex-1">
      <!-- Link URL -->
      <div class="flex items-center gap-3 w-fit mb-3">
        <div
          class="size-10 bg-surface rounded-lg flex items-center justify-center"
        >
          <Icon name="link" size={24} class="text-primary" />
        </div>
        <div class="">
          <p class="text-sm text-text-muted">Link</p>
          <p class="text-base text-text truncate font-medium">
            {domain || item.contentText}
          </p>
        </div>
        <button
          onclick={handleOpenLink}
          class="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors flex items-center gap-2"
        >
          <Icon name="externalLink" size={16} />
          <span>Open</span>
        </button>
      </div>

      <!-- Metadata preview card -->
      {#if loadingMetadata}
        <div class="  bg-surface rounded-xl border border-border p-6">
          <div class="flex items-center justify-center py-12">
            <div class="animate-spin">
              <Icon name="loader" size={32} class="text-primary" />
            </div>
          </div>
        </div>
      {:else if linkMetadata && (linkMetadata.title || linkMetadata.image)}
        <div
          class="w-2/3 bg-surface rounded-xl border border-border overflow-hidden hover:border-primary/50 transition-colors"
        >
          <!-- Preview image -->
          {#if linkMetadata.image}
            <div class="w-full h-64 bg-surface-hover overflow-hidden">
              <img
                src={linkMetadata.image}
                alt={linkMetadata.title || "Link preview"}
                class="w-full h-full object-cover"
                onerror={(e) => {
                  (
                    e.currentTarget as HTMLImageElement
                  ).parentElement!.style.display = "none";
                }}
              />
            </div>
          {/if}

          <!-- Metadata content -->
          <div class="p-6">
            {#if linkMetadata.site_name}
              <p class="text-xs text-text-muted uppercase tracking-wider mb-2">
                {linkMetadata.site_name}
              </p>
            {/if}

            {#if linkMetadata.title}
              <h3 class="text-xl font-semibold text-text mb-3">
                {linkMetadata.title}
              </h3>
            {/if}

            {#if linkMetadata.description}
              <p class="text-sm text-text-muted line-clamp-3">
                {linkMetadata.description}
              </p>
            {/if}

            <div class="mt-4 pt-4 border-t border-border">
              <p class="text-xs text-text-muted truncate">
                {item.contentText}
              </p>
            </div>
          </div>
        </div>
      {:else}
        <!-- Fallback: simple link display -->
        <div
          class="w-fit max-w-2xl mx-auto bg-surface rounded-xl border border-border p-6"
        >
          <div class="flex items-center gap-3">
            <div
              class="w-16 h-16 rounded-lg overflow-hidden bg-surface-hover flex items-center justify-center"
            >
              <img
                src="https://www.google.com/s2/favicons?domain={domain}&sz=64"
                alt="Favicon"
                class="w-10 h-10"
                onerror={(e) => {
                  (e.currentTarget as HTMLImageElement).style.display = "none";
                }}
              />
            </div>
            <div class="flex-1">
              <p class="text-sm text-text-muted mb-1">Website</p>
              <p class="text-base text-text font-medium truncate">
                {domain}
              </p>
              <p class="text-xs text-text-muted truncate mt-1">
                {item.contentText}
              </p>
            </div>
          </div>
        </div>
      {/if}

      <!-- Copy button -->
      <div class="w-fit mt-6 flex justify-start">
        <button
          onclick={() => handleCopy(item.contentText || "")}
          class="px-6 py-2 bg-surface hover:bg-surface-hover rounded-lg transition-colors flex items-center gap-2 border border-border"
        >
          <Icon
            name={copied ? "check" : "copy"}
            size={16}
            class={copied ? "text-primary" : "text-text"}
          />
          <span class="text-sm text-text">
            {copied ? "Copied!" : "Copy URL"}
          </span>
        </button>
      </div>
    </div>
  {:else}
    <!-- Text/Code content -->
    <div class="flex-1 flex flex-col">
      <!-- Content area -->
      <div class="flex-1">
        <pre
          class={cn(
            "text-xs text-text font-mono whitespace-pre-wrap wrap-break-words p-4 "
          )}>{#if searchQuery.trim()}
            <HighlightedText text={item.contentText!} query={searchQuery} />
          {:else}
            {item.contentText}
          {/if}</pre>
      </div>

      <!-- Copy button -->
      <div class="px-6 pb-6">
        <button
          onclick={() => handleCopy(item.contentText || "")}
          class="w-full px-4 py-1 bg-gray-700 text-white rounded-lg text-sm hover:bg-gray-800 transition-colors flex items-center justify-center gap-2"
        >
          <Icon name={copied ? "check" : "copy"} size={18} class="text-white" />
          <span>{copied ? "Copied!" : "Copy to clipboard"}</span>
        </button>
      </div>
    </div>
  {/if}
</div>

<style>
  .line-clamp-3 {
    display: -webkit-box;
    -webkit-line-clamp: 3;
    line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .animate-spin {
    animation: spin 1s linear infinite;
  }
</style>
