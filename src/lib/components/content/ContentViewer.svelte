<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import { tauriWriteToClipboard } from "$lib/tauri/commands";
  import type { ClipboardItem } from "$lib/types";

  interface Props {
    item: ClipboardItem | null;
  }

  let { item }: Props = $props();

  let isCopied = $state(false);

  const handleCopy = async () => {
    if (!item?.contentText) return;

    try {
      await tauriWriteToClipboard(item.contentText);
      isCopied = true;
      setTimeout(() => {
        isCopied = false;
      }, 2000);
    } catch (err) {
      console.error("Failed to copy:", err);
    }
  };

  // Detect if content is code (simple heuristic)
  const isCode = $derived(item?.contentType === "code");

  // Detect if content is a color
  const isColor = $derived(item?.contentType === "color");
</script>

<div class="flex-1 bg-background border-b border-border flex flex-col">
  {#if !item}
    <!-- Empty state -->
    <div
      class="flex-1 flex flex-col items-center justify-center px-6 text-center"
    >
      <Icon name="text" size={64} class="text-text-muted mb-4 opacity-50" />
      <h3 class="text-lg font-semibold text-text mb-2">No item selected</h3>
      <p class="text-sm text-text-muted">
        Select an item from the history to view its content
      </p>
    </div>
  {:else}
    <!-- Content header with copy button -->
    <div
      class="px-6 py-4 border-b border-border flex items-center justify-between"
    >
      <h3 class="text-sm font-semibold text-text uppercase tracking-wide">
        Content
      </h3>
      <Button
        variant="outline"
        size="sm"
        onclick={handleCopy}
        disabled={isCopied}
      >
        {#if isCopied}
          <Icon name="check" size={16} class="mr-2" />
          Copied!
        {:else}
          <Icon name="copy" size={16} class="mr-2" />
          Copy
        {/if}
      </Button>
    </div>

    <!-- Content body with scroll (no scrollbar) -->
    <div class="flex-1 overflow-y-auto px-6 py-4">
      {#if isColor && item.contentText}
        <!-- Color preview -->
        <div class="flex flex-col gap-4">
          <div
            class="w-full h-32 rounded-lg border border-border"
            style="background-color: {item.contentText};"
          ></div>
          <pre
            class="text-sm text-text font-mono bg-surface p-4 rounded-lg border border-border">{item.contentText}</pre>
        </div>
      {:else if isCode && item.contentText}
        <!-- Code with simple syntax highlighting (monospace) -->
        <pre
          class="text-sm text-text font-mono whitespace-pre-wrap wrap-break-words leading-relaxed bg-surface p-4 rounded-lg border border-border">{item.contentText}</pre>
      {:else if item.contentType === "link" && item.contentText}
        <!-- Link with clickable preview -->
        <div class="flex flex-col gap-3">
          <a
            href={item.contentText}
            target="_blank"
            rel="noopener noreferrer"
            class="text-primary hover:underline text-sm break-all"
          >
            {item.contentText}
          </a>
          <Button
            variant="outline"
            size="sm"
            onclick={() => window.open(item.contentText!, "_blank")}
          >
            <Icon name="externalLink" size={16} class="mr-2" />
            Open in browser
          </Button>
        </div>
      {:else}
        <!-- Regular text -->
        <p
          class="text-sm text-text whitespace-pre-wrap wrap-break-words leading-relaxed"
        >
          {item.contentText || "No content"}
        </p>
      {/if}
    </div>
  {/if}
</div>
