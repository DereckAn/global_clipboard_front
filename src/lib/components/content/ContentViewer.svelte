<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import {
    tauriConvertColor,
    tauriExtractDomain,
    tauriWriteToClipboard,
  } from "$lib/tauri/commands";
  import type { ClipboardItem } from "$lib/types";
  import { getLanguageInfo } from "$lib/utils/languages";

  interface Props {
    item: ClipboardItem | null;
  }

  interface ColorFormat {
    name: string;
    value: string;
    original: boolean;
  }

  let { item }: Props = $props();

  let isCopied = $state(false);
  let copiedFormat = $state<string | null>(null);
  let colorFormats = $state<ColorFormat[]>([]);
  let rgbPreview = $state<string>("");
  let linkDomain = $state<string>("");
  let faviconUrl = $state<string>("");

  const handleCopy = async (text?: string) => {
    const textToCopy = text || item?.contentText;
    if (!textToCopy) return;

    try {
      await tauriWriteToClipboard(textToCopy);
      copiedFormat = text || "main";
      isCopied = true;
      setTimeout(() => {
        isCopied = false;
        copiedFormat = null;
      }, 2000);
    } catch (err) {
      console.error("Failed to copy:", err);
    }
  };

  // Detect if content is code
  const isCode = $derived(item?.contentType === "code");

  // Detect if content is a color
  const isColor = $derived(item?.contentType === "color");

  // Detect if content is a link
  const isLink = $derived(item?.contentType === "link");

  const languageInfo = $derived(
    item?.contentType === "code" && item?.codeLanguage
      ? getLanguageInfo(item.codeLanguage)
      : null
  );

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

  // Load link domain and favicon when item changes
  $effect(() => {
    if (isLink && item?.contentText) {
      tauriExtractDomain(item.contentText)
        .then((domain) => {
          linkDomain = domain;
          // Use Google's favicon service as fallback
          faviconUrl = `https://www.google.com/s2/favicons?domain=${domain}&sz=64`;
        })
        .catch((err) => {
          console.error("Failed to extract domain:", err);
          linkDomain = "";
          faviconUrl = "";
        });
    } else {
      linkDomain = "";
      faviconUrl = "";
    }
  });
</script>

<div
  class="flex-1 bg-background border-b border-border flex
  flex-col overflow-y-auto"
>
  {#if !item}
    <!-- Empty state -->
    <div
      class="flex-1 flex flex-col items-center justify-center px-6 text-center"
    >
      <Icon
        name="text"
        size={64}
        class="text-[var(--color-text-muted)] mb-4 opacity-50"
      />
      <h3 class="text-lg font-semibold text-[var(--color-text)] mb-2">
        No item selected
      </h3>
      <p class="text-sm text-[var(--color-text-muted)]">
        Select an item from the history to view its content
      </p>
    </div>
  {:else}
    <!-- Content header with copy button -->
    <div class="px-6 py-4 flex items-center justify-between">
      <h3
        class="text-sm font-semibold text-[var(--color-text-muted)] uppercase tracking-wide"
      >
        Content
      </h3>
      <Button
        variant="outline"
        size="sm"
        onclick={() => handleCopy()}
        disabled={isCopied && copiedFormat === "main"}
      >
        {#if isCopied && copiedFormat === "main"}
          <Icon name="check" size={16} class="mr-2" />
          Copied!
        {:else}
          <Icon name="copy" size={16} class="mr-2" />
          Copy
        {/if}
      </Button>
    </div>

    <!-- Content body with scroll (no scrollbar) -->
    <div class="flex-1 px-6 py-4 overflow-auto">
      {#if isColor && item.contentText}
        <!-- Color preview with all formats -->
        <div class="flex flex-col gap-6">
          <!-- Large color preview -->
          <div
            class="w-full h-48 rounded-lg border border-[var(--color-border)] shadow-lg"
            style="background-color: {rgbPreview || item.contentText};"
          ></div>

          <!-- Color formats list -->
          {#if colorFormats.length > 0}
            <div class="space-y-3">
              <h4
                class="text-sm font-semibold text-[var(--color-text-muted)] uppercase tracking-wide"
              >
                Color Formats
              </h4>
              {#each colorFormats as format}
                <div
                  class="flex items-center justify-between p-3 bg-[var(--color-surface)] rounded-lg border border-[var(--color-border)] hover:border-[var(--color-primary)] transition-colors group"
                >
                  <div class="flex items-center gap-3 flex-1 min-w-0">
                    <!-- Small color circle -->
                    <div
                      class="w-8 h-8 rounded-full border-2 border-[var(--color-border)] flex-shrink-0"
                      style="background-color: {rgbPreview};"
                    ></div>

                    <!-- Format info -->
                    <div class="flex-1 min-w-0">
                      <div class="flex items-center gap-2">
                        <span
                          class="text-xs font-semibold text-[var(--color-text-muted)]"
                        >
                          {format.name}
                        </span>
                        {#if format.original}
                          <span
                            class="px-2 py-0.5 text-xs bg-[var(--color-primary)]/20 text-[var(--color-primary)] rounded"
                          >
                            Original
                          </span>
                        {/if}
                      </div>
                      <code
                        class="text-sm text-[var(--color-text)] font-mono block truncate"
                      >
                        {format.value}
                      </code>
                    </div>
                  </div>

                  <!-- Copy button -->
                  <Button
                    variant="ghost"
                    size="sm"
                    onclick={() => handleCopy(format.value)}
                    class="opacity-0 group-hover:opacity-100 transition-opacity"
                  >
                    {#if isCopied && copiedFormat === format.value}
                      <Icon name="check" size={14} />
                    {:else}
                      <Icon name="copy" size={14} />
                    {/if}
                  </Button>
                </div>
              {/each}
            </div>
        {/if}
      </div>
    {:else if isCode && item.contentText}
      <!-- Code with language badge -->
        <div class="flex flex-col gap-3">
        {#if languageInfo}
            <div class="flex items-center gap-2">
                <div
                    class="px-2 py-1 rounded text-xs font-semibold text-white flex items-center gap-1"
                    style="background-color: {languageInfo.color};"
                    >
                        <span>{languageInfo.icon}</span>
                        <span>{languageInfo.name}</span>
                </div>
            </div>
        {/if}
        <pre class="text-xs text-[var(--color-text)] font-mono whitespace-pre-wrap break-words leading-relaxed bg-[var(--color-surface)] p-4 rounded-lg border border-[var(--color-border)]">{item.contentText}</pre>
        </div>
      {:else if isLink && item.contentText}
        <!-- Link with info (no iframe due to security restrictions) -->
        <div class="flex flex-col gap-4">
          <!-- Link card -->
          <div
            class="p-6 bg-[var(--color-surface)] rounded-lg border border-[var(--color-border)]"
          >
            {#if faviconUrl}
              <div class="flex items-start gap-4">
                <img
                  src={faviconUrl}
                  alt="Site icon"
                  class="w-12 h-12 rounded-lg flex-shrink-0"
                  onerror={(e) =>
                    ((e.currentTarget as HTMLImageElement).style.display =
                      "none")}
                />
                <div class="flex-1 min-w-0">
                  <p class="text-xs text-[var(--color-text-muted)] mb-1">
                    Domain
                  </p>
                  <p
                    class="text-sm font-semibold text-[var(--color-text)] mb-3"
                  >
                    {linkDomain}
                  </p>

                  <p class="text-xs text-[var(--color-text-muted)] mb-1">URL</p>
                  <a
                    href={item.contentText}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-[var(--color-primary)] hover:underline text-sm break-all block"
                  >
                    {item.contentText}
                  </a>
                </div>
              </div>
            {:else}
              <div>
                <p class="text-xs text-[var(--color-text-muted)] mb-1">URL</p>
                <a
                  href={item.contentText}
                  target="_blank"
                  rel="noopener noreferrer"
                  class="text-[var(--color-primary)] hover:underline text-sm break-all block"
                >
                  {item.contentText}
                </a>
              </div>
            {/if}
          </div>

          <!-- Open button -->
          <Button
            variant="default"
            size="default"
            onclick={() => window.open(item.contentText!, "_blank")}
            class="w-full"
          >
            <Icon name="externalLink" size={16} class="mr-2" />
            Open in browser
          </Button>

          <!-- Info box -->
          <div
            class="p-4 bg-[var(--color-surface-hover)] rounded-lg border border-[var(--color-border)]"
          >
            <p class="text-xs text-[var(--color-text-muted)]">
              <Icon name="text" size={12} class="inline mr-1" />
              For security reasons, we can't show a live preview of websites. Click
              "Open in browser" to view the page.
            </p>
          </div>
        </div>
      {:else}
        <!-- Regular text -->
        <p
          class="text-sm text-[var(--color-text)] whitespace-pre-wrap break-words leading-relaxed"
        >
          {item.contentText || "No content"}
        </p>
      {/if}
    </div>
  {/if}
</div>
