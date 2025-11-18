<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";
  import { clipboardStore } from "$lib/stores/clipboard.svelte";
  import {
    tauriBumpItem,
    tauriConvertColor,
    tauriExtractDomain,
    tauriFetchLinkMetadata,
    tauriWriteFileToClipboard,
    tauriWriteImageToClipboard,
    tauriWriteToClipboard,
    type LinkMetadata,
  } from "$lib/tauri/commands";
  import type { ClipboardItem } from "$lib/types";
  import { cn } from "$lib/utils/cn";
  import { sanitizeSvg } from "$lib/utils/svg";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { openPath } from "@tauri-apps/plugin-opener";
  import Button from "../ui/Button.svelte";
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
  const isSvg = $derived(item?.contentType === "svg");
  const isImage = $derived(item?.contentType === "image");
  const isFile = $derived(item?.contentType === "file");
  const safeSvg = $derived.by(() => {
    if (!isSvg || !item?.contentText) return "";
    return sanitizeSvg(item.contentText);
  });

  const parsedMetadata = $derived.by(() => {
    if (!item?.contentMetadata) return null;
    try {
      return typeof item.contentMetadata === "string"
        ? JSON.parse(item.contentMetadata)
        : item.contentMetadata;
    } catch (err) {
      console.error("Failed to parse metadata:", err);
      return null;
    }
  });

  const fileThumbnailPath = $derived(
    isFile && parsedMetadata?.thumbnail_path
      ? parsedMetadata.thumbnail_path
      : null
  );
  const fileThumbnailUrl = $derived.by(() => {
    if (!fileThumbnailPath) return null;
    return convertFileSrc(fileThumbnailPath);
  });
  const textPreview = $derived(parsedMetadata?.text_preview || null);
  const previewLanguage = $derived(parsedMetadata?.preview_language || null);
  const externalPath = $derived(
    parsedMetadata?.external_path || item?.fileUrl || null
  );
  const externalMissing = $derived(Boolean(parsedMetadata?.external_missing));
  const fileExists = $derived(externalPath && !externalMissing);
  const isExternalImage = $derived(
    isImage && parsedMetadata?.source === "file"
  );
  const imagePreviewPath = $derived.by(() => {
    if (!isImage) return null;
    if (isExternalImage) {
      return parsedMetadata?.thumbnail_path || null;
    }
    return item?.fileUrl || null;
  });
  const imagePreviewUrl = $derived.by(() => {
    if (!imagePreviewPath) return null;
    return convertFileSrc(imagePreviewPath);
  });

  const handleCopyFile = async () => {
    if (!item?.id || !item.fileUrl) return;

    try {
      await tauriWriteFileToClipboard(item.fileUrl);
      await tauriBumpItem(item.id);
      await clipboardStore.loadItems();
      copied = true;
      setTimeout(() => (copied = false), 2000);
    } catch (err) {
      console.error("Failed to copy file:", err);
    }
  };

  const handleOpenFile = async () => {
    if (!fileExists || !externalPath) return;
    try {
      await openPath(externalPath);
    } catch (err) {
      console.error("Failed to open file:", err);
    }
  };

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
      // Bump item to top (update timestamp)
      if (item?.id) {
        await tauriBumpItem(item.id);
      }

      // Handle based on content type
      if (isImage && item?.fileUrl) {
        // Copy image to clipboard
        await tauriWriteImageToClipboard(item.fileUrl);
      } else {
        // Copy text to clipboard
        await tauriWriteToClipboard(text);
      }

      // Reload items to reflect new order
      await clipboardStore.loadItems();

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
    <div class="flex flex-col flex-1 max-w-full">
      <!-- Metadata preview card -->
      {#if loadingMetadata}
        <div class="  bg-surface border border-border p-6">
          <div class="flex items-center justify-center py-12">
            <div class="animate-spin">
              <Icon name="loader" size={32} class="text-primary" />
            </div>
          </div>
        </div>
      {:else if linkMetadata && (linkMetadata.title || linkMetadata.image)}
        <div class="w-full">
          <!-- Preview image -->
          {#if linkMetadata.image}
            <div class="w-full h-64 overflow-hidden px-3 rounded-2xl">
              <img
                src={linkMetadata.image}
                alt={linkMetadata.title || "Link preview"}
                class="w-full h-full object-cover rounded-2xl"
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
      <div class=" my-3 flex justify-center px-2">
        <Button
          variant="default"
          onclick={() => handleCopy(item.contentText || "")}
          class="w-full"
        >
          <Icon
            name={copied ? "check" : "copy"}
            size={16}
            class={copied ? "text-primary" : "text-text"}
          />
          <span class="text-sm text-text">
            {copied ? "Copied!" : "Copy to clipboard"}
          </span>
        </Button>
      </div>
    </div>
  {:else if isImage}
    <!-- Image content -->
    <div class="flex-1 flex flex-col">
      <!-- Image Preview -->
      <div class="flex-1 flex items-center justify-center mb-4 overflow-auto">
        {#if imagePreviewUrl}
          {console.log("🖼️ Loading image preview:", {
            previewPath: imagePreviewPath,
            originalPath: item.fileUrl,
            metadata: parsedMetadata,
          })}
          <div class="max-w-full relative flex items-center justify-center">
            <img
              src={imagePreviewUrl}
              alt={item.fileName || "Clipboard image"}
              class="max-w-full object-scale-down shadow-2xl animate-in fade-in duration-300"
              decoding="async"
              draggable={false}
              onload={() =>
                console.log("✅ Image preview loaded from:", imagePreviewPath)}
              onerror={(e) =>
                console.error("❌ Failed to load image:", imagePreviewPath, e)}
            />

            <!-- Image info overlay -->
            <div
              class="absolute bottom-4 left-4 right-4 bg-surface/10 backdrop-blur rounded-lg p-3 border border-border z-20"
            >
              <div class="flex items-center justify-between gap-4">
                <div class="flex items-center gap-3">
                  <Icon name="image" size={20} class="text-primary" />
                  <div>
                    <p class="text-xs text-white">
                      {(parsedMetadata?.width as number) ||
                        "?"}x{(parsedMetadata?.height as number) || "?"} •
                      {item.fileSizeBytes
                        ? (item.fileSizeBytes / 1024).toFixed(0)
                        : "?"} KB
                    </p>
                    {#if isExternalImage}
                      <p class="text-[10px] text-white/70">
                        Preview from Quick Look cache
                      </p>
                    {/if}
                  </div>
                </div>
              </div>
            </div>
          </div>
        {:else}
          <div class="text-center">
            <Icon name="image" size={64} class="text-text-muted mx-auto mb-4" />
            <p class="text-text-muted">Image file not found</p>
          </div>
        {/if}
      </div>

      <!--  Copy button -->
      <div class="px-6 pb-6">
        <button
          onclick={() => handleCopy("")}
          class="w-full px-4 py-1 bg-gray-700 text-white rounded-lg text-sm hover:bg-gray-800 transition-colors flex items-center justify-center gap-2"
        >
          <Icon name={copied ? "check" : "copy"} size={18} class="text-white" />
          <span>{copied ? "Copied!" : "Copy image to clipboard"}</span>
        </button>
      </div>
    </div>
  {:else if isFile}
    <div class="flex-1 flex flex-col max-w-full">
      <div
        class="flex-1 flex flex-col items-center justify-center gap-4 text-center"
      >
        {#if externalMissing}
          <div
            class="w-full max-w-xl rounded-2xl border border-border/70 bg-surface-high p-4 text-left"
          >
            <p class="text-sm font-semibold text-danger mb-1">File not found</p>
            <p class="text-sm text-text-muted break-all">
              The original file was moved or deleted ({externalPath ??
                "unknown path"}).
            </p>
          </div>
        {:else if textPreview}
          <div class="w-full max-w-full py-4 text-left p-2 overflow-hidden">
            <pre
              class="font-mono text-xs max-w-full w-fit leading-relaxed text-text whitespace-pre-wrap wrap-break-words">{textPreview}</pre>
          </div>
        {:else if fileThumbnailUrl}
          <div class="w-full">
            <img
              src={fileThumbnailUrl}
              alt="File preview"
              class="max-h-[360px] w-full object-contain"
              style="filter: brightness(1.25) contrast(1.05);"
            />
          </div>
        {:else}
          <div class="w-full max-w-md p-6 flex items-center justify-center">
            <Icon name="file" size={70} class="text-primary" />
          </div>
        {/if}
        <div class="flex flex-row w-full gap-2 p-2">
          <Button
            onclick={() => handleOpenFile()}
            disabled={!fileExists}
            class=""
          >
            Open file
          </Button>
          <Button
            variant="outline"
            onclick={() => handleCopyFile()}
            disabled={!fileExists}
          >
            <Icon name={"copy"} size={18} class="text-white mr-2 flex-1" />
            Copy file to clipboard
          </Button>
        </div>
      </div>
    </div>
  {:else if isSvg}
    <!-- SVG content -->
    <div class="flex-1 flex flex-col">
      <!-- SVG Preview -->
      <div class="flex-1 flex items-center justify-center p-8 overflow-auto">
        <div
          class="max-w-2xl max-h-full bg-surface rounded-xl border border-border p-8 flex items-center justify-center"
        >
          {@html safeSvg}
        </div>
      </div>

      <!-- Copy and View Source buttons -->
      <div class="px-6 pb-6 flex gap-3">
        <button
          onclick={() => handleCopy(item.contentText || "")}
          class="flex-1 px-4 py-1 bg-gray-700 text-white rounded-lg text-sm hover:bg-gray-800 transition-colors flex items-center justify-center gap-2"
        >
          <Icon name={copied ? "check" : "copy"} size={18} class="text-white" />
          <span>{copied ? "Copied!" : "Copy SVG"}</span>
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
            "text-xs text-text font-mono whitespace-pre-wrap wrap-break-words p-4 text-wrap long-content-guard"
          )}>{#if searchQuery.trim()}
            <HighlightedText text={item.contentText!} query={searchQuery} />
          {:else}{item.contentText}
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

  .animate-in {
    animation-fill-mode: both;
  }

  .fade-in {
    animation-name: fadeIn;
  }

  .duration-300 {
    animation-duration: 300ms;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
</style>
